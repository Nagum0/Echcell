use crate::CSV;
use crate::csv::funcs::func_caller;

/// FUNCTIONS ENUM
/// Every impletemnted function is found here.
/// Calc => Mathematical expression (+,-,*,/);
///         Evaluates a given mathematical expression.
/// 
/// Sum  => Returns the sum over a range of cells;
///         It takes in 2 arguments the start of the range and the end of a range. 
/// 
/// Avg  => Returns the average of a range of cells;
///         It takes in 2 arguments the start of the range ant the end of a range.
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum Functions {
    Calc,
    Sum,
    Avg,
    If,
}

/// BINARY OPERATORS
/// Mainly used for CALC function.
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Plus,
    Minus,
    Mult,
    Div,
}

impl BinaryOp {
    /// Checks whether the operator the method was called on has higher precedence thant the comparison one.
    pub fn check_precendece(&self, op_cmp: &Self) -> bool {
        match (self, op_cmp) {
            (Self::Plus | Self::Minus, Self::Mult | Self::Div) => false,
            (Self::Mult | Self::Div, Self::Plus | Self::Minus) => true,
            _ => false,       }
    }
}

/// COMPARISON OPERATORS
/// Mainly used in _IF functions.
#[derive(Debug, Clone, Copy)]
pub enum CmpOp {
    Eq,
}

/// TOKEN ENUM
/// Cell     => Holds the value of a cell as a String from the csv body (can be a cell_ptr or a string of text);
/// Func     => Represents a function with the `Functions` enum;
/// Number   => A number (currently it is a f64);
/// Operator => Holds a BinaryOp. Used for CALC function and other math expressions;
#[derive(Debug, Clone)]
pub enum Token {
    Cell(String),
    Number(f64),
    Operator(BinaryOp),
    CmpOperator(CmpOp),
    Func(Functions),

    // IF specific tokens:
    Then,
    Else,
}

impl Token {
    /// Tokenizes the input expression.
    /// Returns a vector of tokens.
    pub fn tokenize(expr: &String) -> Vec<Self> {
        let split_expr: Vec<String> = expr.split_whitespace().map(String::from).collect();
        
        split_expr.iter().map(|word| {
            // Functions:
            if word == "SUM" {
                Self::Func(Functions::Sum)
            }
            else if word == "AVG" {
                Self::Func(Functions::Avg)
            }
            else if word == "CALC" {
                Self::Func(Functions::Calc)
            }
            else if word == "IF" {
                Self::Func(Functions::If) 
            }
            else if word == "THEN" {
                Self::Then
            }
            else if word == "ELSE" {
                Self::Else
            }

            // Binary operators:
            else if word == "+" {
                Self::Operator(BinaryOp::Plus)
            }
            else if word == "-" {
                Self::Operator(BinaryOp::Minus)
            }
            else if word == "*" {
                Self::Operator(BinaryOp::Mult)
            }
            else if word == "/" {
                Self::Operator(BinaryOp::Div)
            }

            // Comparison operators:
            else if word == "==" {
                Self::CmpOperator(CmpOp::Eq)
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

    /// Returns Result type of String (the value in a Token::Cell):
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
        return "#[NULL]".to_string();
    }

    // If the cell contains an expression:
    if &item[0..1] == "=" {
        //println!("[FOUND EXPR] {}", item);

        // Expression to be tokenized:
        // (First 2 elements are removed because they are the '= ')
        let expr: String = item[2..item.len()].to_string();

        // Tokens (tokenizing):
        let tokens = Token::tokenize(&expr);
        //println!("[TOKENS] {:?}", tokens);
        
        if tokens.is_empty() {
            return "#[TOKEN ERROR]".to_string();
        }
        
        // Creating arguments vector:
        let args = &tokens[1..tokens.len()].to_vec();
        
        // Caller function:
        // (Evaluates the functions)
        match func_caller(&csv, &tokens[0], &args) {
            Ok(val)  => return val,
            Err(err) => return err.to_string(),
        }
    }
    
    // If it's not a expression:
    item.to_string()
}
