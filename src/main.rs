use core::time;
use std::process::exit;
use std::thread::sleep;
use std::path::Path;
use std::env;

use wallpaper;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Invalid arguments");
        eprintln!("Usage: ./dimmer <dimmer_img_path> <process_string>");
        exit(1);
    }

    let path_dimmer = &args[1];
    let process_string = &args[2];

    // First, get the current background path so that we can reset it when this program ends
    // let cl = path_user_wp.clone();
    let path_user_wp = wallpaper::get().unwrap();

    // ctrl-c handler
    ctrlc::set_handler({
        let path_user_wp_clone = path_user_wp.clone();
        move || {
            println!("Resetting wallpaper...");
            wallpaper::set_from_path(&path_user_wp_clone).unwrap();
            exit(1);
        }
    }).unwrap();

    println!("User wallpaper: {}", path_user_wp);
    println!("new wallpaper: {}", path_dimmer);

    if !Path::new(&path_dimmer).exists() {
        panic!("Wallpaper not found ({})", path_dimmer);
    }

    // init system
    let mut system = sysinfo::System::new();

    let mut is_active = false;
    loop {
        system.refresh_processes();
        if system.processes_by_name(process_string).collect::<Vec<_>>().len() > 0 {
            if !is_active {
                wallpaper::set_from_path(&path_dimmer).unwrap();
                is_active = true;
                println!("Dimmer active");
            }
        }
        else {
            if is_active {
                wallpaper::set_from_path(&path_user_wp).unwrap();
                is_active = false;
                println!("Dimmer inactive");
            }
        }
        sleep(time::Duration::from_millis(1000));
    }

}
