use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Result<String, String> {
    let mut full_path = "./res/".to_owned();
    full_path.push_str(path);

    File::open(full_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut content = String::new();

            file.read_to_string(&mut content)
                .map_err(|err| err.to_string())
                .map(|_| content)
        })
}
