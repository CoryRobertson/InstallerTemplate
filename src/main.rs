use std::fs;
use std::io;
use eframe::egui;
use egui::Vec2;

fn main() {

    let mut options = eframe::NativeOptions::default();

    options.initial_window_size = Option::from(Vec2::new(300 as f32,300 as f32));

    eframe::run_native(
        "installer template",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );




}

fn extract() {
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

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Test".to_owned(),
            age: 111,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.heading("Installer Template!");
            if ui.button("Click to extract").clicked() {
                extract();
            }
        });
    }
}