use winapi::{
    user32::{ChangeDisplaySettingsEx, ENUMDISPLAYSETTINGS, MONITORINFOEX, MONITOR_DEFAULTTOPRIMARY},
    wtypes::HDC,
};

use std::{
    env::args,
    fs::File,
    io::prelude::*,
    path::Path,
};

fn main() {
    // Get the primary monitor.
    let mut monitor_info = MONITORINFOEX {
        cbSize: std::mem::size_of::<MONITORINFOEX>() as u32,
        rcMonitor: RECT {
            left: 0,
            top: 0,
            bottom: 0,
            right: 0,
        },
        dwFlags: MONITORINFOF_PRIMARY,
    };
    let monitor_handle = GetMonitorInfo(MONITOR_DEFAULTTOPRIMARY, &mut monitor_info);

    // Get the current display settings.
    let mut display_settings = ENUMDISPLAYSETTINGS {
        cbSize: std::mem::size_of::<ENUMDISPLAYSETTINGS>() as u32,
        DeviceName: [0; 32],
        DeviceString: [0; 128],
        Flags: 0,
        Mode: 0,
        Width: 0,
        Height: 0,
        ColorDepth: 0,
        RefreshRate: 0,
    };
    let mut i = 0;
    while EnumDisplaySettingsEx(
        monitor_handle,
        i,
        &mut display_settings,
        ENUM_CURRENT_SETTINGS,
    ) != 0
    {
        i += 1;
    }

    // Create a dropdown menu.
    let mut screenshots: Vec<String> = Vec::new();
    for screenshot in args().skip(1) {
        let path = Path::new(&screenshot);
        if path.is_file() {
            let mut file = File::open(&path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            screenshots.push(String::from_utf8(buffer).unwrap());
        }
    }
    let mut selected_screenshot = 0;
    loop {
        println!("Select a screenshot:");
        for (i, screenshot) in screenshots.iter().enumerate() {
            println!("{}: {}", i, screenshot);
        }
        let input = std::io::stdin().read_line().unwrap();
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let selected_screenshot = input.parse::<usize>().unwrap();
        if selected_screenshot < screenshots.len() {
            break;
        }
    }

    // Change the screen resolution to the selected screenshot.
    let new_width = screenshots[selected_screenshot].split('x').next().unwrap().parse::<u32>().unwrap();
    let new_height = screenshots[selected_screenshot].split('x').last().unwrap().parse::<u32>().unwrap();
    let new_refresh_rate = 60;
    if ChangeDisplaySettingsEx(
        monitor_handle,
        &display_settings,
        ENUM_CHANGE_SETTINGS,
        &mut monitor_info,
    ) == 0
    {
        println!("Error changing screen resolution: {}", GetLastError());
        return;
    }

    println!("Screen resolution changed to {}x{}", new_width, new_height);
}
