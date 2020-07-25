mod calculate;
 
fn calculate(input: &str) -> f64 {
    1.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_argument() {
        assert_eq!(calculate("1"), 1.0);        
    }
}