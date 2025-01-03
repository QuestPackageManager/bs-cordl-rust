#[cfg(feature = "LoadBeatmapLevelDataResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LoadBeatmapLevelDataResult {
    pub isError: bool,
    pub beatmapLevelData: *mut crate::GlobalNamespace::IBeatmapLevelData,
}
#[cfg(feature = "LoadBeatmapLevelDataResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LoadBeatmapLevelDataResult =>
    ""."LoadBeatmapLevelDataResult"
);
#[cfg(feature = "LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LoadBeatmapLevelDataResult")]
impl crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    pub fn FromValue(
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::LoadBeatmapLevelDataResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromValue", (beatmapLevelData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Success(
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::LoadBeatmapLevelDataResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Success", (beatmapLevelData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        isError: bool,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (isError, beatmapLevelData),
        )?;
        Ok(__cordl_ret.into())
    }
}
