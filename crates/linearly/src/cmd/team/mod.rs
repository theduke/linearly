use comfy_table::{Attribute, Cell};

use crate::render::Render;

use super::CliCommand;

pub mod list;

#[derive(clap::Subcommand)]
pub enum CmdTeam {
    #[clap(alias = "ls")]
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
    type Fields = TeamTableColumns;

    fn default_list_fields() -> Vec<Self::Fields> {
        vec![
            TeamTableColumns::Key,
            TeamTableColumns::Name,
            TeamTableColumns::Description,
            TeamTableColumns::CreatedAt,
        ]
    }

    fn default_detail_fields() -> Vec<Self::Fields> {
        Self::default_list_fields()
    }

    fn render_field_header(field: &Self::Fields) -> Cell {
        match field {
            TeamTableColumns::Key => Cell::new("Key").add_attribute(Attribute::Bold),
            TeamTableColumns::Name => Cell::new("Name"),
            TeamTableColumns::Description => Cell::new("Description"),
            TeamTableColumns::CreatedAt => Cell::new("Created"),
        }
    }

    fn render_field(item: &Self, field: Self::Fields) -> Cell {
        let timeformat = time::format_description::parse("[year]-[month]-[day]").unwrap();

        match field {
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
        }
    }
}
