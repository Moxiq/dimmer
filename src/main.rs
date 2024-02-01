use core::time;
use std::thread::sleep;
use std::path::Path;
use std::env;

use wallpaper;

fn get_processes(system: &mut sysinfo::System) {
    system.refresh_processes();

    for (pid, process) in system.processes() {
        println!("{}:{}", pid, process.name());
    }
}

fn main() {
    // First, get the current background path so that we can reset it when this program ends
    let path_user_wp = wallpaper::get().unwrap();
    let path_cur = env::current_dir().unwrap();
    let path_new_wp = format!("{}/black.jpg", path_cur.display());

    println!("User wallpaper: {}", path_user_wp);
    println!("new wallpaper: {}", path_new_wp);

    if !Path::new(&path_new_wp).exists() {
        panic!("Wallpaper not found ({})", path_new_wp);
    }

    // init system
    let mut system = sysinfo::System::new();
    let mut is_active = false;

    loop {
        system.refresh_processes();
        if system.processes_by_name("League of").collect::<Vec<_>>().len() > 0 {
            if !is_active {
                wallpaper::set_from_path(&path_new_wp).unwrap();
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
        sleep(time::Duration::from_secs(2));
    }

}
