use anyhow::Context;
use cynic::QueryBuilder;

use linear_api::schema::{
    issue_list::{Issue, IssueFilter, IssueList, IssueListVariables},
    PaginationOrderBy,
};

use crate::{
    cmd::{CliCommand, CommonArgs, Format},
    render::Render,
};

#[derive(clap::Parser)]
pub struct CmdIssueList {
    #[clap(flatten)]
    common: CommonArgs,

    #[clap(long, default_value = "20")]
    pub limit: i32,

    /// Name of the assigned user.
    #[clap(long)]
    pub assignee: Option<String>,

    /// Name of the creator.
    #[clap(long)]
    pub author: Option<String>,

    /// Filter by team.
    /// Expects the short team KEY.
    #[clap(long)]
    pub team: Option<String>,

    #[clap(long, short, default_value = "table")]
    pub format: Format,
}

impl CliCommand for CmdIssueList {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        let filter = {
            let mut filters = Vec::<IssueFilter>::new();

            if let Some(name) = self.assignee {
                filters.push(IssueFilter::assignee_username(name));
            }
            if let Some(name) = self.author {
                filters.push(IssueFilter::creator_username(name));
            }
            if let Some(team) = self.team {
                filters.push(IssueFilter::team_key(team));
            }

            match filters.len() {
                0 => None,
                1 => Some(filters.pop().unwrap()),
                _ => Some(IssueFilter::new_and(filters)),
            }
        };

        let vars = IssueListVariables {
            first: self.limit,
            order: PaginationOrderBy::UpdatedAt,
            sort: None,
            filter,
        };
        let op = IssueList::build(vars);

        let res = self
            .common
            .client()?
            .run(op)
            .await?
            .data
            .context("no data")?;

        let output = match self.format {
            Format::Table => {
                let columns = Issue::default_list_table_columns();
                Issue::render_list_table(&res.issues.nodes, &columns)
            }
            Format::Json => serde_json::to_string_pretty(&res.issues.nodes)?,
        };

        println!("{}", output);

        Ok(())
    }
}
