mod build;
mod table;
mod verify;



pub use {
    arya::build::JsonBuilder,
    arya::build::JsonSource,
    arya::verify::JsonVerifier,
};



/// the error type for arya json errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonError {
    /// the input stream is not valid json.
    Invalid,

    /// conversion to a string failed because the input stream is not valid a utf8 sequence.
    Utf8,

    /// parse failed because the input stream contained an object exceeding the maximum specified depth.
    Exceeded,
}



/// describes json parse status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonStatus {
    /// more characters are needed to complete this json object.
    Continue,

    /// this object is a valid json object.
    Valid,
}
