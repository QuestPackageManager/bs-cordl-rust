#[cfg(feature = "LevelDataAssetDownloadUpdate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LevelDataAssetDownloadUpdate {
    pub levelID: *mut quest_hook::libil2cpp::Il2CppString,
    pub bytesTotal: u32,
    pub bytesTransferred: u32,
    pub assetDownloadingState: crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState,
}
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelDataAssetDownloadUpdate =>
    ""."LevelDataAssetDownloadUpdate"
);
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
impl crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
    #[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
    pub type AssetDownloadingState = crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState;
    pub fn _ctor(
        &mut self,
        levelID: *mut quest_hook::libil2cpp::Il2CppString,
        bytesTotal: u32,
        bytesTransferred: u32,
        assetDownloadingState: crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (levelID, bytesTotal, bytesTransferred, assetDownloadingState),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelDataAssetDownloadUpdate_AssetDownloadingState {
    Completed = 2i32,
    Downloading = 1i32,
    PreparingToDownload = 0i32,
}
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState => ""
    ."LevelDataAssetDownloadUpdate/AssetDownloadingState"
);
