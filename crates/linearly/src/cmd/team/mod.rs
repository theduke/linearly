use comfy_table::{Attribute, Cell};

use crate::render::Render;

use super::CliCommand;

pub mod list;

#[derive(clap::Subcommand)]
pub enum CmdTeam {
    List(list::CmdTeamList),
}

impl CliCommand for CmdTeam {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Self::List(c) => c.run().await,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TeamTableColumns {
    Key,
    Name,
    Description,
    CreatedAt,
}

impl Render for linear_api::schema::team_list::Team {
    type TableColumns = TeamTableColumns;

    fn default_list_table_columns() -> Vec<Self::TableColumns> {
        vec![
            TeamTableColumns::Key,
            TeamTableColumns::Name,
            TeamTableColumns::Description,
            TeamTableColumns::CreatedAt,
        ]
    }

    fn list_table_header(columns: &[Self::TableColumns]) -> Vec<Cell> {
        columns
            .iter()
            .map(|c| match c {
                TeamTableColumns::Key => Cell::new("Key").add_attribute(Attribute::Bold),
                TeamTableColumns::Name => Cell::new("Name"),
                TeamTableColumns::Description => Cell::new("Description"),
                TeamTableColumns::CreatedAt => Cell::new("Created"),
            })
            .collect()
    }

    fn list_table_row(item: &Self, columns: &[Self::TableColumns]) -> Vec<Cell> {
        let timeformat = time::format_description::parse("[year]-[month]-[day]").unwrap();

        columns
            .iter()
            .map(|col| match col {
                TeamTableColumns::Key => Cell::new(&item.key),
                TeamTableColumns::Name => Cell::new(&item.name),
                TeamTableColumns::Description => {
                    Cell::new(item.description.as_deref().unwrap_or_default())
                }
                TeamTableColumns::CreatedAt => {
                    let v = item
                        .created_at
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
