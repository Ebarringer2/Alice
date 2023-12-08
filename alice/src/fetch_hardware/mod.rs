use sysinfo::{System, SystemExt, ComponentExt};

pub fn fetch_hardware() {
    let mut system = System::new_all();
    let s: String = "N/A".to_string();
    system.refresh_all();
    println!("OS: {}", system.long_os_version().unwrap_or(s));
    println!("Kernel version: {}", system.kernel_version().unwrap());
    for component in system.components() {
        println!("Component: {}", component.label());
        println!("  Temperature: {}°C", component.temperature());
        //println!("  Critical Temperature: {}°C", component.critical());
        println!("  Maximum Temperature usage: {}W", component.max());
        println!();
    }
}
