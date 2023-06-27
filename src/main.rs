// to hide console
//   msvc:
//     cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
//   gcc:
//     cargo rustc --release -- -Clink-args="-Wl,--subsystem,windows"


/*!
    An application that runs in the system tray. Requires the following features: "tray-notification message-window menu cursor"
*/
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
use std::sync::Mutex;
use nwd::NwgUi;
use nwg::NativeUi;
mod shortcuts;


#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    // #[nwg_resource(source_file: Some("../res/tray.ico"))]  // need a file
    #[nwg_resource(source_bin: Some(include_bytes!("../res/tray.ico")))]  // static compiled into exe
    icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon), tip: Some("WinShortcuts"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Hot Corner")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::toggle_enabled_hot_corner])]
    tmi_hot_corner: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Block LWin")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::toggle_enabled_lwin_blocker])]
    tmi_lwin_blocker: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Help")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::how_help_msg])]
    tmi_help: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tmi_exit: nwg::MenuItem,

    executor: Mutex<shortcuts::ShortcutsExecutor>,
}

impl SystemTray {

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn how_help_msg(&self) {
        let msg = "Click screen Left-Top corner to show task view.\n(By simulating hotkey RWin+Tab)";
        nwg::modal_info_message(&self.window, "WinShortcuts Help", msg);
    }

    fn toggle_enabled_lwin_blocker(&self) {
        if self.tmi_lwin_blocker.checked() {
            self.executor.lock().unwrap().disable_lwin_blocker();
            self.tmi_lwin_blocker.set_checked(false);
        }
        else {
            self.executor.lock().unwrap().enable_lwin_blocker();
            self.tmi_lwin_blocker.set_checked(true);
        }
    }

    fn toggle_enabled_hot_corner(&self) {
        if self.tmi_hot_corner.checked() {
            self.executor.lock().unwrap().disable_hot_corner();
            self.tmi_hot_corner.set_checked(false);
        }
        else {
            self.executor.lock().unwrap().enable_hot_corner();
            self.tmi_hot_corner.set_checked(true);
            // let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
            // self.tray.show("Listener started!", Some("Hot Corner"), Some(flags), Some(&self.icon));
        }
    }

    fn init_menu_item_status(&self) {
        self.tmi_hot_corner.set_checked(self.executor.lock().unwrap().is_hot_corner_enabled());
        self.tmi_lwin_blocker.set_checked(self.executor.lock().unwrap().is_lwin_blocker_enabled());
    }

}


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    SystemTray::init_menu_item_status(&_ui);
    nwg::dispatch_thread_events();
}
