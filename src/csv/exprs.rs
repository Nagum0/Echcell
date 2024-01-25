use super::CSV;
use super::super::error::CsvError;

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
enum Functions {
    Calc,
    Sum,
    Avg,
    If,
}

/// BINARY OPERATORS
/// Mainly used for CALC function.
#[derive(Debug, Clone, Copy)]
enum BinaryOp {
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
enum CmpOp {
    Eq,
}

/// TOKEN ENUM
/// Cell     => Holds the value of a cell as a String from the csv body (can be a cell_ptr or a string of text);
/// Func     => Represents a function with the `Functions` enum;
/// Number   => A number (currently it is a f64);
/// Operator => Holds a BinaryOp. Used for CALC function and other math expressions;
#[derive(Debug, Clone)]
enum Token {
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

        // Tokens:
        let tokens = Token::tokenize(&expr);
        //println!("[TOKENS] {:?}", tokens);
        
        if tokens.is_empty() {
            return "#[TOKEN ERROR]".to_string();
        }
        
        let args = &tokens[1..tokens.len()].to_vec();

        // We match on the function type and evaluate it.
        // Every value returned from a funcion will be parsed to a String and returned.
        match &tokens[0] {
            Token::Func(f) => {
                match f {
                    // SUM:
                    Functions::Sum  => {
                        match func_sum(&csv, &args) {
                            Ok(n)    => return n.to_string(),
                            Err(err) => return err.to_string(),
                        }
                    },
                    // AVG:
                    Functions::Avg  => {
                        match func_avg(&csv, &args){
                            Ok(n)    => return n.to_string(),
                            Err(err) => return err.to_string(),
                        }
                    },
                    // CALC:
                    Functions::Calc => {
                        match func_calc(&csv, &args) {
                            Ok(n)    => return n.to_string(),
                            Err(err) => return err.to_string(),
                        }
                    },
                    // IF:
                    Functions::If   => {
                        match func_if(&csv, &args) {
                            Ok(val)  => return val,
                            Err(err) => return err.to_string(),
                        }
                    }
                }
            },
            _ => return "#[UNKNOWN FUNCTION]".to_string(),
        }
    }
    
    // If it's not a expression:
    item.to_string()
}

/// ---------------------------------------------------
/// -------------------- FUNCTIONS --------------------
/// ---------------------------------------------------

/// ---------------------------------------------------
/// --------------------     IF    --------------------
/// ---------------------------------------------------

fn func_if(_csv: &CSV, args: &Vec<Token>) -> Result<String, CsvError> {
    println!("[IF ARGS] {:?}", args);

    Ok("astolfo".to_string())
}

/// --------------------     IF    --------------------

/// ---------------------------------------------------
/// --------------------   CALC    --------------------
/// ---------------------------------------------------

/// Evaluates a mathematical expression;
/// It will turn the received arguments (which should be numbers, cells or binary operators) into postfix form;
fn func_calc(csv: &CSV, args: &Vec<Token>) -> Result<f64, CsvError> {
    let postfix_args = infix_to_postfix(&csv, &args)?;
    //println!("[POSTFIX AGRS] {:?}", postfix_args);

    let mut stack: Vec<f64> = Vec::new();
        
    postfix_args.iter().try_for_each(|token| {
        match token {
            Token::Number(n) => {
                stack.push(*n);
                Ok(())
            },
            Token::Operator(op) => {
                //println!("[STACK] {:?} [LEN] {}", stack, stack.len());
                let val1 = match stack.pop() {
                    Some(v) => v,
                    None    => return Err(CsvError::ExprError("Incorrect math expression...".to_string())),
                };

                let val2 = match stack.pop() {
                    Some(v) => v,
                    None    => return Err(CsvError::ExprError("Incorrect math expression...".to_string())),
                };
                
                // Calculating:
                match op {
                    BinaryOp::Plus  => stack.push(val2 + val1),
                    BinaryOp::Minus => stack.push(val2 - val1),
                    BinaryOp::Mult  => stack.push(val2 * val1),
                    BinaryOp::Div   => stack.push(val2 / val1),
                }

                Ok(())
            }
            _ => Err(CsvError::ExprError("NaN".to_string())),
       } 
    })?;
    
    // If the stack is empty:
    if stack.is_empty() {
        return Err(CsvError::ExprError("Empty stack...".to_string()));
    }
    else if stack.len() > 1 {
        return Err(CsvError::ExprError("Incorrect math expression...".to_string()));
    }

    Ok(*stack.last().unwrap())
}

/// Parses arguments into postfix form for `func_calc`:
/// Iterates over the received arguments and forms a postfix expression from them;
/// This way I don't have to deal with precedence checking;
fn infix_to_postfix(csv: &CSV, args: &Vec<Token>) -> Result<Vec<Token>, CsvError> {
    //println!("[FROM POSTIX PARSER] {:?}", args);

    let mut postfix: Vec<Token> = Vec::new();
    let mut stack: Vec<BinaryOp> = Vec::new();    
    
    let _ = args.iter().try_for_each(|token| {
        match token {
            Token::Number(n) => { 
                postfix.push(Token::Number(*n));
                Ok(())
            },
            Token::Cell(cell_ptr) => {
                let value = csv.get_cell_value(cell_ptr)?;
                match value.parse::<f64>() {
                    Ok(n)  => postfix.push(Token::Number(n)),
                    Err(_) => return Err(CsvError::ExprError("NaN".to_string()))
                }
                Ok(())
            },
            Token::Operator(op) => {
                while !stack.is_empty() && !op.check_precendece(stack.last().unwrap()) {
                    postfix.push(Token::Operator(stack.pop().unwrap()));
                }
                stack.push(op.clone());

                Ok(()) 
            }
            _ => Err(CsvError::ExprError("NaN".to_string())),
        }
    })?;
    
    // Finishing up the stack:
    while !stack.is_empty() {
        match stack.pop() {
            Some(op) => postfix.push(Token::Operator(op.clone())),
            None     => {},
        }
    }

    Ok(postfix)
}

/// --------------------   CALC    --------------------

/// ---------------------------------------------------
/// --------------------    SUM    --------------------
/// ---------------------------------------------------

/// --- SUM FUNCTION:
fn func_sum(csv: &CSV, args: &Vec<Token>) -> Result<f64, CsvError> {
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
/// --------------------    SUM    --------------------

/// ---------------------------------------------------
/// --------------------    AVG    --------------------
/// ---------------------------------------------------

fn func_avg(csv: &CSV, args: &Vec<Token>) -> Result<f64, CsvError> {
    // Incorrect argument size:
    if args.len() != 2 {
        return Err(CsvError::ArgError);
    }
    
    // Calculating range length:
    let range_len = csv.get_range_len(&args[0].get_cell(), &args[1].get_cell())?;

    // To get the sum I reuse `func_sum`:
    let sum: f64 = func_sum(csv, args)?;

    Ok(sum / range_len as f64)
}

// --------------------    AVG    --------------------
