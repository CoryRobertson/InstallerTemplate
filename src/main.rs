use std::fs;
use std::fs::File;
use std::io;
use std::path::{PathBuf};
use eframe::egui;
use egui::{Color32, Pos2, Vec2};

fn main() {

    let mut options = eframe::NativeOptions::default();

    options.initial_window_size = Option::from(Vec2::new(300 as f32,300 as f32));

    options.resizable = false;

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
    let body_bytes= body.to_vec();
    io::copy(&mut &body_bytes[..], &mut out).expect("failed to copy content");
}


// this function downloads a file off the internet and saves it as a given name
fn _download_with_url(url: &str, output_name: &str) {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.bytes().expect("body invalid");
    let mut out = File::create(output_name).expect("failed to create file");
    let body_bytes = body.to_vec();
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
        let _dirs = outpath.parent().unwrap();
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
    frames: u128,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // default paths for file to extract to
            path: PathBuf::from("./test/"),
            path_text: String::from("./test/"),
            frames: 0,
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {


            ui.heading("Installer Template!");


            // let time = std::time::SystemTime::now();
            // let since = time.duration_since(UNIX_EPOCH).unwrap();
            //
            // let x = since.as_millis() % 300;

            // circle animation thingy :)
            let x: f32 = ((self.frames as f32 / 100.0).sin() * 100.0) + 150.0;
            let y: f32 = ((self.frames as f32 / 100.0).cos() * 100.0) + 150.0;

            // various prime number divisions so we get slightly uneven changes :) felt pretty smart about this one even though its not very good
            let color_x: f32 = ((self.frames as f32 /101.0).sin() * 255.0).abs();
            let color_y: f32 = ((self.frames as f32 /157.0).cos() * 255.0).abs();
            let color_z: f32 = ((self.frames as f32 /197.0).tan() * 255.0).abs();
            let radius: f32 = ((self.frames as f32 / 293.0).sin() * 50.0).abs();

            ui.painter().circle_filled(Pos2::new(x, y), radius, Color32::from_rgb(color_x as u8, color_y as u8, color_z as u8));
            ctx.request_repaint(); // refresh ui on every chance possible so we can show the sick animation :)
            println!("r{} g{} b{}", color_x as u8, color_y as u8, color_z as u8);

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
                    Err(_) => {
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
            self.frames += 1;

        });
    }
}