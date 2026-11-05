use super::traits::{PlatformAdapter, PlatformCapabilities, PlatformKind, WindowDefaults};

/// Fallback adapter for targets without a dedicated implementation.
pub struct UnsupportedAdapter;

impl PlatformAdapter for UnsupportedAdapter {
    fn kind(&self) -> PlatformKind {
        PlatformKind::Unknown
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            system_tray: false,
            global_shortcut: false,
            window_effects: false,
            autostart: false,
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
