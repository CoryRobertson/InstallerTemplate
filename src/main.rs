use std::fs;
use std::io;

fn main() {

    let file_name = String::from("./test.zip");

    let output_directory = "./test/";

    let file = fs::File::open(&file_name).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // directory output modification
        // println!("{:?}", outpath);
        let dirs = outpath.parent().unwrap();
        outpath = dirs.join(output_directory).join(file.enclosed_name().unwrap()); // ty stack overflow <3
        // println!("{:?}", outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("{}, {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

    }
}
