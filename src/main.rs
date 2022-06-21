use std::borrow::Borrow;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use eframe::egui;
use egui::{Response, TextBuffer, Vec2};
use tokio::runtime::Runtime;

fn main() {

    let mut options = eframe::NativeOptions::default();

    options.initial_window_size = Option::from(Vec2::new(300 as f32,300 as f32));

    eframe::run_native(
        "installer template",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );

}

// this function downloads a fixed url, for the purpose of hard coding the location to get the file from.
fn download(output_name: &str) {

    let url = String::from("https://github.com/CoryRobertson/ThumbnailExtractor/releases/download/v1.1/ThumbnailExtractor-1.0-SNAPSHOT.zip");
    // let save_file_path = "temp.zip";
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.bytes().expect("body invalid");
    // let mut out = File::create(output_name).expect("failed to create file");

    let mut out = File::create(output_name).expect("failed to create file");
    let mut body_bytes= body.to_vec();
    io::copy(&mut &body_bytes[..], &mut out).expect("failed to copy content");
}


// this function downloads a file off the internet and saves it as a given name
fn download_with_url(url: &str, output_name: &str) {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.bytes().expect("body invalid");
    let mut out = File::create(output_name).expect("failed to create file");
    let mut body_bytes = body.to_vec();
    io::copy(&mut &body_bytes[..], &mut out).expect("failed to copy content");
}

// this function extracts given filename of a zip file to a specific output directory
fn extract(file_name: &str, output_directory: &str) {
    // let file_name = String::from("./test.zip");

    // let output_directory = "./test/";

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
        // outpath = dirs.join(output_directory).join(file.enclosed_name().unwrap()); // ty stack overflow <3
        outpath = PathBuf::from(output_directory).join(file.enclosed_name().unwrap()); // ty stack overflow <3
        println!("{:?}", outpath);

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

struct MyApp {
    path: PathBuf,
    path_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // default paths for file to extract to
            path: PathBuf::from("./test/"),
            path_text: String::from("./test/"),
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.path_text);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.heading("Installer Template!");




            if ui.button("Click to download").clicked() {

                download("./test2.zip");

            }

            if ui.button("Click to extract").clicked() {
                match File::open("./test2.zip") {
                    Ok(_) => {
                        extract("./test2.zip", &self.path_text);
                        fs::remove_file("./test2.zip").expect("Unable to delete zip file");
                    }
                    Err(e) => {println!("Zip file not found. {}", e.to_string());}
                };

            }

            // let btn1 = ui.button("test");
            //
            // if btn1.clicked() {
            //     println!("{}", self.path.display());
            // }

            ui.horizontal(|ui| {
                ui.label("Path to install to: ");
                let path_bar = ui.text_edit_singleline(&mut self.path_text);

                let new_path_check = match PathBuf::from(&self.path_text).canonicalize() {
                    Ok(p) => {
                        // self.path = PathBuf::from(&self.path_text);
                        p
                    },
                    Err(e) => {
                        // println!("{}", e.to_string())
                        PathBuf::from("./")
                    },
                };
                if path_bar.changed() {
                    self.path = new_path_check;
                    // println!("path bar changed to: {}", &self.path_text);
                }
                // set the path to the new path written in by the user
            });

            if ui.button("Print path to console").clicked() {
                match fs::create_dir_all(PathBuf::from(&self.path_text)) {
                    Ok(a) => {a}
                    Err(e) => {
                        eprintln!("{}", e.to_string());

                    }
                };
                println!("{}", self.path.display());

            }


        });
    }
}