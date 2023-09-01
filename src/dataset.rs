use crate::dataset::column::*;
use csv::QuoteStyle;
use rand::Rng;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub struct Dataset {
    columns: Vec<Column>,
    num_rows: i32,
    sep: String,
    output: String,
}

impl Dataset {
    pub fn build(
        column_names: Vec<String>,
        mut ranges: Vec<String>,
        rows: i32,
        sep: String,
        output: String,
    ) -> Self {
        match Dataset::check_correct_args_length(&column_names, &ranges) {
            DatasetMode::Default => (),
            DatasetMode::FirstRange => {
                let first_range = &ranges[0];
                let default_ranges = Dataset::create_default_ranges(&column_names, first_range);
                ranges = default_ranges;
            }
        }

        let mut columns: Vec<Column> = Vec::new();
        for (i, name) in column_names.iter().enumerate() {
            let (min, max) = Dataset::parse_range(&ranges[i]).unwrap();
            println!("{:?}", (&min, &max));

            let column = Column::new(name.to_string(), min, max);
            columns.push(column);
        }
        Dataset {
            columns,
            num_rows: rows,
            sep: sep.trim().to_string(),
            output,
        }
    }

    fn parse_range(range: &str) -> Result<(ColumnDataType, ColumnDataType), &'static str> {
        let range: Vec<&str> = range.split(':').collect();
        match range[0].parse::<i32>() {
            Ok(_) => {
                let min = range[0].parse::<i32>().expect("Failed to parse min value");
                let max = range[1].parse::<i32>().expect("Failed to parse min value");
                Ok((ColumnDataType::Int(min), ColumnDataType::Int(max)))
            }
            Err(_) => {
                let min = range[0].parse::<f32>().expect("Failed to parse min value");
                let max = range[1].parse::<f32>().expect("Failed to parse min value");
                Ok((ColumnDataType::Float(min), ColumnDataType::Float(max)))
            }
        }
    }

    fn create_default_ranges(column_names: &Vec<String>, first_range: &str) -> Vec<String> {
        let mut default_ranges: Vec<String> = Vec::new();
        let (min, max) = Dataset::parse_range(first_range).unwrap();

        for _ in 0..column_names.len() {
            default_ranges.push(format!("{}:{}", min, max));
        }
        default_ranges
    }

    fn check_correct_args_length(column_names: &Vec<String>, ranges: &Vec<String>) -> DatasetMode {
        match column_names.len() {
            x if x < ranges.len() => {
                panic!("Columns provided are less than number of ranges provider");
            }
            x if x > ranges.len() => {
                println!("Ranges provided are less than number of columns, using first range for all columns");
                DatasetMode::FirstRange
            }
            _ => DatasetMode::Default,
        }
    }

    pub fn generate_dataset(&self) {
        let mut generated_dataset: Vec<String> = Vec::new();

        // add column names
        let column_header: Vec<String> =
            self.columns.iter().map(|c| c.get_name().clone()).collect();
        generated_dataset.push(column_header.join(&self.sep));

        for _ in 0..self.num_rows {
            let mut row: Vec<String> = Vec::new();

            for column in &self.columns {
                if let (ColumnDataType::Float(min), ColumnDataType::Float(max)) =
                    (&column.get_min(), &column.get_max())
                {
                    let value = rand::thread_rng().gen_range(*min..=*max).to_string();
                    row.push(value);
                    continue;
                }
                if let (ColumnDataType::Int(min), ColumnDataType::Int(max)) =
                    (&column.get_min(), &column.get_max())
                {
                    let value = rand::thread_rng().gen_range(*min..=*max).to_string();
                    row.push(value);
                }
            }
            generated_dataset.push(row.join(&self.sep));
        }

        self.write_to_file(&generated_dataset)
            .expect("Failed to write to file");
    }

    fn write_to_file(&self, generated_dataset: &[String]) -> Result<(), Box<dyn Error>> {
        let file = File::create(&self.output);

        let mut writer = csv::WriterBuilder::new()
            .quote_style(QuoteStyle::Never)
            .from_writer(file?);

        for row in generated_dataset.iter() {
            writer.write_record([row])?;
        }

        writer.flush()?;
        println!(
            "Successfully wrote {} rows to file {}",
            &self.num_rows, &self.output
        );
        Ok(())
    }
}

pub mod column;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let column_names = vec!["Column1".to_string(), "Column2".to_string()];
        let ranges = vec!["1:10".to_string(), "20:30".to_string()];
        let rows = 5;
        let sep = ",".to_string();
        let output = "output.csv".to_string();

        let dataset = Dataset::build(column_names, ranges, rows, sep.clone(), output.clone());

        assert_eq!(dataset.columns.len(), 2);
        assert_eq!(dataset.columns[0].get_min(), &ColumnDataType::Int(1));
        assert_eq!(dataset.columns[0].get_max(), &ColumnDataType::Int(10));
        assert_eq!(dataset.columns[1].get_min(), &ColumnDataType::Int(20));
        assert_eq!(dataset.columns[1].get_max(), &ColumnDataType::Int(30));
        assert_eq!(dataset.num_rows, 5);
        assert_eq!(dataset.sep, sep);
        assert_eq!(dataset.output, output);
    }

    #[test]
    #[should_panic]
    fn test_invalid_parse_range() {
        let range = "1a:10";
        let _ = Dataset::parse_range(range).unwrap();
    }
    #[test]
    fn test_parse_range() {
        let range1 = "1:10";
        let range2 = "2.5:5.5";

        let parsed_range1 = Dataset::parse_range(range1).unwrap();
        let parsed_range2 = Dataset::parse_range(range2).unwrap();

        assert_eq!(
            parsed_range1,
            (ColumnDataType::Int(1), ColumnDataType::Int(10))
        );
        assert_eq!(
            parsed_range2,
            (ColumnDataType::Float(2.5), ColumnDataType::Float(5.5))
        );
    }

    #[test]
    fn test_create_default_ranges() {
        let column_names = vec!["Column1".to_string(), "Column2".to_string()];
        let first_range = "1:10";

        let default_ranges = Dataset::create_default_ranges(&column_names, first_range);

        assert_eq!(default_ranges.len(), 2);
        assert_eq!(default_ranges[0], "1:10");
        assert_eq!(default_ranges[1], "1:10");
    }

    #[test]
    fn test_check_correct_args_length() {
        let column_names = vec!["Column1".to_string(), "Column2".to_string()];
        let ranges1 = vec!["1:10".to_string(), "20:30".to_string()];
        let ranges2 = vec!["1:10".to_string()];

        let mode1 = Dataset::check_correct_args_length(&column_names, &ranges1);
        let mode2 = Dataset::check_correct_args_length(&column_names, &ranges2);

        assert_eq!(mode1, DatasetMode::Default);
        assert_eq!(mode2, DatasetMode::FirstRange);
    }
}
