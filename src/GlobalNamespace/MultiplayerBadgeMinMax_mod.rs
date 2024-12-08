#[cfg(feature = "MultiplayerBadgeMinMax")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerBadgeMinMax {
    Max = 1i32,
    Min = 0i32,
}
#[cfg(feature = "MultiplayerBadgeMinMax")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for MultiplayerBadgeMinMax => ""."MultiplayerBadgeMinMax"
);
