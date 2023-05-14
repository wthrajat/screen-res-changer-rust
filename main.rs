use std::io;
use std::mem;
use winapi::shared::minwindef::{BOOL, FALSE};
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::{POINT, RECT};
use winapi::um::winuser::{EnumDisplaySettingsExW, ChangeDisplaySettingsExW, DEVMODEW, DISP_CHANGE_SUCCESSFUL};

fn main() -> io::Result<()> {
    let mut dev_mode: DEVMODEW = unsafe { mem::zeroed() };
    dev_mode.dmSize = mem::size_of::<DEVMODEW>() as u16;

    // Prompt the user to enter the desired screen resolution
    println!("Enter the desired screen resolution:");
    println!("Width:");
    let width = read_user_input()?;
    dev_mode.dmPelsWidth = width;

    println!("Height:");
    let height = read_user_input()?;
    dev_mode.dmPelsHeight = height;

    unsafe {
        let result = ChangeDisplaySettingsExW(NULL, &mut dev_mode, NULL, 0, NULL);
        if result == DISP_CHANGE_SUCCESSFUL {
            println!("Screen resolution changed successfully.");
        } else {
            println!("Failed to change screen resolution.");
        }
    }

    Ok(())
}

fn read_user_input() -> io::Result<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse().map_err(|_| io::ErrorKind::InvalidInput.into())
}
