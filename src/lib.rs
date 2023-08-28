#[derive(Debug)]
pub struct Dataset {
    columns: Vec<Column>,
    rows: i32,
}

impl Dataset {
    pub fn build(column_names: Vec<String>, ranges: Vec<String>, rows: i32) -> Self {
        Dataset::check_args_length(&column_names, &ranges);

        let mut columns: Vec<Column> = Vec::new();
        for (i, name) in column_names.iter().enumerate() {
            let (min, max) = Dataset::parse_range(&ranges[i]);

            let column = Column {
                name: name.to_string(),
                min: ColumnDataType::Int(min),
                max: ColumnDataType::Int(max),
            };
            columns.push(column);
        }
        Dataset { columns, rows }
    }

    fn parse_range(range: &str) -> (i32, i32) {
        let range: Vec<&str> = range.split(':').collect();
        let min = range[0].parse::<i32>().expect("Failed to parse min value");
        let max = range[1].parse::<i32>().expect("Failed to parse min value");

        (min, max)
    }

    fn check_args_length(column_names: &Vec<String>, ranges: &Vec<String>) {
        match column_names.len() {
            x if x < ranges.len() => panic!("Number of columns less than number of ranges"),
            x if x > ranges.len() => panic!("Number of columns greater than number of ranges"),
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    min: ColumnDataType,
    max: ColumnDataType,
}

#[derive(Debug)]
pub enum ColumnDataType {
    Float(f32),
    Int(i32),
}
