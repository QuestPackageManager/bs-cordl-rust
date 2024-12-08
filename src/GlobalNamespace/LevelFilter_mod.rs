#[cfg(feature = "LevelFilter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LevelFilter {
    pub songOwned: bool,
    pub songNotOwned: bool,
    pub songUnplayed: bool,
    pub difficulties: BeatmapDifficultyMask,
    pub songPacks: SongPackMask,
    pub characteristicSerializedName: *mut crate::System::String,
    pub minBpm: f32,
    pub maxBpm: f32,
    pub sensitivity: PlayerSensitivityFlag,
    pub searchText: *mut crate::System::String,
    pub limitIds: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
}
#[cfg(feature = "LevelFilter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for LevelFilter => ""."LevelFilter"
);
#[cfg(feature = "LevelFilter")]
unsafe impl quest_hook::libil2cpp::ThisArgument for LevelFilter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LevelFilter")]
impl LevelFilter {
    #[cfg(feature = "LevelFilter+__c__DisplayClass13_1")]
    pub type __c__DisplayClass13_1 = crate::GlobalNamespace::LevelFilter___c__DisplayClass13_1;
    #[cfg(feature = "LevelFilter+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::LevelFilter___c__DisplayClass13_0;
    #[cfg(feature = "LevelFilter+_FilterLevelsAsync_d__13")]
    pub type _FilterLevelsAsync_d__13 = crate::GlobalNamespace::LevelFilter__FilterLevelsAsync_d__13;
    #[cfg(feature = "LevelFilter+__c")]
    pub type __c = crate::GlobalNamespace::LevelFilter___c;
}
