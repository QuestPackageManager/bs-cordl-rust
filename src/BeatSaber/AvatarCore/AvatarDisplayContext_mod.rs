#[cfg(feature = "BeatSaber+AvatarCore+AvatarDisplayContext")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarDisplayContext {
    #[default]
    MultiplayerBigAvatar = 5i32,
    MultiplayerGameplay = 3i32,
    MultiplayerLobby = 2i32,
    MultiplayerResults = 4i32,
    UI = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarDisplayContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarDisplayContext =>
    "BeatSaber.AvatarCore"."AvatarDisplayContext"
);
