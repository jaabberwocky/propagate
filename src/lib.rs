use csv::QuoteStyle;
use rand::Rng;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub struct Dataset {
    columns: Vec<Column>,
    rows: i32,
    sep: String,
    output: String,
}

impl Dataset {
    pub fn build(
        column_names: Vec<String>,
        ranges: Vec<String>,
        rows: i32,
        sep: String,
        output: String,
    ) -> Self {
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
        Dataset {
            columns,
            rows,
            sep,
            output,
        }
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

    pub fn generate_dataset(&self) {
        let mut generated_dataset: Vec<String> = Vec::new();

        // add column names
        let column_header: Vec<String> = self.columns.iter().map(|c| c.name.clone()).collect();
        generated_dataset.push(column_header.join(&self.sep));

        for _ in 0..self.rows {
            let mut row: Vec<String> = Vec::new();

            for column in &self.columns {
                let ColumnDataType::Int(min) = &column.min;
                let ColumnDataType::Int(max) = &column.max;
                let value = rand::thread_rng().gen_range(*min..=*max).to_string();
                row.push(value);
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
        println!("Successfully wrote to file {}", &self.output);
        Ok(())
    }
}

#[derive(Debug)]
struct Column {
    name: String,
    min: ColumnDataType,
    max: ColumnDataType,
}

#[derive(Debug)]
enum ColumnDataType {
    Int(i32),
}
