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
