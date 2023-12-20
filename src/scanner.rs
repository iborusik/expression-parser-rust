use std::fmt::Display;

use crate::common::{ParseError, UToken, UTokenType};

pub struct ExprScanner {}

impl ExprScanner {    
    pub fn parse(&self, input: String) -> Result<Vec<UToken>, ParseError> {
        let res = self.break_tokens(&input);
        
        if res.is_err() {
            return Err(ParseError::UnexpectedToken);
        }

        return Ok(res.unwrap());
    }
    
    pub fn break_tokens(&self, input: &String) -> Result<Vec<UToken>, ParseError> {
        let mut v: Vec<UToken> = vec![];
        let mut index: usize = 0;
        while index < input.len() {
            let c: char = input.chars().nth(index).unwrap();
            if ExprScanner::is_digit(c) {
                let res = ExprScanner::parse_number(index, input);
                if let Err(x) = res {
                    return Err(x);
                }
                let r_ = res.unwrap();
                index = r_.0;
                v.push(r_.1);
            } else {
                let res = ExprScanner::parse_operator(index, input);
                if let Err(x) = res {
                    return Err(x);
                }          
                let r_ = res.unwrap();      
                index = r_.0;
                v.push(r_.1)
            }
            index = index + 1;
        }
                   
        return Ok(v);
    }
    
    fn is_digit(c: char) -> bool {
        return c.is_digit(10);
    }
    
    fn parse_number(from_index: usize, input_str: &String) -> Result<(usize, UToken), ParseError> {
        let mut fin_index = from_index;
        let len = input_str.len();
        while (fin_index + 1) < len && 
            ExprScanner::is_digit(
                input_str.chars().nth(fin_index + 1).unwrap()
            ) 
        {
            fin_index = fin_index + 1;
        }
        
        let mut dlt = fin_index - from_index;
        if dlt == 0 {
            dlt = 1;
        }
        let sl = input_str.get(from_index..(from_index+dlt)).unwrap();
        
        let nm: Result<i32, _> = sl.parse();
        if nm.is_err() {
            return Err(ParseError::UnexpectedToken);
        }
        
        let mut tok = UToken::new(UTokenType::Number);
        tok._val = Some(String::from(sl));
            
        let r = (
            fin_index,
            tok
        );
        
        Ok(r)
    }
    
    fn parse_operator(from_index: usize, input_str: &String) -> Result<(usize, UToken), ParseError>  {
        
        let mut token_type = UTokenType::Star;
                
        match input_str.chars().nth(from_index).unwrap() {
            '+' => token_type = UTokenType::Plus,
            '-' => token_type = UTokenType::Minus,
            '*' => token_type = UTokenType::Star,
            '/' => token_type = UTokenType::Div,
            '(' => token_type = UTokenType::Left,
            ')' => token_type = UTokenType::Right,
            '^' => token_type = UTokenType::Pow,            
            _ => return Err(ParseError::UnexpectedToken)
        };
        
        let r = (
            from_index, 
            UToken::new(token_type)
        );
        
        Ok(r)
    }
    
}