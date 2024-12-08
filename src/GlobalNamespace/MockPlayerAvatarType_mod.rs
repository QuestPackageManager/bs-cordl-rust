#[cfg(feature = "MockPlayerAvatarType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MockPlayerAvatarType {
    Beat = 0i32,
    Meta = 1i32,
}
#[cfg(feature = "MockPlayerAvatarType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlayerAvatarType => ""
    ."MockPlayerAvatarType"
);
