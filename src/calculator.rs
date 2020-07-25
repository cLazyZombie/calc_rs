pub fn calculate(input: &str) -> f64 {
    1.0
}

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
}