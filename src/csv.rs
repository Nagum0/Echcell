use std::fs;
use std::io::Write;

use super::error::CsvError;

mod exprs;
use exprs::eval;

#[derive(Debug)]
pub struct CSV {
    pub file: String,
    pub header: Vec<String>,
    pub body: Vec<Vec<String>>,
}

/// Defines the type of acceptable ranges.
#[derive(Debug)]
enum RangeType {
    Row,
    Col,
}

impl RangeType {
    /// Returns the type of a given range.
    fn get_range_type(x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> Option<RangeType> {
        if x_start == x_end {
            Some(RangeType::Col)
        }
        else if y_start == y_end {
            Some(RangeType::Row)
        }
        else {
            None
        }
    }
}

impl CSV {
    /// -- PRIVATE --
    /// CSV PARSER:
    /// Parses input into a tuple of Vec<String>(CSV header) and Vec<Vec<String>>(CSV body).
    /// Returns a Result type of the tuple or CsvError. 
    fn parse(file_path: &String) -> Result<(Vec<String>, Vec<Vec<String>>), CsvError> {
        // Splitting file into lines:
        let lines: Vec<String> = match fs::read_to_string(file_path) {
            Ok(contents) => contents.lines().map(String::from).collect(),
            Err(_) => return Err(CsvError::FileError("Could not read csv file...".to_string())),
        };

        // Splitting lines by commas:
        let mut data: Vec<Vec<String>> = Vec::new();
        for line in lines.iter() {
            let split_line: Vec<String> = line.split(',').map(String::from).collect();
            data.push(split_line);
        }

        // Spearating to header and body:
        let header: Vec<String> = data[0].to_vec();
        let body: Vec<Vec<String>> = data[1..data.len()].to_vec();

        Ok((header, body))
    }

    /// Receives a cell pointer and returns a column index or a CsvError::CellPError().
    fn get_column_cor(&self, cell_pointer: &str) -> Result<usize, CsvError> {
        match self.header.iter().position(|col| col == &cell_pointer[0..1]) {
            Some(val) => Ok(val),
            None             => Err(CsvError::CellPError("Column index out of bounds...".to_string())), 
        }
    }

    /// Receives a cell pointer and returns a row index or an CsvError::CellPError().
    /// Also checks whether the row coordinate is in bounds.
    fn get_row_cor(&self, cell_pointer: &str) -> Result<usize, CsvError> {
        // Getting the coordinate:
        let cor = match cell_pointer[1..cell_pointer.len()].parse::<usize>() {
            Ok(val) => val - 1,
            Err(_)         => return Err(CsvError::CellPError("Incorrect row index specifier...".to_string())),
        };

        // Checking whether it's outside of bounds:
        if cor > self.body.len() {
            return Err(CsvError::CellPError("Row index out of bounds...".to_string()));
        }

        Ok(cor)
    }

    /// -- PUBLIC --
    /// Creates a new CSV object.
    /// Returns a Result type of Self(CSV) or std::io::Error.
    pub fn new(file_path: String) -> Result<Self, CsvError> {
        let parsed_data = Self::parse(&file_path)?;
        Ok(Self { file: file_path, header: parsed_data.0, body: parsed_data.1 })
    }

    /// Returns a Result type of item (String. An item from the csv body.) or a CsvError with a specified error message.
    /// This function can be called on a CSV object and takes in a cell pointer in this format: "A1", "C2", ...
    #[allow(unused)]
    pub fn get_cell_value(&self, cell_pointer: &str) -> Result<String, CsvError> {
        // Getting the x coordinate:
        let x_cor = self.get_column_cor(cell_pointer)?;
        // Getting the y coordinate:
        let y_cor = self.get_row_cor(cell_pointer)?;

        Ok(self.body[y_cor][x_cor].to_string())
    }

    /// Returns a Result type of a vector of strings or a specified error message.
    /// Receives 2 cell pointers a start of a range and an end of a range.
    /// The resulting vector of strings are the values of cells inside the given range.
    /// Either the column or row index must match on both cell pointers (ranges are either column base or row based; nothing diagonal).
    #[allow(unused)]
    pub fn get_range_values(&self, cell_pointer_start: &str, cell_pointer_end: &str) -> Result<Vec<String>, CsvError> {
        // Getting the coordinates:
        let x_start = self.get_column_cor(cell_pointer_start)?;
        let x_end = self.get_column_cor(cell_pointer_end)?;
        let y_start = self.get_row_cor(cell_pointer_start)?;
        let y_end = self.get_row_cor(cell_pointer_end)?;

        // Determining the range type (row, column or nil):
        if let Some(r_type) = RangeType::get_range_type(x_start, x_end, y_start, y_end) {
            match r_type {
                RangeType::Row => Ok(self.body[y_start][x_start..=x_end].to_vec()),
                RangeType::Col => Ok((y_start..=y_end).map(|y| self.body[y][x_start].clone()).collect()),
            }
        }
        else {
            Err(CsvError::RangeError("Unknown range type...".to_string()))
        }
    }
}

/// Iterates over the created CSV object and evaluates all the expressions found and creates an output csv file.
///
/// DOES NOT HANDLE INVALID EXPRESSIONS. (They will be parsed into the output file with a error message inside the corresponding cell).
/// 
/// Returns a Result type of () or CsvError if the file generation failed.
pub fn generate_output(csv: &CSV) -> Result<(), CsvError> {
    // Creating output file:
    let mut output_file = match fs::File::create(format!("out_{}", csv.file)) {
        Ok(f) => f,
        Err(_) => return Err(CsvError::FileError("Could not create output file...".to_string())),
    };
    
    // Writing the header to the output file:
    match writeln!(&mut output_file, "{}", csv.header.join(",")) {
        Ok(_)  => {},
        Err(_) => return Err(CsvError::FileError("Could not write to output file...".to_string())),
    }

    // Writing the body and evaluating the expressions:
    csv.body.iter().try_for_each(|row| {
        let buffer: String = row.iter().map(|item| eval(item, &csv)).collect::<Vec<_>>().join(",");
        match writeln!(&mut output_file, "{}", buffer) {
            Ok(_)  => Ok(()),
            Err(_) => return Err(CsvError::FileError("Could not write to output file...".to_string())),
        }
    })
}
