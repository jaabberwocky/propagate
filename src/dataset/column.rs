#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub min: ColumnDataType,
    pub max: ColumnDataType,
}

#[derive(Debug)]
pub enum ColumnDataType {
    Int(i32),
}

pub enum DatasetMode {
    Default,
    FirstRange,
}
