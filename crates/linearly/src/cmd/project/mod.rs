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
    type TableColumns = ProjectTableColumns;

    fn default_list_table_columns() -> Vec<Self::TableColumns> {
        vec![
            ProjectTableColumns::Name,
            ProjectTableColumns::Description,
            ProjectTableColumns::Status,
            ProjectTableColumns::Created,
            ProjectTableColumns::Updated,
        ]
    }

    fn list_table_header(columns: &[Self::TableColumns]) -> Vec<Cell> {
        columns
            .iter()
            .map(|c| match c {
                ProjectTableColumns::Name => Cell::new("Name").add_attribute(Attribute::Bold),
                ProjectTableColumns::Description => Cell::new("Description"),
                ProjectTableColumns::Status => Cell::new("Status"),
                ProjectTableColumns::Created => Cell::new("Created"),
                ProjectTableColumns::Updated => Cell::new("Updated"),
            })
            .collect()
    }

    fn list_table_row(item: &Self, columns: &[Self::TableColumns]) -> Vec<Cell> {
        let timeformat = time::format_description::parse("[year]-[month]-[day]").unwrap();

        columns
            .iter()
            .map(|col| match col {
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
            })
            .collect()
    }
}
