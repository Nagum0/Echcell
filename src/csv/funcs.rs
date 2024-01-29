use crate::csv::CSV;
use crate::error::CsvError;
use super::exprs::{
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
fn func_if(csv: &CSV, args: &Vec<Token>) -> Result<String, CsvError> {
    // println!("[IF ARGS] {:?}", args);

    // Split into 3 parts:
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
    
    // println!("[SPLIT IF] {:?}", split_if);

    if split_if.len() != 3 {
        return Err(CsvError::ArgError);
    }
    
    // Checking the condition:
    let cond_args = &split_if[0];
    let cond_val = condition_eval(&csv, &cond_args)?;
    let output;

    if cond_val {
        output = &split_if[1];
    }
    else {
        output = &split_if[2];
    }

    if output.is_empty() {
        return Err(CsvError::ArgError);
    }

    match &output[0] {
        Token::Cell(val) => Ok(val.to_string()), // What if the cell contains a cell pointer?
        Token::Number(n) => Ok(n.to_string()),
        Token::Func(_)   => func_caller(&csv, &output[0], &output[1..output.len()].to_vec()),
        _ => Err(CsvError::ExprError("Wrong argument type...".to_string())),
    }
}

// Evaluates whether a condition is true or false:
fn condition_eval(csv: &CSV, cond_args: &Vec<Token>) -> Result<bool, CsvError> {
    println!("[COND ARGS] {:?}", cond_args);

    if cond_args.len() != 3 {
        return Err(CsvError::ArgError);
    }

    let op = &cond_args[1];
    let left = &cond_args[0];
    let right = &cond_args[2];
    
    // Extracting the values from left and right:
    let (l_val, r_val) = get_cmp_values(&csv, &left, &right)?;
    println!("[LEFT] {:?}, [RIGHT] {:?}", l_val, r_val);

    // Comparing left and right:
    if let Token::CmpOperator(cmp) = op {
        match cmp {
            CmpOp::Eq => Ok(CmpOp::eq(l_val, r_val)),
            CmpOp::Gt => Ok(CmpOp::gt(l_val, r_val)),
            CmpOp::Lt => Ok(CmpOp::lt(l_val, r_val)),
            CmpOp::Ge => Ok(CmpOp::ge(l_val, r_val)),
            CmpOp::Le => Ok(CmpOp::le(l_val, r_val)),
        } 
    }
    else {
        Err(CsvError::ExprError("Expected a comparison operator...".to_string()))
    }
}

// Extracts the Numbers from the tokens or cell pointers:
// Returns a CsvError if the tokens are not numbers or the cell pointers point to non number values;
fn get_cmp_values(csv: &CSV, left: &Token, right: &Token) -> Result<(f64, f64), CsvError> {
    match (left, right) {
        (Token::Number(n), Token::Number(k)) => Ok((*n, *k)),
        (Token::Cell(c1), Token::Cell(c2))   => {
            let n = match csv.get_cell_value(c1)?.parse::<f64>() {
                Ok(n)  => n,
                Err(_) => return Err(CsvError::ExprError("Uncomparable ypes...".to_string())),
            };

            let k = match csv.get_cell_value(c2)?.parse::<f64>() {
                Ok(k)  => k,
                Err(_) => return Err(CsvError::ExprError("Uncomparable ypes...".to_string())),
            };

            Ok((n.clone(), k.clone()))
        }, 
        (Token::Number(n), Token::Cell(cptr)) => {
            let k = match csv.get_cell_value(cptr)?.parse::<f64>() {
                Ok(k)  => k,
                Err(_) => return Err(CsvError::ExprError("Uncomparable ypes...".to_string())),
            };

            Ok((*n, k.clone()))
        },
        (Token::Cell(cptr), Token::Number(n)) => {
            let k = match csv.get_cell_value(cptr)?.parse::<f64>() {
                Ok(k)  => k,
                Err(_) => return Err(CsvError::ExprError("Uncomparable ypes...".to_string())),
            };

            Ok((k.clone(), *n))
        },
        _ => return Err(CsvError::ExprError("Uncomparable types...".to_string())),
    }
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
