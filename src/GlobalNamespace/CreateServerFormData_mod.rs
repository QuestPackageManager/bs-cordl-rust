#[cfg(feature = "CreateServerFormData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CreateServerFormData {
    pub usePassword: bool,
    pub password: *mut crate::System::String,
    pub maxPlayers: i32,
    pub allowInviteOthers: bool,
    pub netDiscoverable: bool,
    pub difficulties: BeatmapDifficultyMask,
    pub modifiers: GameplayModifierMask,
    pub songPacks: SongPackMask,
    pub gameplayServerMode: GameplayServerMode,
    pub songSelectionMode: SongSelectionMode,
    pub gameplayServerControlSettings: GameplayServerControlSettings,
}
#[cfg(feature = "CreateServerFormData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for CreateServerFormData => ""."CreateServerFormData"
);
#[cfg(feature = "CreateServerFormData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for CreateServerFormData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "CreateServerFormData")]
impl CreateServerFormData {}
