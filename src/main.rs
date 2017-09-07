//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//

extern crate carma;
extern crate glium;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate fern;

use std::env;
use carma::support;
use carma::support::camera::CameraState;
use carma::support::car::Car;
use carma::support::render_manager::RenderManager;
use carma::support::texture::PixelMap;

fn setup_logging() -> Result<(), fern::InitError> {
    let base_config = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    });

    let stdout_config = fern::Dispatch::new()
        .level(log::LogLevelFilter::Info)
        .chain(std::io::stdout());

    let file_config = fern::Dispatch::new()
        .level(log::LogLevelFilter::Trace)
        .chain(std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // start log file anew each run
            .create(true)
            .open("debug.log")?);

    base_config.chain(stdout_config).chain(file_config).apply()?;

    Ok(())
}

use std::fs::File;
use std::io::BufWriter;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> Result<(), carma::support::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn convert_pixmap(fname: String, palette: &PixelMap) -> Result<(), carma::support::Error> {
    let pmap = PixelMap::load_from(fname.clone()).expect(
        format!(
            "Couldnt open pix file {:?}",
            fname
        ).as_ref(),
    );
    // let mut counter = 0;
    for pix in pmap {
        // counter += 1;
        let mut pngname = PathBuf::from(&fname);
        // let name = String::from(pngname.file_name().unwrap().to_str().unwrap());
        pngname.set_file_name(&pix.name);
        pngname.set_extension("png");

        info!("Creating file {:?}", pngname);
        let file = File::create(&pngname).expect(
            format!(
                "Couldnt create png file {:?}",
                pngname
            ).as_ref(),
        );
        let ref mut w = BufWriter::new(file);

        pix.write_png_remapped_via(&palette, w).expect(
            "Failed to write PNG",
        );
    }
    Ok(())
}

/// Uses different palette for race-selection part
fn convert_menu_pixmap(fname: String) -> Result<(), carma::support::Error> {
    let palette = &PixelMap::load_from(String::from("DecodedData/DATA/REG/PALETTES/DRACEFLC.PAL"))?
        [0];
    convert_pixmap(fname, palette)
}

fn convert_game_pixmap(fname: String) -> Result<(), carma::support::Error> {
    let palette = &PixelMap::load_from(String::from("DecodedData/DATA/REG/PALETTES/DRRENDER.PAL"))?
        [0];
    convert_pixmap(fname, palette)
}

/// Load palette once and then apply to a bunch of pixmap data
fn convert_all_pixmaps() -> Result<(), carma::support::Error> {
    let palette = &PixelMap::load_from(String::from("DecodedData/DATA/REG/PALETTES/DRRENDER.PAL"))?
        [0];
    visit_dirs(&Path::new("DecodedData"), &|dir_entry| {
        if let Ok(file_type) = dir_entry.file_type() {
            let fname = String::from(dir_entry.path().to_str().unwrap());
            if file_type.is_file() && fname.ends_with(".PIX") {
                convert_pixmap(fname, palette);
            }
        }
    })
}


fn main() {
    setup_logging().expect("failed to initialize logging");

    // convert_all_pixmaps().expect("Listing failed");
    convert_game_pixmap(String::from("DecodedData/DATA/PIXELMAP/EAGYELE.PIX"))
        .expect("Conversion failed");

    let car = Car::load_from(env::args().nth(1).unwrap()).unwrap();
    car.dump();

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("carma")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut render_manager = RenderManager::new(&display);
    render_manager.prepare_car(&car, &display);

    let mut camera = CameraState::new();

    support::start_loop(|| {
        camera.update();

        let mut target = display.draw();
        target.clear_color_and_depth((0.4, 0.4, 0.4, 0.0), 1.0);

        render_manager.draw_car(&car, &mut target, &camera);
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => action = support::Action::Stop,
                    _ => camera.process_input(&event),
                }
            }
            _ => (),
        });

        action
    });
}
