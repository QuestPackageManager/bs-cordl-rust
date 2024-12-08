#[cfg(feature = "LevelSelectionOptions")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LevelSelectionOptions {
    pub preferredCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub preferredDifficulty: crate::System::Nullable_1<
        crate::GlobalNamespace::BeatmapDifficulty,
    >,
    pub sortAlphabetically: bool,
    pub isFiltered: bool,
}
#[cfg(feature = "LevelSelectionOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelSelectionOptions => ""
    ."LevelSelectionOptions"
);
#[cfg(feature = "LevelSelectionOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::LevelSelectionOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LevelSelectionOptions")]
impl crate::GlobalNamespace::LevelSelectionOptions {}
