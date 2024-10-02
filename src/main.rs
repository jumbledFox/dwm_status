use std::{error::Error, ffi::CString, fs, ptr, thread, time::{Duration, Instant}};

use chrono::Local;
use x11::xlib;

const BATTERY_BARS: usize = 10;

fn main() {
    let display = unsafe { xlib::XOpenDisplay(ptr::null())};
    if display.is_null() {
        eprintln!("Unable to open the display!!");
        return;
    }

    // Get root window of default screen
    let screen_num = unsafe { xlib::XDefaultScreen(display) };
    let root_window = unsafe { xlib::XRootWindow(display, screen_num) };

    let mut x: usize = 0;
    let mut d: isize = 1;
    loop {
        let mut frame = [' '; 100];
        if x == 0 && d == -1 {
            d = 1;
        }
        if x == 99 && d == 1 {
            d = -1;
        }
        x = x.saturating_add_signed(d);
        frame[x] = '󰄛';

        let current_time = Local::now().format("%d/%m/%y   %H:%M:%S").to_string();
        let battery = match battery_info() {
            Ok(b) => format!("   {b}"),
            Err(_) => String::new(),
        };

        // Change the root window name
        let new_name = format!("{}| jumbledFox :3   {}{}", String::from_iter(frame), current_time, battery);
        // let new_name = format!("jumbledFox :3   {}{}", current_time, battery);
        let new_name_c = CString::new(new_name).expect("Failed to make CString!!!");

        unsafe {
            xlib::XStoreName(display, root_window, new_name_c.as_ptr());
            xlib::XFlush(display);
        }

        // Wait until the next second
        // let millis_until_next_second = 1000 - now.timestamp_subsec_millis() as u64;
        // thread::sleep(Duration::from_millis(millis_until_next_second));
        thread::sleep(Duration::from_millis(17));
    }
}

fn battery_info() -> Result<String, Box<dyn Error>> {
    let capacity_path = "/sys/class/power_supply/BAT0/capacity";
    let status_path = "/sys/class/power_supply/BAT0/status";

    let percent_string = fs::read_to_string(capacity_path)?;
    let percent: usize = percent_string.trim().parse()?;
    
    let full_count = (percent as f32 / BATTERY_BARS as f32).round() as usize;
    let visual = format!("{}{}", "▉".repeat(full_count), " ".repeat(BATTERY_BARS.saturating_sub(full_count)));

    let status = fs::read_to_string(status_path)?;
    let status = status.trim();

    Ok(format!("[{}] {:?}% {}", visual, percent, status))
}