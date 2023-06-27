use std::ptr;
use winapi::shared::windef::HHOOK;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::um::winuser::{SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx, WH_MOUSE_LL, MSLLHOOKSTRUCT, WH_KEYBOARD_LL, KBDLLHOOKSTRUCT};
use winapi::um::winuser::{HC_ACTION, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, WM_LBUTTONDOWN};
use winapi::um::winuser::{keybd_event, VK_LWIN, VK_TAB, VK_RWIN};
use winapi::um::libloaderapi::GetModuleHandleW;


pub struct ShortcutsExecutor {
    hc_enabled: bool,
    lwin_blocker_enabled: bool,
    mouse_hook: HHOOK,
    kb_hook: HHOOK,
}

impl Default for ShortcutsExecutor {
    fn default() -> Self {
        let mut ins = Self {
            hc_enabled: false,
            lwin_blocker_enabled: false,
            mouse_hook: ptr::null_mut(),
            kb_hook: ptr::null_mut(),
        };
        ins.enable_hot_corner();
        ins.disable_lwin_blocker();
        ins
    }
}

impl Drop for ShortcutsExecutor {
    fn drop(&mut self) {
        self.unreg_mouse_hook();
        self.unreg_keyboard_hook();
    }
}

impl ShortcutsExecutor {
    fn reg_mouse_hook(&mut self) {
        if self.mouse_hook == ptr::null_mut() {
            unsafe {
                let cur_module_handler = GetModuleHandleW(ptr::null_mut());
                self.mouse_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_callback), cur_module_handler, 0);
            }
        }
    }
    fn unreg_mouse_hook(&mut self) {
        if self.mouse_hook != ptr::null_mut() {
            unsafe {
                UnhookWindowsHookEx(self.mouse_hook);
            }
            self.mouse_hook = ptr::null_mut();
        }
    }

    fn reg_keyboard_hook(&mut self) {
        if self.kb_hook == ptr::null_mut() {
            unsafe {
                let cur_module_handler = GetModuleHandleW(ptr::null_mut());
                self.kb_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_callback), cur_module_handler, 0);
            }
        }
    }
    fn unreg_keyboard_hook(&mut self) {
        if self.kb_hook != ptr::null_mut() {
            unsafe {
                UnhookWindowsHookEx(self.kb_hook);
            }
            self.kb_hook = ptr::null_mut();
        }
    }

    pub fn enable_hot_corner(&mut self) {
        if !self.hc_enabled {
            self.reg_mouse_hook();
            self.hc_enabled = true;
        }
    }
    pub fn disable_hot_corner(&mut self) {
        if self.hc_enabled {
            self.unreg_mouse_hook();
            self.hc_enabled = false;
        }
    }
    pub fn is_hot_corner_enabled(&self) -> bool { self.hc_enabled }


    pub fn enable_lwin_blocker(&mut self) {
        if !self.lwin_blocker_enabled {
            self.reg_keyboard_hook();
            self.lwin_blocker_enabled = true;
        }
    }
    pub fn disable_lwin_blocker(&mut self) {
        if self.lwin_blocker_enabled {
            self.unreg_keyboard_hook();
            self.lwin_blocker_enabled = false;
        }
    }
    pub fn is_lwin_blocker_enabled(&self) -> bool { self.lwin_blocker_enabled }

}

unsafe fn send_task_view_keys() {
    // win down, tab down
    keybd_event(VK_RWIN as u8, 0, KEYEVENTF_EXTENDEDKEY, 0);
    keybd_event(VK_TAB as u8, 0, KEYEVENTF_EXTENDEDKEY, 0);
    // tab up, win up
    keybd_event(VK_TAB as u8, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
    keybd_event(VK_RWIN as u8, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
}


unsafe extern "system" fn mouse_callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION {
        if w_param == WM_LBUTTONDOWN as usize {
            let mouse_struct = *(l_param as *const MSLLHOOKSTRUCT);
            if mouse_struct.pt.x < 3 && mouse_struct.pt.y < 3 {
                // println!("Mouse clicked in the top left corner!");
                send_task_view_keys();
                return 1;  // block the event from passing on
            }
        }
    }
    CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param)
}

unsafe extern "system" fn keyboard_callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION {
        let keyboard_struct = *(l_param as *const KBDLLHOOKSTRUCT);
        if keyboard_struct.vkCode == VK_LWIN as u32 {
            return 1; // block lwin
        }
    }
    CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param)
}