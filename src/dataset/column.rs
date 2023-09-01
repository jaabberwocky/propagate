use std::fmt;

#[derive(Debug)]
pub struct Column {
    name: String,
    min: ColumnDataType,
    max: ColumnDataType,
}

impl Column {
    pub fn new(name: String, min: ColumnDataType, max: ColumnDataType) -> Self {
        match (&min, &max) {
            (ColumnDataType::Int(_), ColumnDataType::Int(_)) => {}
            (ColumnDataType::Float(_), ColumnDataType::Float(_)) => {}
            _ => panic!("Column min and max must be of the same type"),
        }

        match min < max {
            true => {}
            false => panic!("Column min must be less than max"),
        }
        Column { name, min, max }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_min(&self) -> &ColumnDataType {
        &self.min
    }

    pub fn get_max(&self) -> &ColumnDataType {
        &self.max
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ColumnDataType {
    Int(i32),
    Float(f32),
}

impl fmt::Display for ColumnDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColumnDataType::Int(value) => write!(f, "{}", value),
            ColumnDataType::Float(value) => write!(f, "{:.2}", value), // 2 decimal places
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DatasetMode {
    Default,
    FirstRange,
}
