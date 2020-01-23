//use crate::core::core::Value;

use std::fs::File;
use std::io::prelude::*;
//#[cfg(test)]
use std::path::Path;

#[cfg(test)]
use crate::core::core::SourceInfo;

//use super::core::{Error, RunnerError};
use super::core::{Error, RunnerError};
use super::super::core::ast::*;

impl Body {
    pub fn eval(self, context_dir: String) -> Result<Vec<u8>, Error> {
        return self.value.eval(context_dir);
    }
}

impl Bytes {
    pub fn eval(self, context_dir: String) -> Result<Vec<u8>, Error> {
        return match self {
            Bytes::MultilineString { value, .. } => Ok(value.into_bytes()),
            Bytes::Base64 { value, .. } => Ok(value),
            Bytes::Xml { value, .. } => Ok(value.into_bytes()),
            Bytes::Json { value, .. } => Ok(value.into_bytes()),
            Bytes::File { filename, .. } => {
                let path = Path::new(filename.value.as_str());
                let absolute_filename = if path.is_absolute() {
                    filename.clone().value
                } else {
                    Path::new(context_dir.as_str()).join(filename.value).to_str().unwrap().to_string()
                };
                match File::open(absolute_filename.clone()) {
                    Ok(f) => {
                        let mut bytes = vec![];
                        for byte in f.bytes() {
                            bytes.push(byte.unwrap());
                        }
                        Ok(bytes)
                    }
                    Err(_) => Err(Error {
                        source_info: filename.source_info,
                        inner: RunnerError::FileReadAccess { value: absolute_filename },
                        assert: false,
                    })
                }
            }
        };
    }
}


// region body-file
#[cfg(test)]
pub fn create_test_file() {
    let path = Path::new("/tmp/data.bin");
    let display = path.display();
    match File::open(path) {
        Err(_) => match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {:?}", display, why),
            Ok(mut file) => match file.write_all("Hello World!".as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {:?}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
        },
        _ => {}
    }
}

#[test]
pub fn test_body_file() {
    create_test_file();

    // file, data.bin;
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };

    let bytes = Bytes::File {
        space0: whitespace.clone(),
        filename: Filename { value: String::from("/tmp/data.bin"), source_info: SourceInfo::init(1, 7, 1, 15) },
        space1: whitespace.clone(),
    };

    assert_eq!(bytes.eval("current_dir".to_string()).unwrap(), "Hello World!".as_bytes());
}

#[test]
pub fn test_body_file_error() {
    // file, data.bin;
    let whitespace = Whitespace {
        value: String::from(" "),
        source_info: SourceInfo::init(0, 0, 0, 0),
    };

    let bytes = Bytes::File {
        space0: whitespace.clone(),
        filename: Filename { value: String::from("data.bin"), source_info: SourceInfo::init(1, 7, 1, 15) },
        space1: whitespace.clone(),
    };


    let error = bytes.eval("current_dir".to_string()).err().unwrap();
    assert_eq!(error.inner, RunnerError::FileReadAccess { value: String::from("current_dir/data.bin") });
    assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 15));
}

// endregion

