use crate::dataset::column::*;
use csv::QuoteStyle;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

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

    fn parse_range(range: &str) -> Result<(ColumnDataType, ColumnDataType), Box<dyn Error>> {
        let range: Vec<&str> = range.split(':').collect();
        match (range[0].parse::<i32>(), range[1].parse::<i32>()) {
            (Ok(_), Ok(_)) => {
                let min = range[0].parse::<i32>().expect("Failed to parse min value");
                let max = range[1].parse::<i32>().expect("Failed to parse min value");
                Ok((ColumnDataType::Int(min), ColumnDataType::Int(max)))
            }
            _ => {
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
        let file = File::create(&self.output)?;
        let buffered = BufWriter::new(file);

        let mut writer = csv::WriterBuilder::new()
            .quote_style(QuoteStyle::Never)
            .from_writer(buffered);

        for row in generated_dataset.iter() {
            let fields: Vec<&str> = row.split(&self.sep).collect();
            writer.write_record(&fields)?;
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
        let range3 = "100:1000.5";

        let parsed_range1 = Dataset::parse_range(range1).unwrap();
        let parsed_range2 = Dataset::parse_range(range2).unwrap();
        let parsed_range3 = Dataset::parse_range(range3).unwrap();

        assert_eq!(
            parsed_range1,
            (ColumnDataType::Int(1), ColumnDataType::Int(10))
        );
        assert_eq!(
            parsed_range2,
            (ColumnDataType::Float(2.5), ColumnDataType::Float(5.5))
        );
        assert_eq!(
            parsed_range3,
            (ColumnDataType::Float(100.0), ColumnDataType::Float(1000.5))
        )
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
    #[should_panic]
    fn test_min_less_than_max() {
        let min = ColumnDataType::Int(5);
        let max = ColumnDataType::Int(1);
        let name: &str = "Column1";

        let _ = Column::new(name.to_string(), min, max);
    }

    #[test]
    fn test_column_creation_int() {
        let min = ColumnDataType::Int(1);
        let max = ColumnDataType::Int(10);

        let min_test = ColumnDataType::Int(1);
        let max_test = ColumnDataType::Int(10);
        let name: &str = "Column1";

        let column = Column::new(name.to_string(), min, max);

        assert_eq!(column.get_name(), &name.to_string());
        assert_eq!(column.get_min(), &min_test);
        assert_eq!(column.get_max(), &max_test);
    }

    #[test]
    fn test_column_creation_float() {
        let min = ColumnDataType::Float(1.0);
        let max = ColumnDataType::Float(10.0);
        let name: &str = "Column1";

        let min_test = ColumnDataType::Float(1.0);
        let max_test = ColumnDataType::Float(10.0);

        let column = Column::new(name.to_string(), min, max);

        assert_eq!(column.get_name(), &name.to_string());
        assert_eq!(column.get_min(), &min_test);
        assert_eq!(column.get_max(), &max_test);
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

    #[test]
    #[should_panic]
    fn test_cols_less_than_ranges() {
        let column_names = vec!["Column1".to_string()];
        let ranges = vec!["1:10".to_string(), "20:30".to_string()];

        let _ = Dataset::check_correct_args_length(&column_names, &ranges);
    }
}
