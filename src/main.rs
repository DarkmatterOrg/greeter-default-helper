use std::fs;
use std::{path::Path, process::Command};

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

    return image_name;
}

fn systemd_default() -> String {
    let check_status = Command::new("systemctl")
        .args(["get-default"])
        .output()
        .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");

    let target_status = String::from_utf8_lossy(&check_status.stdout);

    let status = target_status.trim();

    return status.to_string();
}

fn systemd_service(service_name: &str) {
    let status = systemd_default();

    if status.contains("graphical.target") {
        println!("Target already set to graphical. Exiting");

        let _disable_service = Command::new("systemctl")
            .args(["disable", service_name])
            .status()
            .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");

        return;
    } else {
        let _set_graphical = Command::new("systemctl")
            .args(["set-default", "graphical.target"])
            .status()
            .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");

        let _disable_service = Command::new("systemctl")
            .args(["disable", service_name])
            .status()
            .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");
    }
}

fn main() {
    /*if image_type.contains("nova") {
        systemd_service("nova-first-boot.service");
    } else if image_type.contains("umbra") {
        systemd_service("umbra-first-boot.service");
    };*/

    let _change_target = systemd_service("greeter-default.service");

    let _reboot = Command::new("systemctl")
        .args(["reboot"])
        .status()
        .expect("An error occured. Guess, I'm gonna have a panic attack now! :(");
}
