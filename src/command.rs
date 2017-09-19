use error::*;
use std::env;
use std::path::Path;
use std::iter::Map;
use std::slice::Iter;

pub(crate) struct CreateArchive {
    output_path: String,
    input_paths: Vec<String>,
}

impl CreateArchive {
    pub fn from_args() -> Result<Self> {
        let mut args = env::args().skip(1);

        let output_path = args.next().ok_or_else(|| Error::usage())?;
        let input_paths: Vec<_> = args.collect();

        if input_paths.len() == 0 {
            return Err(Error::usage());
        }

        Ok(CreateArchive { output_path, input_paths })
    }

    pub fn output(&self) -> &Path {
        self.output_path.as_ref()
    }

    pub fn inputs(&self) -> Map<Iter<String>, fn(&String) -> &Path> {
        fn convert(s: &String) -> &Path { s.as_ref() }
        self.input_paths.iter().map(|path| path.as_ref())
    }
}
