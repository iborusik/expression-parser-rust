

use crate::common::{UToken, ParseError, UTokenType};

pub trait Expression {
    fn evaluate(&self) -> Box<dyn Expression>;
    
    fn get_left(&self) -> Option<&Box<dyn Expression>>;
    
    fn get_right(&self) -> Option<&Box<dyn Expression>>;
    
    fn get_desc(&self) -> String;
}

struct BinaryExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    token: UToken
}

struct NumberLiteral {
    token: UToken
}

struct GroupingExpression {
    expression: Box<dyn Expression>
}

impl Expression for GroupingExpression {
    fn evaluate(&self) -> Box<dyn Expression> {
        todo!()
    }

    fn get_left(&self) -> Option<&Box<dyn Expression>> {
        None
    }

    fn get_right(&self) -> Option<&Box<dyn Expression>> {
        Some(&self.expression)
    }

    fn get_desc(&self) -> String {
        String::from("GroupingExpression")
    }
}

impl Expression for NumberLiteral {
    fn evaluate(&self) -> Box<dyn Expression> {
        todo!()
    }

    fn get_left(&self) -> Option<&Box<dyn Expression>> {
        None
    }

    fn get_right(&self) -> Option<&Box<dyn Expression>> {
        None
    }
    
    fn get_desc(&self) -> String {
        format!("NumberLiteral:{}", self.token._val.as_ref().unwrap())
    }    
}

impl Expression for BinaryExpression {
    fn evaluate(&self) -> Box<dyn Expression> {
        todo!()
    }

    fn get_left(&self) -> Option<&Box<dyn Expression>> {
        Some(&self.left)
    }

    fn get_right(&self) -> Option<&Box<dyn Expression>> {
        Some(&self.right)
    }
    
    fn get_desc(&self) -> String {
        format!("BinaryExpression: {}", self.token._type)
    }    
}

pub struct AstParser {
    tokens: Vec<UToken>,    
    current_token: usize
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ECallNext {
    Pow,
    Group,
    Add,
    Mul
}

impl AstParser {
    pub fn parse_fun(&mut self, tokens: Vec<UToken>) -> Result<Box<dyn Expression>, ParseError> {        
        self.tokens = tokens;
        
        return Ok(self.parse());
    }
    
    pub fn parse(&mut self)  -> Box<dyn Expression> {
        self.add()        
    }
        
    fn add(&mut self) -> Box<dyn Expression> {
        let m1 = self.mul();
                
        self.parse_binary(
            m1, 
            ECallNext::Mul, 
            vec![UTokenType::Plus, UTokenType::Minus]
        )
    }
    
    fn mul(&mut self) -> Box<dyn Expression> {
        let powl = self.group();
        self.parse_binary( powl , ECallNext::Group, vec![UTokenType::Star, UTokenType::Div])
    }
    
    fn pow(&mut self) -> Box<dyn Expression> {
        let gr = self.group();                
        self.parse_binary( gr , ECallNext::Pow, vec![UTokenType::Pow])
    }
    
    fn group(&mut self) -> Box<dyn Expression> {
        if self.match_token(&vec![UTokenType::Number]) {
            let prev = self.previous();
            return Box::new(NumberLiteral{ token: prev.clone() } )
        }
        if self.match_token(&vec![UTokenType::Left]) {
            let expr = self.parse();
            if self.match_token(&vec![UTokenType::Right]) == false {
                panic!();
            } 
            return Box::new(GroupingExpression{expression: expr})
        }
        panic!();
    }    
        
    fn parse_binary( 
        &mut self,
        start: Box<dyn Expression>,
        repeating: ECallNext,
        tokens: Vec<UTokenType>
    ) -> Box<dyn Expression> {
        let mut expression = start;
        while self.match_token(&tokens) {
            if self.is_end() {
                panic!();
            }
            let p = self.previous().clone();
            let rep = self.call_next(repeating);
            //println!("BinaryExpr: {}. children[{}, {}]", p._type, expression.get_desc(), rep.get_desc());
            expression = Box::new(
                BinaryExpression{
                    left: expression,
                    right: rep, 
                    token: p
                }
            )
        }
        return expression                
    }
            
    fn advance<'a>(&'a mut self) -> &'a UToken {
        if !self.is_end() {
            self.current_token += 1;
        }
        
        return self.previous();
    }
    
    fn match_token(&mut self, types: &Vec<UTokenType>) -> bool {
        let has_m = types.into_iter().any(|x| self.check(*x));
        if has_m {
            self.advance();
        }
        
        has_m
    }
    
    fn check(&self, t: UTokenType) -> bool {
        if self.is_end() {
            return false;
        }
        
        t == self.peek()._type
    }
    
    fn previous<'a>(&'a self) -> &'a UToken {
        return &self.tokens[self.current_token - 1];
    }
    
    fn peek<'a>(&'a self) -> &'a UToken {
        return &self.tokens[self.current_token];
    }
        
    fn is_end(&self) -> bool {
        return self.current_token >= self.tokens.len();
    }
    
    pub fn new() -> AstParser {
        AstParser { 
            tokens: vec![],            
            current_token: 0
        }
    }
    
    fn call_next(&mut self, next_call: ECallNext) -> Box<dyn Expression> {
        match next_call {
            ECallNext::Pow => self.pow(),
            ECallNext::Group => self.group(),
            ECallNext::Add => self.add(),
            ECallNext::Mul => self.mul(),
        }
    }
}