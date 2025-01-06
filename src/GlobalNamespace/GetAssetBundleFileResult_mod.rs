#[cfg(feature = "GetAssetBundleFileResult")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetAssetBundleFileResult {
    pub isError: bool,
    pub assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "GetAssetBundleFileResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GetAssetBundleFileResult => ""
    ."GetAssetBundleFileResult"
);
#[cfg(feature = "GetAssetBundleFileResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::GetAssetBundleFileResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "GetAssetBundleFileResult")]
impl crate::GlobalNamespace::GetAssetBundleFileResult {
    pub fn Success(
        assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GetAssetBundleFileResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::GetAssetBundleFileResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Success", (assetBundlePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        isError: bool,
        assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (isError, assetBundlePath),
        )?;
        Ok(__cordl_ret.into())
    }
}
