use std::{
    thread, time::Duration
};
use windows::{
    core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*
};

fn main() {
    loop {
        let _ = unsafe { EnumWindows(Some(check), LPARAM(0)) };
        thread::sleep(Duration::from_millis(100));
    }
}

unsafe extern "system" fn check(hwnd: HWND, _: LPARAM) -> BOOL {
    let cwnd = FindWindowExA(hwnd, HWND(0), s!("BannerAdContainer"), PCSTR::null());

    if cwnd != HWND(0) {
        hide(hwnd);
        return FALSE;
    }

    TRUE
}

fn hide(ad_hwnd: HWND) {
    let title = w!("카카오톡");
    
    let mut hwnd = HWND(0);
    let mut frame = RECT::default();

    let handle = unsafe { FindWindowW(PCWSTR::null(), title) };
    let _ = unsafe { GetWindowRect(handle, &mut frame) };
    let kt_size = frame.bottom - frame.top;

    let _ = unsafe { GetWindowRect(ad_hwnd, &mut frame) };
    let ad_size = frame.bottom - frame.top;

    let _ = unsafe { ShowWindow(ad_hwnd, SW_HIDE) };

    loop {
        hwnd = unsafe { FindWindowExA(handle, hwnd, PCSTR::null(), PCSTR::null()) };

        if hwnd == HWND(0) {
            break;
        }

        let size = unsafe { GetWindowTextLengthW(hwnd) } as usize;
        let mut caption_vec = vec![0 as u16; size];
        let _ = unsafe { GetWindowTextW(hwnd, &mut caption_vec) };
        let caption = String::from_utf16(&mut caption_vec).unwrap();

        if caption.starts_with("OnlineMainView") || caption.starts_with("LockModeView") {
            let _ = unsafe { GetWindowRect(hwnd, &mut frame) };
            let wn_size = frame.bottom - frame.top;

            if wn_size != 0 && kt_size > wn_size + ad_size {
                let _ = unsafe { SetWindowPos(hwnd, HWND(0), 0, 0, frame.right - frame.left, wn_size + ad_size + 1, SWP_NOZORDER | SWP_NOMOVE) };
            }
        }
    }
}