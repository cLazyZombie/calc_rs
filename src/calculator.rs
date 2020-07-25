pub fn calculate(input: &str) -> f64 {
    let infix_tokens = tokenize(input);
    let postfix_tokens = infix_to_postfix(&infix_tokens);
    calculate_postfix(&postfix_tokens)
}

// 1 + 2 * 3 - 4
// 1 2 3 * + 4 -
// +  * 
// post fix로 수정
// 1 2 3 * + 4 -
// stack에 operator 쌓는데
// 더 낮은 priority의 op가오면 pop?

// 1 * 2 + 3 * 4 - 5
// 1 2 * 3 4  * + 5 -


fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let splitted : Vec<&str> = input.split_whitespace().collect();
    for s in splitted {
        match s {
            "+" => tokens.push(Token::Operator(Operator::Add)),
            "-" => tokens.push(Token::Operator(Operator::Sub)),
            "*" => tokens.push(Token::Operator(Operator::Mul)),
            "/" => tokens.push(Token::Operator(Operator::Div)),
            _ => {
                let num_result = s.parse::<f64>();
                if let Ok(num) = num_result {
                    tokens.push(Token::Number(num));
                }
            }
        }
    }

    tokens
}

fn infix_to_postfix(infix: &[Token]) -> Vec<Token> {
    let mut op_stack = Vec::<Operator>::new();
    let mut postfix = Vec::<Token>::new();

    for tok in infix {
        match tok {
            Token::Operator(op) => {
                if let Some(top_op) = op_stack.last() {
                    if op.lower_precedence(top_op) {
                        while let Some(pop) = op_stack.pop() {
                            postfix.push(Token::Operator(pop));
                        }
                    }
                }
                op_stack.push(*op);
            },
            Token::Number(num) => postfix.push(Token::Number(*num)),
        }
    }

    while let Some(pop) = op_stack.pop() {
        postfix.push(Token::Operator(pop));
    }

    postfix
}

fn tokens_to_string(tokens :&[Token]) -> String {
    let mut result : String = String::new();
    for tok in tokens {
        if result.is_empty() == false {
            result.push_str(" ");
        }
        match tok {
            Token::Operator(op) => {
                result.push_str(op.to_str());
            },
            Token::Number(num) => {
                result.push_str(&num.to_string());
            },
        }
    }

    result
}

fn calculate_postfix(tokens : &[Token]) -> f64 {
    let mut num_stack = Vec::<f64>::new();
    for tok in tokens {
        match tok {
            Token::Number(num) => num_stack.push(*num),
            Token::Operator(op) => {
                let right = num_stack.pop().unwrap();
                let left = num_stack.pop().unwrap();
                let result = op.exec(left, right);
                num_stack.push(result);
            },
        }
    }
    assert!(num_stack.len() == 1);
    num_stack.pop().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(f64),
    Operator(Operator),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn lower_precedence(&self, other: &Operator) -> bool {
        self.precedence() < other.precedence()
    }

    fn precedence(&self) -> i8 {
        match self {
            Operator::Add => 0,
            Operator::Sub => 0,
            Operator::Mul => 2,
            Operator::Div => 2,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        }
    }

    fn exec(&self, left: f64, right: f64) -> f64 {
        match self {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_argument() {
        assert_eq!(calculate("1"), 1.0);        
    }

    #[test]
    fn add_two() {
        assert_eq!(calculate("1 + 2"), 3.0);
    }

    #[test]
    fn minus_two() {
        assert_eq!(calculate("3 - 1"), 2.0);
    }

    #[test]
    fn tokenize_add() {
        let tokens = tokenize("+");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Operator(Operator::Add));
    }

    #[test]
    fn tokenize_number_and_add() {
        let tokens = tokenize("1 + 2");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens, vec![Token::Number(1.0), Token::Operator(Operator::Add), Token::Number(2.0)]);
    }

    #[test]
    fn infix_to_prefix_test1() {
        let tokens = tokenize("1 + 2 * 3");
        let postfix = infix_to_postfix(&tokens);
        assert_eq!(tokens_to_string(&postfix), "1 2 3 * +");
    }

    #[test]
    fn infix_to_prefix_test2() {
        let tokens = tokenize("1 + 2 * 3 - 4 / 5");
        let postfix = infix_to_postfix(&tokens);
        assert_eq!(tokens_to_string(&postfix), "1 2 3 * + 4 5 / -");
    }
}