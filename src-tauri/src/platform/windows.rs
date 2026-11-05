use super::traits::{PlatformAdapter, PlatformCapabilities, PlatformKind, WindowDefaults};

/// Windows adapter: system tray, global shortcuts, acrylic/mica effects and
/// registry-based autostart are available.
pub struct WindowsAdapter;

impl PlatformAdapter for WindowsAdapter {
    fn kind(&self) -> PlatformKind {
        PlatformKind::Windows
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
            min_width: 900,
            min_height: 620,
        }
    }
}
