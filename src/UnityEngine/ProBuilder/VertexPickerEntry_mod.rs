#[cfg(feature = "UnityEngine+ProBuilder+VertexPickerEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VertexPickerEntry {
    pub mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
    pub vertex: i32,
    pub screenDistance: f32,
    pub worldPosition: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPickerEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::VertexPickerEntry =>
    "UnityEngine.ProBuilder"."VertexPickerEntry"
);
#[cfg(feature = "UnityEngine+ProBuilder+VertexPickerEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::VertexPickerEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPickerEntry")]
impl crate::UnityEngine::ProBuilder::VertexPickerEntry {}
