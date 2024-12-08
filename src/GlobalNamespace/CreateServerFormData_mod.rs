#[cfg(feature = "CreateServerFormData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CreateServerFormData {
    pub usePassword: bool,
    pub password: *mut crate::System::String,
    pub maxPlayers: i32,
    pub allowInviteOthers: bool,
    pub netDiscoverable: bool,
    pub difficulties: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub modifiers: crate::GlobalNamespace::GameplayModifierMask,
    pub songPacks: crate::GlobalNamespace::SongPackMask,
    pub gameplayServerMode: crate::GlobalNamespace::GameplayServerMode,
    pub songSelectionMode: crate::GlobalNamespace::SongSelectionMode,
    pub gameplayServerControlSettings: crate::GlobalNamespace::GameplayServerControlSettings,
}
#[cfg(feature = "CreateServerFormData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreateServerFormData => ""
    ."CreateServerFormData"
);
#[cfg(feature = "CreateServerFormData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::CreateServerFormData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "CreateServerFormData")]
impl crate::GlobalNamespace::CreateServerFormData {}
