#![windows_subsystem = "windows"]

use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_E, VK_SHIFT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetSystemMetrics, GetWindowRect, SetWindowPos, HWND_TOP,
    SET_WINDOW_POS_FLAGS, SYSTEM_METRICS_INDEX,
};

unsafe fn center_window() {
    let hwnd: HWND = GetForegroundWindow();
    if hwnd.0 == null_mut() {
        return;
    }

    let mut rect: RECT = RECT::default();
    if GetWindowRect(hwnd, &mut rect).is_err() {
        return;
    }

    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;

    let screen_width = GetSystemMetrics(SYSTEM_METRICS_INDEX(0));
    let screen_height = GetSystemMetrics(SYSTEM_METRICS_INDEX(1));

    let x = (screen_width - width) / 2;
    let y = (screen_height - height) / 2;

    SetWindowPos(hwnd, HWND_TOP, x, y, 0, 0, SET_WINDOW_POS_FLAGS(0x0001))
        .expect("TODO: panic message");
}

fn main() {
    println!("Console");
    loop {
        let ctrl_pressed = unsafe { GetAsyncKeyState(VK_CONTROL.0 as i32) } < 0;
        let shift_pressed = unsafe { GetAsyncKeyState(VK_SHIFT.0 as i32) } < 0;
        let e_pressed = unsafe { GetAsyncKeyState(VK_E.0 as i32) } < 0;

        if ctrl_pressed && shift_pressed && e_pressed {
            unsafe { center_window() }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
