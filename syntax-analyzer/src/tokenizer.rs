use super::types::token::{Token, TokenType};

pub struct JackTokenizer {
    pub code: String,
    pub pointer: (usize, usize),
    pub current_token: Option<Token>,
}

impl JackTokenizer {
    pub fn new(data: String) -> Self {
        Self {
            code: data,
            pointer: (0, 0),
            current_token: None,
        }
    }

    pub fn advance(&mut self) {
        let mut step_count = 0;
        loop {
            self.goto_next_char();
            step_count += 1;

            let buffer = self
                .code
                .get(self.pointer.1 - 1..self.pointer.1)
                .unwrap_or("")
                .to_string();
            let token = Token::new(buffer.clone());

            if buffer == " " || token.token_type == TokenType::Symbol {
                let look_ahead = (step_count > 1) as usize;
                let current_buffer = self
                    .code
                    .get(self.pointer.0..self.pointer.1 - look_ahead)
                    .unwrap_or("")
                    .to_string();

                self.current_token = Some(Token::new(current_buffer));
                self.update_pointers(look_ahead);
                break;
            }
        }
    }

    fn goto_next_char(&mut self) {
        self.pointer = (self.pointer.0, self.pointer.1 + 1);
    }

    fn update_pointers(&mut self, loook_ahead: usize) {
        self.pointer = (self.pointer.1 - loook_ahead, self.pointer.1 - loook_ahead)
    }
}
