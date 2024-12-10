#[cfg(feature = "LastUsedBeatmapDataCache")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LastUsedBeatmapDataCache {
    pub cachedReadonlyBeatmapData: *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub _environmentInfo: *mut crate::GlobalNamespace::IEnvironmentInfo,
    pub _gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub _playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
}
#[cfg(feature = "LastUsedBeatmapDataCache")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LastUsedBeatmapDataCache => ""
    ."LastUsedBeatmapDataCache"
);
#[cfg(feature = "LastUsedBeatmapDataCache")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::LastUsedBeatmapDataCache {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LastUsedBeatmapDataCache")]
impl crate::GlobalNamespace::LastUsedBeatmapDataCache {
    pub fn AreSameBeatmapDataCached(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AreSameBeatmapDataCached",
            (beatmapKey, environmentInfo, gameplayModifiers, playerSpecificSettings),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
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
        Ok(__cordl_ret.into())
    }
}
