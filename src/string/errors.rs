// Std
use std::fmt;

#[derive(Debug)]
pub struct FirstElementNotFound {
    message: String,
}

impl FirstElementNotFound {
    pub fn new(element: &str) -> FirstElementNotFound {
        let _message = format!("The first element '{}' was not found", &element);
        FirstElementNotFound { message: _message }
    }
}

impl fmt::Display for FirstElementNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The first element was not found")
    }
}

impl ::std::error::Error for FirstElementNotFound {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn cause(&self) -> ::std::option::Option<&::std::error::Error> {
        None
    }
}
