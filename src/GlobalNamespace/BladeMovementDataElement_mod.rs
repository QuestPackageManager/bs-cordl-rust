#[cfg(feature = "BladeMovementDataElement")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BladeMovementDataElement {
    pub _cordl_time: f32,
    pub segmentAngle: f32,
    pub topPos: crate::UnityEngine::Vector3,
    pub bottomPos: crate::UnityEngine::Vector3,
    pub segmentNormal: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BladeMovementDataElement")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BladeMovementDataElement => ""
    ."BladeMovementDataElement"
);
#[cfg(feature = "BladeMovementDataElement")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BladeMovementDataElement {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BladeMovementDataElement")]
impl crate::GlobalNamespace::BladeMovementDataElement {}
