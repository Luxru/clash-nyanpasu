#[cfg(target_os = "macos")]
pub mod macos {
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
    use objc2_foundation::{MainThreadMarker, NSProcessInfo};
    use std::cell::Cell;
    thread_local! {
        static MARK: Cell<MainThreadMarker> = Cell::new(MainThreadMarker::new().unwrap());
    }

    pub fn show_dock_icon() {
        let app = NSApplication::sharedApplication(MARK.get());
        app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
        let version = NSProcessInfo::processInfo().operatingSystemVersion();
        if version.majorVersion >= 14 {
            app.activate();
        } else {
            #[expect(deprecated, reason = "Required on supported macOS versions before 14")]
            app.activateIgnoringOtherApps(true);
        }
    }

    pub fn hide_dock_icon() {
        let app = NSApplication::sharedApplication(MARK.get());
        app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    }
}
