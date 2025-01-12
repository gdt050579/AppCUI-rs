pub struct ExpressionParser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self {
            chars: expression.chars().peekable(),
        }
    }

    pub fn parse_expression(&mut self) -> Result<f64, ()> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<f64, ()> {
        let mut result = self.parse_factor()?;
        while let Some(&c) = self.chars.peek() {
            match c {
                '+' | '-' => {
                    self.chars.next();
                    let next_factor = self.parse_factor()?;
                    if c == '+' {
                        result += next_factor;
                    } else {
                        result -= next_factor;
                    }
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<f64, ()> {
        let mut result = self.parse_primary()?;
        while let Some(&c) = self.chars.peek() {
            match c {
                '*' | '/' => {
                    self.chars.next();
                    let next_primary = self.parse_primary()?;
                    if c == '*' {
                        result *= next_primary;
                    } else {
                        if next_primary == 0.0 {
                            return Err(());
                        }
                        result /= next_primary;
                    }
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_primary(&mut self) -> Result<f64, ()> {
        self.skip_whitespace();
        if let Some(&c) = self.chars.peek() {
            if c == '(' {
                self.chars.next();
                let result = self.parse_term()?;
                if self.chars.next() == Some(')') {
                    return Ok(result);
                } else {
                    return Err(());
                }
            } else if c.is_ascii_digit() || c == '.' {
                return self.parse_number();
            }
        }
        Err(())
    }

    fn parse_number(&mut self) -> Result<f64, ()> {
        let mut num_string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() || c == '.' {
                num_string.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        num_string.parse::<f64>().map_err(|_err| ())
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }
}