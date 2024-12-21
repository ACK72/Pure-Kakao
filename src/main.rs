#![windows_subsystem = "windows"]

use std::{
	thread, time::Duration
};
use windows::{
	core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::UI::WindowsAndMessaging::*
};

fn main() {
	let title = w!("카카오톡");
	
	loop {
		let pwnd = unsafe { FindWindowW(PCWSTR::null(), title) };
		
		if pwnd == HWND(0) {
			thread::sleep(Duration::from_secs(1));
			continue;
		}

		_ = unsafe { EnumWindows(Some(check), LPARAM(pwnd.0)) };
		thread::sleep(Duration::from_millis(15));
	}
}

unsafe extern "system" fn check(hwnd: HWND, param: LPARAM) -> BOOL {
	let pwnd = HWND(param.0);
	
	if pwnd == GetParent(hwnd) {
		let cwnd = GetWindow(hwnd, GW_CHILD);
		let nwnd = GetWindow(hwnd, GW_HWNDNEXT);

		if cwnd != HWND(0) && nwnd == pwnd {
			hide(hwnd, pwnd);
			return FALSE;
		}
	}

	TRUE
}

fn hide(hwnd: HWND, pwnd: HWND) {
	let mut cwnd = HWND(0);
	let mut frame = RECT::default();

	_ = unsafe { GetWindowRect(pwnd, &mut frame) };
	let kt_size = frame.bottom - frame.top;

	_ = unsafe { GetWindowRect(hwnd, &mut frame) };
	let ad_size = frame.bottom - frame.top;

	_ = unsafe { ShowWindow(hwnd, SW_HIDE) };

	loop {
		cwnd = unsafe { FindWindowExA(pwnd, cwnd, PCSTR::null(), PCSTR::null()) };

		if cwnd == HWND(0) {
			break;
		}

		let size = unsafe { GetWindowTextLengthW(cwnd) } as usize;
		let mut caption_vec = vec![0 as u16; size];
		_ = unsafe { GetWindowTextW(cwnd, &mut caption_vec) };
		let caption = String::from_utf16(&mut caption_vec).unwrap();

		if caption.starts_with("OnlineMainView") || caption.starts_with("LockModeView") {
			_ = unsafe { GetWindowRect(cwnd, &mut frame) };
			let wn_size = frame.bottom - frame.top;

			if wn_size != 0 && kt_size > wn_size + ad_size {
				_ = unsafe { SetWindowPos(cwnd, HWND(0), 0, 0, frame.right - frame.left, wn_size + ad_size + 1, SWP_NOZORDER | SWP_NOMOVE) };
			}

			_ = unsafe { RedrawWindow(cwnd, None, HRGN(0), RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN) };
		}
	}
}