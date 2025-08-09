#![allow(dead_code, unused_variables, unused_assignments,unused_mut)]
use std::io;


const DEFAULT_NUMERIC_VALUE:f32 = 0.0;
const LEAF_ERROR_RESULT:(i32,i32)=(-1,-1);
const DEFAULT_OPER_ENUM:char = ' ';


#[derive(PartialEq, Eq, Debug,Clone)]
enum TokenKind{
    Operator,
    Data
}

fn main() {
    // type in text calculator
    
    println!("Enter a math expression:\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut v_tokens: Vec<Token> = Vec::new();
    let mut v_i_number:Vec<f32>=Vec::new();
    
    // create tokens from input
    v_tokens= string_to_tokens(String::from(input.trim_end()));
    print_tokens(&v_tokens);
    // validate number of parentheses

    let paren_types_eq = open_close_parens_match(&v_tokens);
    if paren_types_eq {
        println!("paren types are equal")
    }else{
        println!("Error: mismatched parentheses.");
        std::process::exit(1);
    }

    // evaluate until no more operators, leaf nodes first
    print_tokens(&v_tokens);
    let mut all_parens_resolved: bool = false;
    loop {
        let node_bounds= find_leaf_node_expression_bounds(&v_tokens);
        if node_bounds.0 == node_bounds.1 && node_bounds.1 == -1{
            // leaf node not found
            println!("paren node not found");
            all_parens_resolved = true;
            break;
        }else{
            let mut start: usize= 1+node_bounds.0 as usize;
            let mut end: usize = node_bounds.1 as usize;
            if -1 ==  node_bounds.1 {
                end =v_tokens.len() -1;
            } 
            println!("start {} end {}",start,end);
            let slice_tokens = &v_tokens[start..end]; // make slice
            let v_slice_tokens:Vec<Token> = slice_tokens.to_vec(); // conv to vec
            let vo_slice_tokens = v_slice_tokens.to_owned();
            print_tokens(&vo_slice_tokens);
            println!("vo len {}",vo_slice_tokens.len());
            let output_tokens=evaluate_leaf_expression(&vo_slice_tokens);
            match output_tokens{
                Ok(value)=> v_tokens = value,
                Err(err)=>println!("{}",err),
            }

        }
    }

    // evaluate final expression
    let mut expression_has_operators: bool = true;
    loop{
        expression_has_operators = check_expression_has_operators(&v_tokens);
        if !expression_has_operators{
            break;
        }
        if v_tokens.len() ==2{
            return
        }
        let v_tokens_opt:Result<Vec<Token>,String> = evaluate_leaf_expression(&v_tokens);
        match v_tokens_opt{
            Ok(value)=>{
                v_tokens=value;
            },
            Err(err)=>{
                println!("{}",err);
            }
        }

    }

    match v_tokens.get(0){
        Some(value)=>{
            println!("Final result: {}", value.n_value)
        },
        None=>println!("Failed to calculate final result.")
    }

    


}

#[derive(Clone, Debug)]
struct Token{
    // token fields
    token_kind:TokenKind,
    o_value:char,
    n_value:f32,
}

impl Token{
    // token methods
    pub fn is_number(&self)->bool{
        return self.token_kind == TokenKind::Data
    }
    pub fn to_string(&self)->String{
        if self.token_kind==TokenKind::Data{
            return self.n_value.to_string()
        }else if self.token_kind ==TokenKind::Operator{
            let mut str_tmp = String::new();
            str_tmp.push(self.o_value);
            return str_tmp;

        }else{
            return String::from("ERROR");
        }
    }

}

fn print_tokens(tokens:&Vec<Token>){
    println!("Tokens amount: {}",tokens.len());
    let mut iterator= tokens.iter();
    let mut position = 0;
    while let Some(token) = iterator.next(){
        let value = &token.to_string();
        println!("Pos: {} value: {}",position,value);
        position+=1;
    }

}

fn check_expression_has_operators(input_tokens:&Vec<Token>)->bool{
    
    let mut iterator= input_tokens.iter();
 
    while let Some(token) = iterator.next(){
        if token.token_kind == TokenKind::Operator{
            return true
        }
    }
    return false

}

fn is_binary_operation(tokens:Vec<Token>)->bool{
    
    let mut l_number = false;
    let mut middle_oper= false;
    let mut r_number = false;
   
    if tokens.len()!=3{
        return false
    }else{
        if let Some(oper) = tokens.get(1){
            if oper.is_number() {
                middle_oper = true;
            }else{
                return false;
            }
        }else{
            return false;
        }
        let middle_token_char=str_to_char(tokens.get(1).unwrap().to_string().clone());
        let number_l=tokens.get(0).unwrap();
        let number_r=tokens.get(2).unwrap();
        if number_l.is_number()&&number_r.is_number(){
            
            return true;
        }else{
            return false;
        }

    }
    
    
}

fn perform_binary_operation(tokens:Vec<Token>)->Result<Token,String>{
    //let mut output_tokens:Vec<Token>=Vec::new();
    let mut num_result: f32=DEFAULT_NUMERIC_VALUE;
    if tokens.len()!=3{
        Err( String::from("error: wrong number of arguments"))
    }else{
        let middle_token_char=str_to_char(tokens.get(1).unwrap().to_string().clone());
        let number_l=tokens.get(0).unwrap().n_value;
        let number_r=tokens.get(2).unwrap().n_value;
        if is_math_operator(middle_token_char){
            match middle_token_char{
                '+'=> {num_result=number_l+number_r},
                '-'=> {num_result=number_l-number_r},
                '/'=> {num_result=number_l/number_r},
                '*'=> {num_result=number_l*number_r},
                '^'=> {num_result=number_l.powf(number_r)},
                _=> {
                },

            }
            let num_result_token = float_to_token(num_result);
            Ok(num_result_token)
        }else{
            Err( String::from("error: operator is invalid"))
        }

        
    }
    
    
}



fn evaluate_leaf_expression(tokens:&Vec<Token>)->Result<Vec<Token>,String>{
    /*
    return single token or err
    ensure length is odd
    ensure begins and ends with number
    find highest order operator
     */
    let mut tokens_temp:Vec<Token>=Vec::new();
    tokens_temp = tokens.clone();
    println!("tokens_temp length {}",tokens_temp.len());
    let mut output_tokens:Vec<Token>=Vec::new();
    let token_start=tokens.first().unwrap().is_number();
    let token_end=tokens.last().unwrap().is_number();
    if tokens.len()%2==0{
        Err( format!("error: leaf exp must be odd num tokens long. len {}",tokens.len()))
    }else if !token_start || !token_end {
        Err( format!("error: leaf exp invalid start/end values len {}",tokens.len()))
    } else{
        loop{
            if tokens_temp.len()==1 {
                // leaf expression is complete
                let ret_token:Token = tokens_temp.get(0).unwrap().clone();
                output_tokens.push(ret_token);
                return Ok(output_tokens);
                
            }
            let highest_oper_index: i32 = expression_find_highest_order_operator(&tokens);
            if highest_oper_index==0 ||highest_oper_index==tokens.len()as i32-1 {
                return Err(String::from("error: op can't be at start or end of expression"));
            }else{
                let triplet_index_0 = highest_oper_index-1;
                let triplet_index_2 = highest_oper_index+1;
                let mut tokens_triplet:Vec<Token>=Vec::new();
                // make triplet: number, oper, number
                tokens_triplet.push(tokens.get(triplet_index_0 as usize).unwrap().clone());
                tokens_triplet.push(tokens.get(highest_oper_index as usize).unwrap().clone());
                tokens_triplet.push(tokens.get(triplet_index_2 as usize).unwrap().clone());
                let mut new_token:Token ;
                let result = perform_binary_operation(tokens_triplet);
                match result{
                    Ok(value)=>{new_token=value},
                    Err(error)=>{println!("{}",error);
                    new_token=make_number_token(&DEFAULT_NUMERIC_VALUE.to_string());
                },
                }
                //make first part
                let slice_tokens_begin = &tokens[0..triplet_index_0 as usize ]; // make slice
                let v_slice_tokens:Vec<Token> = slice_tokens_begin.to_vec(); // conv to vec
                let vo_slice_tokens = v_slice_tokens.to_owned();
                let old_tokens_length = tokens_temp.len();
                let change_index = triplet_index_0 as usize;
                tokens_temp.remove(change_index);
                tokens_temp.remove(change_index);
                tokens_temp.remove(change_index);
                tokens_temp.insert(change_index,new_token);

            }

        }

        
    }
    
    
}

fn expression_find_highest_order_operator(tokens:&Vec<Token>)->i32{
    /*
    use after parens resolved
    ret index location of highest order operator
    ranks:
    -1 nothing
    0 add sub
    1 mul div
    2 exp
    
     */
    let mut highest_oper_index: i32 = -1;
    let mut highest_oper_rank: i32 = -1;
    
    for (index, token) in tokens.iter().enumerate() {
        if !token.is_number(){
            let char_val = str_to_char(token.to_string().clone());
            match char_val{
                '^'=>{
                    if highest_oper_rank < 2{
                        highest_oper_rank = 2;
                        highest_oper_index = index as i32;
                    }
                
                },
                '/'|'*'=>{
                    if highest_oper_rank < 1{
                        highest_oper_rank = 1;
                        highest_oper_index = index as i32;
                    }
                },
                '-'|'+'=>{
                    if highest_oper_rank < 0{
                        highest_oper_rank = 0;
                        highest_oper_index = index as i32;
                    }
                },
                _=>{
                    println!("non matching value: {}",char_val);
                }
            }
        }
        
    }
    return highest_oper_index as i32
    
    
}

fn resolve_parens(tokens:Vec<Token>)->Result<Vec<Token>,String>{
    let mut output_tokens:Vec<Token>=Vec::new();
    if true{
        Err( String::from("error"))
    }else{

        Ok(output_tokens)
    }
    
    
}

fn resolve_expression_without_parens(tokens:Vec<Token>)->Result<Vec<Token>,String>{
let mut output_tokens:Vec<Token>=Vec::new();
    if true{
        Err( String::from("error"))
    }else{

        Ok(output_tokens)
    }
}

fn find_leaf_node_expression_bounds(tokens:&Vec<Token>)->(i32,i32){
    // return (-1,-1) if no valid leaf node found
    let mut left_paren_location: i32 = -1;
    let mut right_paren_location:i32 = -1;
    let mut found_left_paren = false;
    let tokens_length=tokens.len();

    for (ind, token) in tokens.iter().enumerate()  {
        
        if !token.is_number(){
            //let first_char = token.s_value.chars.next().unwrap();
            let str_value:String = token.to_string().clone();
            let token_char_value : char = str_value.chars().next().unwrap();
            if token_char_value == '('{
                found_left_paren=true;
                left_paren_location=ind as i32;
            } else if token_char_value == ')' && found_left_paren{
                return (left_paren_location,ind as i32)
            }else if token_char_value == ')' && !found_left_paren{
                return LEAF_ERROR_RESULT;
            }
        }
        
        }
        println!("find_leaf_node_expression_bounds failed");
        return LEAF_ERROR_RESULT;

}

fn open_close_parens_match(tokens:&Vec<Token>)->bool{
    // return true if amount of open parens eq amount closing parens
    let mut left_paren_amt: i32 = 0;
    let mut right_paren_amt:i32 = 0;
    let tokens_length=tokens.len();

    for (ind, token) in tokens.iter().enumerate()  {
        
        if !token.is_number(){
            let token_char_value = str_to_char(token.to_string().clone());
            if token_char_value == '('{
                left_paren_amt+=1;
            } else if token_char_value == ')'{
                right_paren_amt+=1;
            }
        }
        
        }
    return left_paren_amt== right_paren_amt;

}

fn str_to_char(my_str:String)->char{
    let char_value : char = my_str.chars().next().unwrap();
    return char_value
}

fn has_parens(tokens:Vec<Token>)->bool{
    let mut found_left_paren = false;
    let mut iterator = tokens.iter();
    while let Some(token) = iterator.next() {
        if !token.is_number(){
            let token_char_value = str_to_char(token.to_string().clone());
            if token_char_value == '('{
                found_left_paren=true;
            } else if token_char_value == ')' && found_left_paren{
                return true;
            }else{
                continue;
            }
        }
        
        }
        return false;

    
}

fn has_token_eq_to_char(tokens:Vec<Token>,my_char:char)->bool{
    // search tokens vector for matching char, true if found
    let mut found_left_paren = false;
    let mut iterator = tokens.iter();
    while let Some(token) = iterator.next() {
        if token.to_string().len()==1{
            let token_char_value = str_to_char(token.to_string().clone());
            if token_char_value == my_char{
                return true
            }
        }
        
        }
        return false;

    
}

fn is_operator(my_char: char)->bool{
    match my_char {
        '+'|'-'|'/'|'*'|'('|')'|'^'=>true,
        _=>false,
    }
}

fn is_math_operator(my_char: char)->bool{
    match my_char {
        '+'|'-'|'/'|'*'|'^'=>true,
        _=>false,
    }
}

fn is_number(my_char: char)->bool{
    match my_char {
        '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0'|'.'=>true,
        _=>false,
    }
}



fn split_string_by_rules(my_string:String)->Vec<String>{
    let mut output_strings = Vec::new();
    let mut string_tmp = String::new();
    let mut current_index=0;
    let mut current_char:char;
    loop{
        if current_index>=my_string.len(){
            break;
        }
        current_char= my_string.chars().nth(current_index).unwrap();

        if is_operator(current_char){
            if string_tmp.len() > 0{
                output_strings.push(string_tmp);
                string_tmp = String::new();
            }
            let mut new_string:String = String::new();
            new_string.push(current_char);
            output_strings.push(new_string);
            current_index+=1;
        }else if is_number(current_char){
            string_tmp.push(current_char);
            current_index+=1;

        }else{
            println!("Invalid char {} ",current_char);
            current_index+=1;
        }

    }
    return output_strings
}

fn make_operator_token(my_string:String)->Token{
    let oper_char = my_string.chars().nth(0).unwrap();
            let token = Token {
                    token_kind:TokenKind::Operator,
                    n_value:DEFAULT_NUMERIC_VALUE,
                    o_value:oper_char,
                };
                return token
}


fn make_number_token(my_string:&String)->Token{
                let num_val: f32;
                if let Ok(n) = my_string.parse::<f32>(){
                    num_val=n;
                }else{
                    num_val=DEFAULT_NUMERIC_VALUE;
                }
                //create new token object
                let token = Token {
                    token_kind:TokenKind::Data,
                    n_value:num_val,
                    o_value:DEFAULT_OPER_ENUM,
                };
                return token
}

fn string_to_tokens(my_string:String)->Vec<Token>{
    /*
    Turns a string into a vector of tokens.
    Operators are one character and turned immediately into tokens.
    Numbers and decimal are added up to a temp string,
    then parsed into a number, and a data token is created.
     */
    let mut v_tokens:Vec<Token> = Vec::new();

    let sentence_length = my_string.len();

   
    let mut string_tmp = String::new();
    let mut current_index=0;
    let mut current_char:char;
    loop{
        if current_index>=my_string.len(){
            break;
        }
        current_char= my_string.chars().nth(current_index).unwrap();
        println!("current char {}",current_char);

        if is_operator(current_char){
            if string_tmp.len() > 0{
                //if last char was number and this is not
                let new_token = make_number_token(&string_tmp);
                v_tokens.push(new_token);
                string_tmp = String::new();
            }
            
            let mut new_string:String = String::new();
            new_string.push(current_char);
            let new_token = make_operator_token(new_string);
            v_tokens.push(new_token);
            current_index+=1;
        }else if is_number(current_char){
            string_tmp.push(current_char);
            current_index+=1;
            if current_index >= my_string.len(){
            let new_token = make_number_token(&string_tmp);
                v_tokens.push(new_token);
            }
        

        }else{
            println!("Invalid char {} ",current_char);
            current_index+=1;
        }

    }

    
    return v_tokens;
}

fn float_to_token(my_float:f32)->Token{
    let num_val: f32;
    let s_value_new:String =format!("{}",my_float);
    //create new token object
    let token = Token {
        token_kind:TokenKind::Data,
        n_value:my_float,
        o_value:DEFAULT_OPER_ENUM,
    };
    return token
}