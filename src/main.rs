#![feature(conservative_impl_trait)]

extern crate zip;

mod command;
mod error;

use command::CreateArchive;
use error::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use zip::ZipWriter;

fn main() {
    if let Err(e) = CreateArchive::from_args().map(|command| create_archive(command)) {
        use std::error::Error;

        println!("{}", e.description());

        // Apparently, enum variants are naturally isize, but they should be safe to 
        // cast as i32, at least in any program I have ever written. >.>
        std::process::exit(e.kind as i32);
    }
}

fn open_archive<P: AsRef<Path>>(path: P) -> Result<ZipWriter<File>> {
    File::create(path).map(|f| ZipWriter::new(f)).map_err(|e| {
        Error::io(e, "Unable to create zip file")
    })
}

/// This function attempts to create an archive and returns a result indicating the success or
/// failure of the operation.
///
/// The first command line argument is the path of the zip file to be created, and all other
/// arguments are the paths of files to be added to the archive.
fn create_archive(command: CreateArchive) -> Result<()> {
    use std::io;

    let mut archive = open_archive(command.output())?;
    

    // Draining the tail of the vector like this allows me to take ownership of the contents of 
    // the vector without cloning anything.
    let files = args.drain(1..).map(|path| {
        
        // We need to take ownership of the string we get from the args iterator, otherwise it
        // will fall out of scope before we can use it. Hence we create a path buffer rather than
        // a path. Normally, I would just use the bare string the way I did in open_archive up
        // above, but later on you want the filename, so this seems worthwhile.
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

        // This formatting looks weird, but it's what rustfmt picks for me. /shrug
        //
        // I can usually take or leave rustfmt. We agree on most things, but then it does stuff 
        // like this. >.>
        archive.start_file(name, FileOptions::default()).map_err(
            |e| {
                Error::zip(e)
            },
        )?;

        // Honestly not sure if this should be an io error or a zip error. It's possible that
        // it could actually spit out an io error wrapping an error from the compression library,
        // in which case it seems like my bases are covered, but it's not like I've actually seen
        // this fail.
        io::copy(&mut file, &mut archive).map_err(|e| {
            Error::io(e, "An error occurred while copying files")
        })?;
    }

    // I do not know if this is considered "ok" style or not, but I like to do this to strongly
    // associate the ? op with the Ok(()) return value, because the one precludes the other.
    Ok({
        archive.finish().map_err(|e| Error::zip(e))?;
    })
}
