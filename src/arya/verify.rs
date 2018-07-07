use {
    arya,
    arya::JsonError,
    arya::JsonStatus,
    arya::table::ComplexToken,
    arya::table::Token,
    arya::table::Transition,
};



#[derive(Debug, Clone, Copy, PartialEq)]
enum ValueType {
    Key,
    Array,
    Object,
}



/// a fast json syntax validator for utf8 sequences.
///
/// # remarks
///
/// this parser can continue even when an invalid character is attempted.
///
/// invocations to `update(<character>)` only update the state and validity of this parser if and only if the input
/// sequence would result in a valid json object.
///
/// if `character` would cause this json object to become invalid, `update()` will return an error but keep its state.
/// the next invocation of `update()` will operate as if the bad character had never been applied.
///
/// in that sense, you can continue to "test" a character for json validity multiple times until one is foudn.
///
/// # examples.
///
/// ```
/// # use arya::JsonVerifier;
/// #
/// # fn main() {
/// #
/// let mut json = JsonVerifier::new();
///
/// for character in r#"{ "name": "annie", "value": 1 }"#.bytes() {
///     println!(
///         "{} - {:?} - {:?}",
///         character as char,
///         json.update(character),
///         json.status());
/// }
///
/// //     { - Ok(()) - Continue
/// //       - Ok(()) - Continue
/// //     " - Ok(()) - Continue
/// //     n - Ok(()) - Continue
/// //     a - Ok(()) - Continue
/// //     m - Ok(()) - Continue
/// //     e - Ok(()) - Continue
/// //     " - Ok(()) - Continue
/// //     : - Ok(()) - Continue
/// //       - Ok(()) - Continue
/// //     " - Ok(()) - Continue
/// //     a - Ok(()) - Continue
/// //     n - Ok(()) - Continue
/// //     n - Ok(()) - Continue
/// //     i - Ok(()) - Continue
/// //     e - Ok(()) - Continue
/// //     " - Ok(()) - Continue
/// //     , - Ok(()) - Continue
/// //       - Ok(()) - Continue
/// //     " - Ok(()) - Continue
/// //     v - Ok(()) - Continue
/// //     a - Ok(()) - Continue
/// //     l - Ok(()) - Continue
/// //     u - Ok(()) - Continue
/// //     e - Ok(()) - Continue
/// //     " - Ok(()) - Continue
/// //     : - Ok(()) - Continue
/// //       - Ok(()) - Continue
/// //     1 - Ok(()) - Continue
/// //       - Ok(()) - Continue
/// //     } - Ok(()) - Valid
/// # }
/// ```
pub struct JsonVerifier {
    maximum: usize,
    state:   Token,
    stack:   Vec<ValueType>,

    length:  usize,
    last_ok: usize,
}

impl JsonVerifier {
    pub fn new() -> JsonVerifier {
        JsonVerifier::with_maximum_depth(std::usize::MAX)
    }

    pub fn with_maximum_depth(maximum_depth: usize) -> JsonVerifier {
        assert![maximum_depth > 0];

        JsonVerifier {
            stack:   vec![],
            state:   Token::Begin,
            maximum: maximum_depth,

            length:  0,
            last_ok: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn status(&self) -> JsonStatus {
        match self.state == Token::Ok && self.stack.is_empty() {
            true  => JsonStatus::Valid,
            false => JsonStatus::Continue,
        }
    }

    pub fn reset(&mut self) {
        self.length  = 0;
        self.last_ok = 0;
        self.state   = Token::Begin;

        self.stack.clear();
    }

    /// applies `character` to this json object.
    ///
    /// # remarks
    ///
    /// if `character` would cause this json object to become invalid, this method returns an error, but keeps its
    /// state. the next invocation of `update()` will operate as if the bad character had never been applied.
    pub fn update(&mut self, character: u8) -> Result<(), JsonError> {
        // utf8 continuation.
        if character >= 128 {
            return self.state(self.state)
        }


        let character_type = arya::table::character_type(character)?;
        let transition     = arya::table::transition(self.state, character_type)?;

        match transition {
            Transition::Error => {
                panic!("invariant broken: transition::error should never escape `mod table`.");
            },

            Transition::Simple(state) => {
                self.state(state)
            },

            Transition::Complex(ty) => {
                match ty {
                    ComplexToken::BraceEmptyClose => {
                        self.pop(ValueType::Key)?;
                        self.state(Token::Ok)
                    },
                    ComplexToken::BraceClose => {
                        self.pop(ValueType::Object)?;
                        self.state(Token::Ok)
                    },
                    ComplexToken::BracketClose => {
                        self.pop(ValueType::Array)?;
                        self.state(Token::Ok)
                    },
                    ComplexToken::BraceOpen => {
                        self.push(ValueType::Key)?;
                        self.state(Token::Object)
                    },
                    ComplexToken::BracketOpen => {
                        self.push(ValueType::Array)?;
                        self.state(Token::Array)
                    },
                    ComplexToken::Quote => {
                        match self.stack.last() {
                            Some(ValueType::Key)    => self.state(Token::Colon),
                            Some(ValueType::Array)  => self.state(Token::Ok),
                            Some(ValueType::Object) => self.state(Token::Ok),
                            _                       => Err(JsonError::Invalid),
                        }
                    },
                    ComplexToken::Comma => {
                        match self.stack.last() {
                            Some(ValueType::Object) => {
                                self.switch(ValueType::Object, ValueType::Key)?;
                                self.state(Token::Key)
                            },
                            Some(ValueType::Array) => {
                                self.state(Token::Value)
                            },
                            _ => {
                                Err(JsonError::Invalid)
                            },
                        }
                    },
                    ComplexToken::Kolon => {
                        self.switch(ValueType::Key, ValueType::Object)?;
                        self.state(Token::Value)
                    },
                }
            },
        }
    }



    crate fn complete(&self) -> (usize, impl Iterator<Item = u8> + '_) {
        let tokens = self.stack.iter().rev().filter_map(|ty| {
            match ty {
                ValueType::Array  => Some(b']'),
                ValueType::Object => Some(b'}'),
                ValueType::Key    => None,
            }
        });

        (self.last_ok, tokens)
    }



    fn push(&mut self, ty: ValueType) -> Result<(), JsonError> {
        if self.stack.len() < self.maximum {
            self.stack.push(ty);
            Ok(())
        } else {
            Err(JsonError::Exceeded)
        }
    }

    fn pop(&mut self, ty: ValueType) -> Result<(), JsonError> {
        if self.stack.pop() == Some(ty) {
            Ok(())
        } else {
            Err(JsonError::Invalid)
        }
    }

    fn switch(&mut self, from: ValueType, to: ValueType) -> Result<(), JsonError> {
        self.pop(from)?;
        self.push(to)?;
        Ok(())
    }

    fn state(&mut self, state: Token) -> Result<(), JsonError> {
        self.state = state;
        self.length += 1;

        if self.state == Token::Ok {
            self.last_ok = self.length;
        }

        Ok(())
    }
}
