use comfy_table::{Cell, ContentArrangement, Table};

pub trait Render: Sized {
    type TableColumns;

    fn default_list_table_columns() -> Vec<Self::TableColumns>;

    fn list_table_header(columns: &[Self::TableColumns]) -> Vec<Cell>;

    fn list_table_row(item: &Self, columns: &[Self::TableColumns]) -> Vec<Cell>;

    fn list_table_rows(items: &[Self], columns: &[Self::TableColumns]) -> Vec<Vec<Cell>> {
        items
            .iter()
            .map(|item| Self::list_table_row(item, columns))
            .collect()
    }

    fn render_list_table(items: &[Self], columns: &[Self::TableColumns]) -> String {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(Self::list_table_header(columns))
            .add_rows(Self::list_table_rows(items, columns));

        table.to_string()
    }
}
