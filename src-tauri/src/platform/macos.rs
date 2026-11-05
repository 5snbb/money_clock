use super::traits::{PlatformAdapter, PlatformCapabilities, PlatformKind, WindowDefaults};

/// macOS adapter: menu-bar tray, global shortcuts, vibrancy window effects and
/// LaunchAgent-based autostart are available.
pub struct MacosAdapter;

impl PlatformAdapter for MacosAdapter {
    fn kind(&self) -> PlatformKind {
        PlatformKind::Macos
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            system_tray: true,
            global_shortcut: true,
            window_effects: true,
            autostart: true,
        }
    }

    fn window_defaults(&self) -> WindowDefaults {
        WindowDefaults {
            decorations: true,
            transparent: false,
            min_width: 960,
            min_height: 640,
        }
    }
}
