#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontReference")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FontReference {
    pub familyName: *mut crate::System::String,
    pub styleName: *mut crate::System::String,
    pub faceIndex: i32,
    pub filePath: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontReference")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::LowLevel::FontReference
    => "UnityEngine.TextCore.LowLevel"."FontReference"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontReference")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::FontReference {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontReference")]
impl crate::UnityEngine::TextCore::LowLevel::FontReference {}
