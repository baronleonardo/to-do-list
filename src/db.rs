pub mod db {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;

    pub fn read(file_path: &str) -> String {
        let mut buf = String::new();

        if Path::new(file_path).exists() {
            let mut file = File::open(&file_path).unwrap();
            file.read_to_string(&mut buf).unwrap();
        }

        buf
    }

    pub fn write(file_path: &str, text: &String) {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }
}