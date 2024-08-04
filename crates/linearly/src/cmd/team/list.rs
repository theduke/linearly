use anyhow::Context;
use cynic::QueryBuilder;

use linear_api::schema::{
    team_list::{Team, TeamList, TeamListVariables},
    PaginationOrderBy,
};

use crate::{
    cmd::{CliCommand, CommonArgs, Format},
    render::Render,
};

#[derive(clap::Parser)]
pub struct CmdTeamList {
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

impl CliCommand for CmdTeamList {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        let vars = TeamListVariables {
            first: self.limit,
            order: PaginationOrderBy::CreatedAt,
        };
        let op = TeamList::build(vars);

        let res = self
            .common
            .client()?
            .run(op)
            .await?
            .data
            .context("no data")?;

        let output = match self.format {
            Format::Table => {
                let columns = Team::default_list_fields();
                Team::render_list_table(&res.teams.nodes, &columns)
            }
            Format::Json => serde_json::to_string_pretty(&res.teams.nodes)?,
        };

        println!("{}", output);

        Ok(())
    }
}
