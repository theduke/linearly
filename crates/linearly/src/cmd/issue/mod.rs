use comfy_table::{Attribute, Cell};

use crate::render::Render;

use super::CliCommand;

pub mod list;

#[derive(clap::Subcommand)]
pub enum CmdIssue {
    #[clap(alias = "ls")]
    List(list::CmdIssueList),
}

impl CliCommand for CmdIssue {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        match self {
            CmdIssue::List(c) => c.run().await,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IssueTableColumns {
    Key,
    Title,
    #[allow(dead_code)]
    Description,
    State,
    Created,
    Updated,
    AssigneeDisplayName,
}

impl Render for linear_api::schema::issue_list::Issue {
    type TableColumns = IssueTableColumns;

    fn default_list_table_columns() -> Vec<Self::TableColumns> {
        vec![
            IssueTableColumns::Key,
            IssueTableColumns::Title,
            IssueTableColumns::State,
            IssueTableColumns::AssigneeDisplayName,
            IssueTableColumns::Created,
            IssueTableColumns::Updated,
        ]
    }

    fn list_table_header(columns: &[Self::TableColumns]) -> Vec<Cell> {
        columns
            .iter()
            .map(|c| match c {
                IssueTableColumns::Key => Cell::new("Key").add_attribute(Attribute::Bold),
                IssueTableColumns::Title => Cell::new("Title"),
                IssueTableColumns::Description => Cell::new("Description"),
                IssueTableColumns::State => Cell::new("State"),
                IssueTableColumns::AssigneeDisplayName => Cell::new("Assignee"),
                IssueTableColumns::Created => Cell::new("Created"),
                IssueTableColumns::Updated => Cell::new("Updated"),
            })
            .collect()
    }

    fn list_table_row(item: &Self, columns: &[Self::TableColumns]) -> Vec<Cell> {
        let timeformat = time::format_description::parse("[year]-[month]-[day]").unwrap();

        columns
            .iter()
            .map(|col| match col {
                IssueTableColumns::Key => Cell::new(&item.identifier),
                IssueTableColumns::Title => Cell::new(&item.title),
                IssueTableColumns::Description => {
                    Cell::new(item.description.as_deref().unwrap_or_default())
                }
                IssueTableColumns::State => Cell::new(&item.state.name),
                IssueTableColumns::AssigneeDisplayName => Cell::new(
                    item.assignee
                        .as_ref()
                        .map(|x| x.display_name.as_str())
                        .unwrap_or_default(),
                ),
                IssueTableColumns::Created => {
                    let v = item
                        .created_at
                        .parse()
                        .unwrap()
                        .format(&timeformat)
                        .unwrap();
                    Cell::new(v)
                }

                IssueTableColumns::Updated => {
                    let v = item
                        .updated_at
                        .parse()
                        .unwrap()
                        .format(&timeformat)
                        .unwrap();
                    Cell::new(v)
                }
            })
            .collect()
    }
}
