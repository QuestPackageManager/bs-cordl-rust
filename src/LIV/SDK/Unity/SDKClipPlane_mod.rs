#[cfg(feature = "LIV+SDK+Unity+SDKClipPlane")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKClipPlane {
    pub transform: crate::LIV::SDK::Unity::SDKMatrix4x4,
    pub width: i32,
    pub height: i32,
    pub tesselation: f32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKClipPlane")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKClipPlane => "LIV.SDK.Unity"
    ."SDKClipPlane"
);
#[cfg(feature = "LIV+SDK+Unity+SDKClipPlane")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKClipPlane {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKClipPlane")]
impl crate::LIV::SDK::Unity::SDKClipPlane {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
