use crate::CSV;
use crate::CsvError;
#[allow(unused)]
use crate::csv::exprs::{
    Token,
    BinaryOp,
    CmpOp,
    Functions,
};

/// ---------------------------------------------------
/// -------------------- FUNCTIONS --------------------
/// ---------------------------------------------------

/// ---------------------------------------------------
/// --------------------   Caller  --------------------
/// ---------------------------------------------------
/// This function receives a Func and arguments and calls the proper func_<name>.
pub fn func_caller(csv: &CSV, func: &Token, args: &Vec<Token>) -> Result<String, CsvError> {
    // Check whether func is truly a function token:
    if let Token::Func(f) = func {
        match f {
            Functions::Sum  => Ok(func_sum(&csv, &args)?.to_string()),
            Functions::Avg  => Ok(func_avg(&csv, &args)?.to_string()),
            Functions::Calc => Ok(func_calc(&csv, &args)?.to_string()),
            Functions::If   => Ok(func_if(&csv, &args)?),
        }
    }
    else {
        Err(CsvError::ExprError("Unknown function...".to_string()))
    }
}

/// ---------------------------------------------------
/// --------------------     IF    --------------------
/// ---------------------------------------------------
fn func_if(_csv: &CSV, args: &Vec<Token>) -> Result<String, CsvError> {
    println!("[IF ARGS] {:?}", args);

    // Split condition and the rest:
    let mut i = 0;

    let split_if = args.iter().fold(vec![vec![]], |mut acc, token| {
        match token {
            Token::Then => {
                i += 1;
                acc.push(Vec::new());
            },
            Token::Else => {
                i += 1;
                acc.push(Vec::new());
            },
            _ => acc[i].push(token.clone()),
        }
        acc      
    });
      
    println!("[SPLIT IF] {:?}", split_if);

    Ok("astolfo".to_string())
}

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

/// ---------------------------------------------------
/// --------------------    AVG    --------------------
/// ---------------------------------------------------
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
