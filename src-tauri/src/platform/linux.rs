use super::traits::{PlatformAdapter, PlatformCapabilities, PlatformKind, WindowDefaults};

/// Linux adapter: tray and global shortcuts depend on the desktop environment,
/// native window effects are not assumed, autostart uses XDG desktop entries.
pub struct LinuxAdapter;

impl PlatformAdapter for LinuxAdapter {
    fn kind(&self) -> PlatformKind {
        PlatformKind::Linux
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            system_tray: true,
            global_shortcut: true,
            window_effects: false,
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
