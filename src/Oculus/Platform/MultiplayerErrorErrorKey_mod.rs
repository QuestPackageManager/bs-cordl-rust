#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerErrorErrorKey {
    #[default]
    DestinationUnavailable = 1i32,
    DlcRequired = 2i32,
    General = 3i32,
    GroupFull = 4i32,
    InviterNotJoinable = 5i32,
    LevelNotHighEnough = 6i32,
    LevelNotUnlocked = 7i32,
    NetworkTimeout = 8i32,
    NoLongerAvailable = 9i32,
    TutorialRequired = 11i32,
    Unknown = 0i32,
    UpdateRequired = 10i32,
}
#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::MultiplayerErrorErrorKey =>
    "Oculus.Platform"."MultiplayerErrorErrorKey"
);
