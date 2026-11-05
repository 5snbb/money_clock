use serde::Serialize;

/// Operating system family recognized by the adapter layer.
///
/// Only the variant matching the current target is constructed at compile time,
/// so the remaining variants can look unused on any single platform.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlatformKind {
    Windows,
    Macos,
    Linux,
    Unknown,
}

impl PlatformKind {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::Macos => "macOS",
            Self::Linux => "Linux",
            Self::Unknown => "Unknown",
        }
    }
}

/// Platform capabilities declared to the frontend.
///
/// These flags describe what the OS integration *may* support. They are
/// declarative and do not enable any behavior by themselves.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformCapabilities {
    pub system_tray: bool,
    pub global_shortcut: bool,
    pub window_effects: bool,
    pub autostart: bool,
}

/// Window defaults recommended by the platform adapter.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowDefaults {
    pub decorations: bool,
    pub transparent: bool,
    pub min_width: u32,
    pub min_height: u32,
}

/// Contract implemented once per operating system.
///
/// Business code must not branch on `cfg!(target_os = ...)`; it asks the
/// current adapter instead.
pub trait PlatformAdapter: Send + Sync {
    fn kind(&self) -> PlatformKind;

    fn display_name(&self) -> &'static str {
        self.kind().display_name()
    }

    fn capabilities(&self) -> PlatformCapabilities;

    fn window_defaults(&self) -> WindowDefaults;
}
