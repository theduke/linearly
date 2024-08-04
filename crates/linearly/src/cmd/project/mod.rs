use comfy_table::{Attribute, Cell};

use crate::render::Render;

use super::CliCommand;

pub mod list;

#[derive(clap::Subcommand)]
pub enum CmdProject {
    #[clap(alias = "ls")]
    List(list::CmdProjectList),
}

impl CliCommand for CmdProject {
    type Output = ();

    async fn run(self) -> Result<(), anyhow::Error> {
        match self {
            CmdProject::List(c) => c.run().await,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ProjectTableColumns {
    Name,
    Description,
    Status,
    Created,
    Updated,
}

impl Render for linear_api::schema::projects_list::Project {
    type Fields = ProjectTableColumns;

    fn default_list_fields() -> Vec<Self::Fields> {
        vec![
            ProjectTableColumns::Name,
            ProjectTableColumns::Description,
            ProjectTableColumns::Status,
            ProjectTableColumns::Created,
            ProjectTableColumns::Updated,
        ]
    }

    fn default_detail_fields() -> Vec<Self::Fields> {
        Self::default_list_fields()
    }

    fn render_field_header(field: &Self::Fields) -> Cell {
        match field {
            ProjectTableColumns::Name => Cell::new("Name").add_attribute(Attribute::Bold),
            ProjectTableColumns::Description => Cell::new("Description"),
            ProjectTableColumns::Status => Cell::new("Status"),
            ProjectTableColumns::Created => Cell::new("Created"),
            ProjectTableColumns::Updated => Cell::new("Updated"),
        }
    }

    fn render_field(item: &Self, field: Self::Fields) -> Cell {
        let timeformat = time::format_description::parse("[year]-[month]-[day]").unwrap();

        match field {
            ProjectTableColumns::Name => Cell::new(&item.name),
            ProjectTableColumns::Description => Cell::new(&item.description),
            ProjectTableColumns::Status => Cell::new(&item.status.name),
            ProjectTableColumns::Created => {
                let v = item
                    .created_at
                    .parse()
                    .unwrap()
                    .format(&timeformat)
                    .unwrap();
                Cell::new(v)
            }

            ProjectTableColumns::Updated => {
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
