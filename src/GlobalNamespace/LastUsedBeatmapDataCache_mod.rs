#[cfg(feature = "LastUsedBeatmapDataCache")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LastUsedBeatmapDataCache {
    pub cachedReadonlyBeatmapData: *mut IReadonlyBeatmapData,
    pub _beatmapKey: BeatmapKey,
    pub _environmentInfo: *mut IEnvironmentInfo,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub _playerSpecificSettings: *mut PlayerSpecificSettings,
}
#[cfg(feature = "LastUsedBeatmapDataCache")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for LastUsedBeatmapDataCache => ""
    ."LastUsedBeatmapDataCache"
);
#[cfg(feature = "LastUsedBeatmapDataCache")]
unsafe impl quest_hook::libil2cpp::ThisArgument for LastUsedBeatmapDataCache {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LastUsedBeatmapDataCache")]
impl LastUsedBeatmapDataCache {
    pub fn _ctor(
        &mut self,
        beatmapData: *mut IReadonlyBeatmapData,
        beatmapKey: BeatmapKey,
        environmentInfo: *mut IEnvironmentInfo,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                beatmapData,
                beatmapKey,
                environmentInfo,
                gameplayModifiers,
                playerSpecificSettings,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AreSameBeatmapDataCached(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        environmentInfo: *mut IEnvironmentInfo,
        gameplayModifiers: *mut GameplayModifiers,
        playerSpecificSettings: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AreSameBeatmapDataCached",
            (beatmapKey, environmentInfo, gameplayModifiers, playerSpecificSettings),
        )?;
        Ok(__cordl_ret)
    }
}
