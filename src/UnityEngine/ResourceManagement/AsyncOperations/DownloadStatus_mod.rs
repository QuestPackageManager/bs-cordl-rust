#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+DownloadStatus")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DownloadStatus {
    pub TotalBytes: i64,
    pub DownloadedBytes: i64,
    pub IsDone: bool,
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+DownloadStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus =>
    "UnityEngine.ResourceManagement.AsyncOperations"."DownloadStatus"
);
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+DownloadStatus")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+DownloadStatus")]
impl crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus {
    pub fn get_Percent(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Percent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
