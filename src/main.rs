use std::{path::Path, process::Command};
use std::fs;

fn get_image_type() -> String {
    
    let image_type_file: Option<&Path>;

    if Path::new("/usr/share/horizon").exists() {
        image_type_file = Some(Path::new("/usr/share/horizon/image_type"));
    } else if Path::new("/usr/share/nova/image_type").exists() {
        image_type_file = Some(Path::new("/usr/share/nova/image_type"));
    } else if Path::new("/usr/share/umbra/image_type").exists() {
        image_type_file = Some(Path::new("/usr/share/umbra/image_type"));
    } else {
        // Return an error if no image type file is found
        panic!("\x1b[1;31mERROR:\x1b[0m No image type file found!");
    }

    let image = fs::read_to_string(image_type_file.unwrap()).expect("Failed to get image type");

    let image_name = image.trim().to_string();

    return image_name
}

fn systemd_service(service_name: &str) {
    let check_status = Command::new("systemctl")
        .args(["get-default"])
        .output()
        .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");
        

    let target_status = String::from_utf8_lossy(&check_status.stdout);

    if target_status == "graphical.target" {
        return;
    } else {
        let _set_graphical = Command::new("systemctl") 
            .args(["set-default", "graphical.target"])
            .status()
            .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");
            
        let _reboot = Command::new("systemctl")
            .args(["reboot"])
            .status()
            .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");
    }
}

fn main() {
    let image_type = get_image_type();
  
    if image_type.contains("nova") {
        systemd_service("nova-first-boot.service");
    } else if image_type == "plasma" {
        systemd_service("umbra-first-boot.service");
    };
}
