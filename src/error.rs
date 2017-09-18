use std::borrow::Cow;
use std::error;
use std::fmt;
use std::result;

// This result definition is pretty much standard practice. In my programs, I often use a 
// type definition for `Cause`, too, which is just `Box<Error>`. This is because otherwise I 
// need to write out `Box<error::Error>`, but I figured in thise case I'd just leave it.
pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    cause: Option<Box<error::Error>>,

    // Using a copy-on-write string in this case enables me to get away a little more cheaply
    // in the (usual) case wherein there is no need for a heap-allocated string. In the event 
    // I do need some kind of special content for the description (e.g. I want to include some 
    // kind of user input or something), I can do that without changing the type.
    description: Cow<'static, str>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Usage = 1,
    IO = 2,
    Zip = 3,
}

// These constructors are for my convenience, so I've marked them as public within the crate.
// At least, I think that's what I've done. This whole crate visibility thing is new to me.
impl Error {
    pub(crate) fn usage() -> Self {
        Error {
            kind: ErrorKind::Usage,
            cause: None,
            description: Cow::from(
                "Usage: zipper <zip file> <file to archive> <file to archive> ...",
            ),
        }
    }

    pub(crate) fn io<D, E>(error: E, description: D) -> Self
    where
        D: Into<Cow<'static, str>>,
        E: error::Error + 'static,
    {
        Error {
            kind: ErrorKind::IO,
            cause: Some(Box::new(error)),
            description: description.into(),
        }
    }

    pub(crate) fn zip<E>(error: E) -> Self
    where
        E: error::Error + 'static
    {
        Error {
            kind: ErrorKind::Zip,
            cause: Some(Box::new(error)),
            description: Cow::from("An error occurred creating the archive"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.cause {
            None => None,
            Some(ref cause) => Some(cause.as_ref()),
        }
    }
}
