#[cfg(feature = "XPlatformAccessTokenData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XPlatformAccessTokenData {
    pub token: *mut quest_hook::libil2cpp::Il2CppString,
    pub platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
}
#[cfg(feature = "XPlatformAccessTokenData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::XPlatformAccessTokenData => ""
    ."XPlatformAccessTokenData"
);
#[cfg(feature = "XPlatformAccessTokenData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::XPlatformAccessTokenData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "XPlatformAccessTokenData")]
impl crate::GlobalNamespace::XPlatformAccessTokenData {
    pub const kMinimalTokenLength: i32 = 64i32;
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        token: *mut quest_hook::libil2cpp::Il2CppString,
        platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (token, platformEnvironment),
        )?;
        Ok(__cordl_ret)
    }
}
