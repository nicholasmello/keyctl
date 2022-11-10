extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::{fs, io::Error, io::Write, io, env};

const PATH: &str = "/sys/devices/platform/tuxedo_keyboard";

fn main() {
    let mut read = false;
    let mut color = "".to_string();
    let mut brightness = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Change the backlight color of clevo keyboards");

        // read
        ap.refer(&mut read)
            .add_option(&["-r", "--read"], StoreTrue, "Returns the current color and brightness values");
        // color
        ap.refer(&mut color)
            .add_option(&["-c", "--color"], Store, "Set background color of the keyboard");
        // brightness
        ap.refer(&mut brightness)
            .add_option(&["-b", "--brightness"], Store, "Set brightness of the background of the keyboard");
        ap.parse_args_or_exit();
        // Nothing is passed in
        if env::args().len() < 2 {
            let args: Vec<String> = env::args().collect();
            ap.print_help(&args[0][..], &mut io::stdout()).unwrap();
        }
    }
    
    if color != "" {
        let res = write_file(&color, "color_left");
        match res {
            Ok(_) => println!("Color set to {}", color),
            Err(e) => println!("Set color failed with error message: {}", e)
        }
    }

    if brightness != "" {
        let res = write_file(&brightness, "brightness");
        match res {
            Ok(_) => println!("Brightness set to {}", brightness),
            Err(e) => println!("Set brightness failed with error message: {}", e)
        }
    }

    if read {
        println!("Read Results");
        println!("============");
        let res = get_color();
        match res {
            Ok(v) => print!("Color: \t\t0x{}", v),
            Err(_) => println!("Color file not found, check clevo keyboard drivers are installed")
        }
        let res = get_brightness();
        match res {
            Ok(v) => print!("Brightness: \t{}", v),
            Err(_) => println!("Brightness file not found, check clevo keyboard drivers are installed")
        }
    }
}

// fn write_color(red: u8, green: u8, blue: u8) {
//     let mut file = fs::OpenOptions::new()
//         .read(false)
//         .write(true)
//         .create(true)
//         .open(format!("{}/color_left", PATH)).expect("Failed to set color");
//     let color = format!("0x{:02x}{:02x}{:02x}", red, green, blue);
//     file.write_all(color.as_bytes()).expect("Failed to set color");
// }

fn write_file(text: &String, file_name: &str) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(format!("{}/{}", PATH, file_name))?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn get_color() -> Result<String, Error> {
    fs::read_to_string(format!("{}/color_left", PATH))
}

fn get_brightness() -> Result<String, Error> {
    fs::read_to_string(format!("{}/brightness", PATH))
}