
#[cfg(test)]
mod tests {
    use crate::download_with_url;
    use crate::install_program_on_thread;
    use std::fs;
    use std::fs::File;
    // use super::*;

    #[test]
    fn test_download_with_url() {

        download_with_url("https://github.com/CoryRobertson/ThumbnailExtractor/releases/download/v1.1/ThumbnailExtractor-1.0-SNAPSHOT.zip","./test.zip");

        match File::open("./test.zip") {
            Ok(f) => {f}
            Err(_) => {panic!("Failed to download file with url.");}
        };

        match fs::remove_file("./test.zip") {
            Ok(r) => {r}
            Err(_) => {panic!("Failed to remove zip file.")}
        };

        assert!(true);
    }

    #[test]
    fn test_install_program_on_thread() {
        let path_test = "./test/a/".to_string();
        let t1 = install_program_on_thread(&path_test);

        while !t1.is_finished() {

        }

        match fs::read_dir("./test/a/") {
            Ok(_) => {}
            Err(_) => {panic!("directory does not exist.")}
        };

        match fs::remove_dir_all("./test/a/") {
            Ok(_) => {}
            Err(_) => {panic!("unable to remove directory");}
        };

        assert!(true);
    }

}