use comfy_table::{Cell, ContentArrangement, Table};

pub trait Render: Sized {
    type Fields: Copy;

    fn render_field_header(field: &Self::Fields) -> Cell;

    fn render_field(item: &Self, field: Self::Fields) -> Cell;

    fn default_list_fields() -> Vec<Self::Fields>;

    fn default_detail_fields() -> Vec<Self::Fields>;

    fn render_list_table_header(fields: &[Self::Fields]) -> Vec<Cell> {
        fields
            .into_iter()
            .map(|c| Self::render_field_header(c))
            .collect()
    }

    fn render_list_table_row(item: &Self, fields: &[Self::Fields]) -> Vec<Cell> {
        fields
            .into_iter()
            .map(|c| Self::render_field(item, *c))
            .collect()
    }

    fn render_list_table_rows(items: &[Self], fields: &[Self::Fields]) -> Vec<Vec<Cell>> {
        items
            .iter()
            .map(|item| Self::render_list_table_row(item, fields))
            .collect()
    }

    /// Render a table showing multiple items as rows.
    fn render_list_table(items: &[Self], fields: &[Self::Fields]) -> String {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(comfy_table::presets::UTF8_HORIZONTAL_ONLY)
            .set_header(Self::render_list_table_header(fields))
            .add_rows(Self::render_list_table_rows(items, fields));

        table.to_string()
    }

    /// Render a table showing a single item, wich each field as a row.
    fn render_detail_table(item: &Self, fields: &[Self::Fields]) -> String {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(comfy_table::presets::UTF8_HORIZONTAL_ONLY);

        let rows: Vec<Vec<Cell>> = fields
            .into_iter()
            .map(|field| {
                let header = Self::render_field_header(field);
                let value = Self::render_field(item, *field);
                vec![header, value]
            })
            .collect();

        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .add_rows(rows);

        table.to_string()
    }
}
