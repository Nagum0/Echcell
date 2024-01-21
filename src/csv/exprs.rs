use super::CSV;
use super::super::error::CsvError;

/// FUNCTIONS ENUM
/// Every impletemnted function is found here.
/// Expr => Mathematical expression (+,-,*,/);
/// 
/// Sum  => Returns the sum over a range of cells;
///         It takes in 2 arguments the start of the range and the end of a range. 
///         (Currently working on)
/// 
/// Avg  => Returns the average of a range of cells;
#[derive(Debug)]
#[allow(unused)]
enum Functions {
    Expr,
    Sum,
    Avg,
}

/// TOKEN ENUM
/// Cell   => Holds the value of a cell as a String from the csv body;
/// Func   => Represents a function with the `Functions` enum;
/// Number => A number (currently it is a f64); 
#[derive(Debug)]
enum Token {
    Cell(String),
    Number(f64),
    Func(Functions),
}

impl Token {
    /// Tokenizes the input expression.
    /// Returns a vector of tokens.
    pub fn tokenize(expr: &String) -> Vec<Self> {
        let split_expr: Vec<String> = expr.split_whitespace().map(String::from).collect();
        
        split_expr.iter().map(|word| {
            if word == "SUM" {
                Self::Func(Functions::Sum)
            }
            else if word == "AVG" {
                Self::Func(Functions::Avg)
            }
            else if word == "EXPR" {
                Self::Func(Functions::Expr)
            }
            // If the word is parsable to f64 then its a Number:
            else if let Ok(n) = word.parse::<f64>() {
                Self::Number(n)
            }
            else {
                Self::Cell(word.clone())
            }
            
        }).collect::<Vec<Self>>()
    }

    /// Returns Result type of String (the value in a Token::Cell) or a CsvError::
    pub fn get_cell(&self) -> String {
        match self {
            Self::Cell(val) => val.clone(),
            _ => "Unkown".to_string(),
        }
    }
}

/// MAIN EVALUATER
/// Evaluates the input cell.
/// If the evaluation fails it returns a String with an error message.
pub fn eval(item: &String, csv: &CSV) -> String {
    if item.is_empty() {
        return "#NULL".to_string();
    }

    // If the cell contains a expression:
    if &item[0..1] == "=" {
        println!("[FOUND EXPR] {}", item);

        // Expression to be tokenized:
        let expr: String = item[2..item.len()].to_string();

        // Tokens:
        let tokens = Token::tokenize(&expr);
        println!("[TOKENS] {:?}", tokens);
        
        if tokens.is_empty() {
            return "#TOKEN ERROR".to_string();
        }

        // We match on the function type and evaluate it.
        // Every value returned from a funcion will be parse to a String and returned.
        match &tokens[0] {
            Token::Func(f) => {
                match f {
                    // SUM
                    Functions::Sum  => {
                        match func_sum(&csv, &tokens[1..tokens.len()]) {
                            Ok(n)     => return n.to_string(),
                            Err(err) => return err.to_string(),
                        }
                    },
                    Functions::Avg  => todo!(),
                    Functions::Expr => todo!(),
                }
            },
            _ => return "#UNKNOWN FUNCTION".to_string(),
        }
    }

    item.to_string()
}

// -------------------- FUNCTIONS --------------------
fn func_sum(csv: &CSV, args: &[Token]) -> Result<f64, CsvError> {
    // Incorrect argument size:
    if args.len() != 2 {
        return Err(CsvError::ArgError);
    }

    // Extracting argument values:
    let arg1 = args[0].get_cell();
    let arg2 = args[1].get_cell();

    // Getting range values:
    let range_values = csv.get_range_values(&arg1, &arg2)?;

    Ok(range_values.iter().try_fold(0.0, |acc, item| {
        match item.parse::<f64>() {
            Ok(val) => Ok(acc + val),
            Err(_) => Err(CsvError::ExprError("NaN".to_string())), 
        }
    })?)
}
