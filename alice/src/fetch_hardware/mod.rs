use sysinfo::{System, SystemExt};

pub fn fetch_hardware() {
    let mut system = System::new_all();
    system.refresh_all();
    println!("OS: {}", system.get_os_version().unwrap_or("N/A"));
    println!("Kernel version: {}", system.get_kernel_version().unwrap_or("N/A"));
    for (component_name, component) in system.get_components() {
        println!("Component: {}", component_name);
        println!("  Temperature: {}°C", component.get_temperature().unwrap_or(0.0));
        println!("  Voltage: {}V", component.get_voltage().unwrap_or(0.0));
        println!("  Power usage: {}W", component.get_power_usage().unwrap_or(0.0));
        println!();
    }
}
