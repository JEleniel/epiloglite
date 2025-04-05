mod logging;
mod parser;
mod traits;

pub fn add() -> u64 {
    2 + 2
}

#[cfg(test)]
mod tests {
    use crate::{logging::init_logging, parser::parse};
    use log::info;
    use std::{fs::File, io::Read, path::PathBuf};

    #[test]
    fn test_parser() {
        init_logging();

        let test_data_dir = PathBuf::from("./test_data");
        for file in test_data_dir.read_dir().unwrap() {
            let f = file.unwrap();
            if !f.file_type().unwrap().is_dir() {
                info!("Running test file {}", f.path().to_string_lossy());

                let program: &mut String = &mut String::new();
                File::open(f.path())
                    .unwrap()
                    .read_to_string(program)
                    .unwrap();
                parse(&program);
            }
        }
    }
}
