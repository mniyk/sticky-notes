pub const SIDEBAR_W: u32 = 220;
pub const EDITOR_W:  u32 = 800;
pub const EDITOR_H:  u32 = 800;

#[derive(Clone)]
pub struct Screen {
    pub w: f32,
    pub h: f32,
}

impl Screen {
    pub fn get() -> Self {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};
            use windows_sys::Win32::Foundation::RECT;
            let mut rc: RECT = unsafe { std::mem::zeroed() };
            unsafe { SystemParametersInfoW(SPI_GETWORKAREA, 0, &mut rc as *mut RECT as *mut _, 0); }
            return Self { w: (rc.right - rc.left) as f32, h: (rc.bottom - rc.top) as f32 };
        }
        #[cfg(not(target_os = "windows"))]
        Self { w: 1920.0, h: 1080.0 }
    }

    pub fn sidebar_pos(&self) -> (i32, i32) {
        ((self.w as i32) - SIDEBAR_W as i32, 0)
    }

    pub fn editor_center_pos(&self) -> (i32, i32) {
        (
            ((self.w as i32) - EDITOR_W as i32) / 2,
            ((self.h as i32) - EDITOR_H as i32) / 2,
        )
    }
}
