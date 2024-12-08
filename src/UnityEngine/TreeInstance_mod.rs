#[cfg(feature = "UnityEngine+TreeInstance")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TreeInstance {
    pub position: crate::UnityEngine::Vector3,
    pub widthScale: f32,
    pub heightScale: f32,
    pub rotation: f32,
    pub color: crate::UnityEngine::Color32,
    pub lightmapColor: crate::UnityEngine::Color32,
    pub prototypeIndex: i32,
    pub temporaryDistance: f32,
}
#[cfg(feature = "UnityEngine+TreeInstance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TreeInstance => "UnityEngine"
    ."TreeInstance"
);
#[cfg(feature = "UnityEngine+TreeInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::TreeInstance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TreeInstance")]
impl crate::UnityEngine::TreeInstance {}
