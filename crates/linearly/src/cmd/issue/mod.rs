pub mod checkout;
pub mod list;
pub mod view;

use comfy_table::{Attribute, Cell};

use crate::render::Render;

use super::CliCommand;

#[derive(clap::Subcommand)]
pub enum CmdIssue {
    #[clap(alias = "ls")]
    List(list::CmdIssueList),
    View(view::CmdIssueView),
    #[clap(alias = "co")]
    Checkout(checkout::CmdIssueCheckout),
}

impl CliCommand for CmdIssue {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        match self {
            CmdIssue::List(c) => c.run().await,
            CmdIssue::View(c) => c.run().await,
            CmdIssue::Checkout(c) => c.run().await,
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
    type Fields = IssueTableColumns;

    fn default_list_fields() -> Vec<Self::Fields> {
        vec![
            IssueTableColumns::Key,
            IssueTableColumns::Title,
            IssueTableColumns::State,
            IssueTableColumns::AssigneeDisplayName,
            IssueTableColumns::Created,
            IssueTableColumns::Updated,
        ]
    }

    fn default_detail_fields() -> Vec<Self::Fields> {
        vec![
            IssueTableColumns::Key,
            IssueTableColumns::Title,
            IssueTableColumns::State,
            IssueTableColumns::AssigneeDisplayName,
            IssueTableColumns::Description,
            IssueTableColumns::Created,
            IssueTableColumns::Updated,
        ]
    }

    fn render_field_header(field: &Self::Fields) -> Cell {
        match field {
            IssueTableColumns::Key => Cell::new("Key").add_attribute(Attribute::Bold),
            IssueTableColumns::Title => Cell::new("Title"),
            IssueTableColumns::Description => Cell::new("Description"),
            IssueTableColumns::State => Cell::new("State"),
            IssueTableColumns::AssigneeDisplayName => Cell::new("Assignee"),
            IssueTableColumns::Created => Cell::new("Created"),
            IssueTableColumns::Updated => Cell::new("Updated"),
        }
    }

    fn render_field(item: &Self, field: Self::Fields) -> Cell {
        let timeformat = time::format_description::parse("[year]-[month]-[day]").unwrap();
        match field {
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
        }
    }
}
