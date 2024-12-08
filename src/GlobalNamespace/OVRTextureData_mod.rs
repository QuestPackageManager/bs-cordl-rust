#[cfg(feature = "OVRTextureData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRTextureData {
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub width: i32,
    pub height: i32,
    pub format: crate::GlobalNamespace::OVRTextureFormat,
    pub transcodedFormat: crate::UnityEngine::TextureFormat,
    pub uri: *mut crate::System::String,
}
#[cfg(feature = "OVRTextureData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTextureData => ""
    ."OVRTextureData"
);
#[cfg(feature = "OVRTextureData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTextureData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTextureData")]
impl crate::GlobalNamespace::OVRTextureData {}
