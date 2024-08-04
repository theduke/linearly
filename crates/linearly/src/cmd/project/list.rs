use anyhow::Context;
use cynic::QueryBuilder;

use linear_api::schema::{
    projects_list::{Project, ProjectList, ProjectListVariables},
    PaginationOrderBy,
};

use crate::{
    cmd::{CliCommand, CommonArgs, Format},
    render::Render,
};

#[derive(clap::Parser)]
pub struct CmdProjectList {
    #[clap(flatten)]
    common: CommonArgs,

    #[clap(long, default_value = "50")]
    pub limit: i32,

    /// Include archived projects.
    #[clap(long, default_value = "false")]
    pub archived: bool,

    #[clap(long, short, default_value = "table")]
    pub format: Format,
}

impl CliCommand for CmdProjectList {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        let vars = ProjectListVariables {
            first: self.limit,
            include_archived: self.archived,
            oder: PaginationOrderBy::CreatedAt,
        };
        let op = ProjectList::build(vars);

        let res = self
            .common
            .client()?
            .run(op)
            .await?
            .data
            .context("no data")?;

        let output = match self.format {
            Format::Table => {
                let columns = Project::default_list_table_fields();
                Project::render_list_table(&res.projects.nodes, &columns)
            }
            Format::Json => serde_json::to_string_pretty(&res.projects.nodes)?,
        };

        println!("{}", output);

        Ok(())
    }
}
