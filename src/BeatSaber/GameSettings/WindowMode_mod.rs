#[cfg(feature = "BeatSaber+GameSettings+WindowMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Fullscreen = 1i32,
    Windowed = 0i32,
}
#[cfg(feature = "BeatSaber+GameSettings+WindowMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::WindowMode =>
    "BeatSaber.GameSettings"."WindowMode"
);
