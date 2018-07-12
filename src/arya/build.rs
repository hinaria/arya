use {
    hina,

    arya::JsonError,
    arya::JsonStatus,
    arya::JsonVerifier,
};



/// expanded options for constructing a [`JsonBuilder`](./struct.JsonBuilder.html) instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonBuilderOptions {
    maximum_depth:    usize,
    initial_capacity: usize,
}

impl Default for JsonBuilderOptions {
    fn default() -> JsonBuilderOptions {
        JsonBuilderOptions {
            maximum_depth:    std::usize::MAX,
            initial_capacity: 512,
        }
    }
}



/// a string builder for json that can repair and complete incomplete ("damaged") json.
///
/// # remarks
///
/// unlike the [`JsonVerifier`](./struct.JsonVerifier.html), adding a sequence of characters that would make the
/// underlying json object invalid will cause the [`JsonBuilder`](./struct.JsonBuilder.html) to remain invalid, even if
/// more characters are added to it later.
///
/// # examples
/// ```
/// # use arya::JsonBuilder;
/// #
/// # fn main() {
/// #
/// let mut builder = JsonBuilder::new();
///
/// builder.update(r#"{
///     "name": "annie",
///     "age": 14,
///     "parents": {
///         "mother": null,
///         "bro
/// "#);
///
/// builder.update("ken");
///
/// builder.completed_string();
/// // => Ok({
/// // =>     "name": "annie",
/// // =>     "age": 14,
/// // =>     "nested": {
/// // =>         "mother": null
/// // =>     }
/// // => })
/// # }
/// ```
pub struct JsonBuilder {
    data:     Vec<u8>,
    invalid:  bool,
    verifier: JsonVerifier,
}

impl JsonBuilder {
    pub fn new() -> JsonBuilder {
        JsonBuilder {
            data:     vec![],
            invalid:  false,
            verifier: JsonVerifier::new()
        }
    }

    pub fn with_maximum_depth(maximum_depth: usize) -> JsonBuilder {
        JsonBuilder::with_options(JsonBuilderOptions { maximum_depth, ..Default::default() })
    }

    pub fn with_capacity(initial_capacity: usize) -> JsonBuilder {
        JsonBuilder::with_options(JsonBuilderOptions { initial_capacity, ..Default::default() })
    }

    pub fn with_options(options: JsonBuilderOptions) -> JsonBuilder {
        JsonBuilder {
            data:     Vec::with_capacity(options.initial_capacity),
            invalid:  false,
            verifier: JsonVerifier::with_maximum_depth(options.maximum_depth),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn status(&self) -> JsonStatus {
        self.verifier.status()
    }

    pub fn reset(&mut self) {
        self.invalid = false;

        self.data.clear();
        self.verifier.reset();
    }

    pub fn update(&mut self, source: impl JsonSource) -> Result<(), JsonError> {
        if self.invalid {
            Err(JsonError::Invalid)
        } else {
            for character in source.stream() {
                match self.verifier.update(*character) {
                    Ok(()) => {
                        self.data.push(*character);
                    },
                    Err(e) => {
                        self.invalid = true;
                        return Err(e);
                    },
                }
            }

            Ok(())
        }
    }

    pub fn bytes(self) -> Result<Vec<u8>, JsonError> {
        match self.invalid {
            true  => Err(JsonError::Invalid),
            false => Ok(self.data),
        }
    }

    pub fn string(self) -> Result<String, JsonError> {
        let data = self.bytes()?;

        String::from_utf8(data).map_err(|_| JsonError::Utf8)
    }

    pub fn completed_bytes(mut self) -> Result<Vec<u8>, JsonError> {
        if self.invalid {
            Err(JsonError::Invalid)
        } else {
            if self.verifier.status() == JsonStatus::Continue {
                let (until, tokens) = self.verifier.complete();

                self.data.truncate(until);
                self.data.extend(tokens);
            }

            Ok(self.data)
        }
    }

    pub fn completed_string(self) -> Result<String, JsonError> {
        let data = self.completed_bytes()?;

        String::from_utf8(data).map_err(|_| JsonError::Utf8)
    }
}



/// utf8 byte streams for arya's json parsers.
pub trait JsonSource {
    fn stream(&self) -> &[u8];
}

impl JsonSource for u8 {
    fn stream(&self) -> &[u8] {
        hina::as_slice(&self)
    }
}

impl JsonSource for &[u8] {
    fn stream(&self) -> &[u8] {
        &self
    }
}

impl JsonSource for Vec<u8> {
    fn stream(&self) -> &[u8] {
        &self
    }
}

impl JsonSource for &str {
    fn stream(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl JsonSource for String {
    fn stream(&self) -> &[u8] {
        self.as_bytes()
    }
}
