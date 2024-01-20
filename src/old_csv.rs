use std::{fs, fs::File};
use std::io::Write;

#[derive(Debug)]
pub struct CSV {
    header: Vec<String>,
    body: Vec<Vec<String>>
}

impl CSV {
    pub fn parse(file_path: String) -> Result<Self, std::io::Error> {
        println!("-- [CSV STRUCT PARSER INFO] --");
        // Splitting file to lines:
        let mut lines: Vec<String> = match fs::read_to_string(file_path) {
            Ok(str) => str.lines().map(|line| line.to_string()).collect(),
            Err(e) => return Err(e),
        };

        // Removing final comma:
        lines = lines.iter_mut().map(|line| line.trim_end_matches(',').to_string()).collect();

        // Creating CSV:
        let csv_header: Vec<String> = lines[0].split(',').map(String::from).collect();
        println!("[CSV HEADER]: {:?}", csv_header);

        let mut csv_body: Vec<Vec<String>> = Vec::new();
        for line in lines[1..lines.len()].to_vec().iter() {
            csv_body.push(line.split(',').map(String::from).collect());
        }
        println!("[CSV BODY]: {:?}", csv_body);

        Ok(
            Self { header: csv_header, body: csv_body }
        )        
    }

    pub fn eval_expr(csv: Self) -> Result<(), std::io::Error> {
        let f = File::create("output.csv")?;
        let mut output = File::options().append(true).open("output.csv")?;
        writeln!(&mut output, "{}", csv.get_header());

        let mut buffer = String::new();

        for i in csv.body.iter() {
            for j in i.iter() {
                if j == "=" {
                    buffer.push_str("EQ,");
                    continue;
                }
                buffer.push_str(format!("{},", j).as_str());
            }
            writeln!(&mut output, "{}", buffer);
            buffer.clear();
        }

        Ok(())
    }

    pub fn get_header(&self) -> String {
        self.header.iter().map(|s| format!("{},", s)).collect()
    }

    pub fn get_element(&self, x: usize, y: usize) -> Option<&String> {
        self.body.get(y)?.get(x)
    }
}