#[cfg(feature = "GetAssetBundleFileResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GetAssetBundleFileResult {
    pub isError: bool,
    pub assetBundlePath: *mut crate::System::String,
}
#[cfg(feature = "GetAssetBundleFileResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for GetAssetBundleFileResult => ""
    ."GetAssetBundleFileResult"
);
#[cfg(feature = "GetAssetBundleFileResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument for GetAssetBundleFileResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "GetAssetBundleFileResult")]
impl GetAssetBundleFileResult {
    pub fn _ctor(
        &mut self,
        isError: bool,
        assetBundlePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (isError, assetBundlePath),
        )?;
        Ok(__cordl_ret)
    }
}
