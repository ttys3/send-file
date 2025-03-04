use std::net::Ipv4Addr;

use tauri::Manager;

use crate::{
    utils::{self, system_info::SystemInformation, CommandData},
    SERVER_PORT,
};

//close the splash screen when this is called
#[tauri::command]
pub fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

// give connection details of the application core server
// get the ip address of the machine
#[tauri::command]
pub fn get_ip_address() -> String {
    let ip_address = utils::ip_manager::autodetect_ip_address()
        .ok()
        .expect("Invalid Ip address detected")
        .parse::<Ipv4Addr>()
        .unwrap();
    format!("{ip_address}:{port:?}", port = *SERVER_PORT)
}

/// get the system information
/// tis include
/// - the uptime
/// the free memory
/// the port on which the cor server run
#[tauri::command]
pub fn get_system_information() -> CommandData<SystemInformation> {
    CommandData::ok("connected system information ", SystemInformation::new())
}
