use std::io::prelude::*;
use zip::write::FileOptions;
use std::fs::File;

extern crate zip;

fn main() {
    let prog_return = create_archive();
    match prog_return {
        Ok(f) => f,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);

        }
    }

    //Probably not necessary, but we want the
    //process to explicitly return 0 on success
    std::process::exit(0);
}

/*
 * This function creates a zip archive, and returns a Result
 * that indicates if all operations were successful.
 *
 * The first command-line argument is the name and path of the zip file to be created,
 * and all the other arguments are the names and paths of the files to be archived
 *
 */
fn create_archive() -> Result<(), String> {
    //If there are less than 3 command-line arguments, return an error
    if std::env::args().count() < 3 {
        return Err("Usage: zipper <zip file> <file to archive> <file to archive> ...".to_string());
    }

    //The 1st command line argument is the name of the zip file
    let zipfile_name = match std::env::args().nth(1) {
        Some(c) => c,
        None => return Err("Invalid argument for zip file.".to_string()),
    };


    let path_to_zipfile = std::path::Path::new(&zipfile_name);
    let zipfile = match std::fs::File::create(&path_to_zipfile) {
        Err(_) => {
            return Err(format!("Could not create zip file: {}. Check that the file path exists.",
                               zipfile_name))
        }
        Ok(f) => f,
    };
    let mut zip = zip::ZipWriter::new(zipfile);


    //Iterate over the command-line arguments for the names of the
    //files we wish to archive. Skip the first two arguments.
    for arg in std::env::args().skip(2) {
        let filename = &arg;

        //Put this in it's own scope so that the memory for the contents of
        //each file will be freed on each iteration
        {
            //Extract the raw filename out of the full path entered by the user
            //This returns the file name as a type &OsStr slice instead of a &str slice
            let name_of_file = match std::path::Path::new(filename).file_name() {
                Some(c) => c,
                None => return Err(format!("{}: Invalid file name", filename)),
            };

            //Redefine and convert the &OsStr slice to a standard &str slice.
            //Checks if the filename is valid unicode.
            let name_of_file = match name_of_file.to_str() {
                Some(c) => c,
                None => return Err(format!("{}: Invalid file name", filename)),
            };

            //Open the file and check for errors
            let mut f = match File::open(filename) {
                Err(e) => return Err(format!("{}: {}", filename, e)),
                Ok(f) => f,
            };

            //Get the file metadata so we can get the file size. Also check for errors
            let file_metadata = match f.metadata() {
                Err(e) => return Err(format!("{}: {}", filename, e)),
                Ok(f)  => f,
            };

            //Get the exact size of the file from the file metadata
            let file_size = file_metadata.len() as usize;

            //Create a buffer to store the file contents that is exactly the size of the file + 1
            let mut contents: Vec<u8> = Vec::with_capacity(file_size + 1);


            //Read the file and check for errors
            try!(f.read_to_end(&mut contents).map_err(|e| e.to_string()));

            //Now compress the file and write it into the zip archive
            try!(zip.start_file(name_of_file, FileOptions::default())
                     .map_err(|e| e.to_string()));
            try!(zip.write_all(contents.as_slice())
                     .map_err(|e| e.to_string()));

        }

    }

    //Finish the zip file
    try!(zip.finish().map_err(|e| e.to_string()));


    //Return an empty Ok tuple because we had no errors.
    return Ok(());
}
