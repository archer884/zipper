use std::borrow::Cow;
use std::error;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::result;
use zip::write::FileOptions;
use zip::ZipWriter;

extern crate zip;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
struct Error {
    kind: ErrorKind,
    cause: Option<Box<error::Error>>,
    description: Cow<'static, str>
}

#[derive(Debug)]
enum ErrorKind {
    Usage,
    IO,
    Zip,
}

impl Error {
    fn usage() -> Self {
        Error {
            kind: ErrorKind::Usage,
            cause: None,
            description: Cow::from("Usage: zipper <zip file> <file to archive> <file to archive> ..."),
        }
    }

    fn io<D, E>(error: E, description: D) -> Self
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

    fn zip<E: error::Error + 'static>(error: E) -> Self {
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
            Some(ref cause) => Some(cause.as_ref())
        }
    }
}

fn main() {
    if let Err(e) = create_archive() {
        use std::error::Error;

        println!("{}", e.description());
        match e.kind {
            ErrorKind::Usage => std::process::exit(1),
            ErrorKind::IO => std::process::exit(2),
            ErrorKind::Zip => std::process::exit(3),
        }
    }
}

fn open_archive(path: &str) -> Result<ZipWriter<File>> {
    File::create(path)
        .map(|f| ZipWriter::new(f))
        .map_err(|e| Error::io(e, "Unable to create zip file"))
}

/// This function attempts to create an archive and returns a result indicating the success or 
/// failure of the operation.
/// 
/// The first command line argument is the path of the zip file to be created, and all other 
/// arguments are the paths of files to be added to the archive.
fn create_archive() -> Result<()> {
    use std::io;

    let mut args = std::env::args().skip(1);
    let mut archive = open_archive(&args.next().ok_or_else(Error::usage)?)?;

    let files = args.map(|path| {
        let path = PathBuf::from(path);
        let file = File::open(&path).map_err(|e| Error::io(e, "Unable to open source file"));
        (path, file)
    });

    for (path, file) in files {
        // I just want to return early here if there's anything wrong with the file.
        let mut file = file?;

        // We know this is valid UTF-8 because otherwise it wouldn't have been representable as a 
        // command line argument.
        let name = path.file_name().and_then(|s| s.to_str()).unwrap();
        archive.start_file(name, FileOptions::default()) .map_err(|e| Error::zip(e))?;
        
        // Honestly not sure if this should be an io error or a zip error.
        io::copy(&mut file, &mut archive)
            .map_err(|e| Error::io(e, "An error occurred while copying files"))?;
    }

    Ok({
        archive.finish().map_err(|e| Error::zip(e))?;
    })
}
