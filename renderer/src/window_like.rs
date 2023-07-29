use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WebDisplayHandle, WebWindowHandle};

pub struct WindowLike {
    pub window_handle: RawWindowHandle,
    pub display_handle: RawDisplayHandle,
}

unsafe impl raw_window_handle::HasRawWindowHandle for WindowLike {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        return self.window_handle;
    }
}

unsafe impl raw_window_handle::HasRawDisplayHandle for WindowLike {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        return self.display_handle;
    }
}

impl WindowLike {
    pub fn create_for_web(raw_id: u32) -> Self {
        let mut web_window_handle = WebWindowHandle::empty();
        web_window_handle.id = raw_id;
        let window_handle = RawWindowHandle::Web(web_window_handle);

        let web_display_handle = WebDisplayHandle::empty();
        let display_handle = RawDisplayHandle::Web(web_display_handle);

        Self {
            window_handle,
            display_handle,
        }
    }
}
