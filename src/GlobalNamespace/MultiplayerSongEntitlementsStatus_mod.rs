#[cfg(feature = "MultiplayerSongEntitlementsStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerSongEntitlementsStatus {
    #[default]
    Invalid = 1i32,
    _cordl_Ok = 0i32,
}
#[cfg(feature = "MultiplayerSongEntitlementsStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerSongEntitlementsStatus => ""
    ."MultiplayerSongEntitlementsStatus"
);
