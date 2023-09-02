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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_column_with_valid_types() {
        let name = String::from("age");
        let min = ColumnDataType::Int(0);
        let max = ColumnDataType::Int(100);
        let column = Column::new(name.clone(), min.clone(), max.clone());
        assert_eq!(*column.get_name(), name);
        assert_eq!(*column.get_min(), min);
        assert_eq!(*column.get_max(), max);
    }

    #[test]
    #[should_panic(expected = "Column min must be less than max")]
    fn test_new_column_with_invalid_ranges() {
        let name = String::from("temperature");
        let min = ColumnDataType::Float(100.0);
        let max = ColumnDataType::Float(50.0);
        Column::new(name.clone(), min.clone(), max.clone());
    }

    #[test]
    #[should_panic(expected = "Column min and max must be of the same type")]
    fn test_new_column_with_invalid_types() {
        let name = String::from("price");
        let min = ColumnDataType::Int(0);
        let max = ColumnDataType::Float(100.0);
        Column::new(name.clone(), min.clone(), max.clone());
    }
}
