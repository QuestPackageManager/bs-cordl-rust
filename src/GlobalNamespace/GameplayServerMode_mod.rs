#[cfg(feature = "GameplayServerMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayServerMode {
    Countdown = 0i32,
    Managed = 1i32,
    QuickStartOneSong = 2i32,
}
#[cfg(feature = "GameplayServerMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for GameplayServerMode => ""."GameplayServerMode"
);
