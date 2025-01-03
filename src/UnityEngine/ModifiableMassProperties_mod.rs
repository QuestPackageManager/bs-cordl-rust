#[cfg(feature = "UnityEngine+ModifiableMassProperties")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ModifiableMassProperties {
    pub inverseMassScale: f32,
    pub inverseInertiaScale: f32,
    pub otherInverseMassScale: f32,
    pub otherInverseInertiaScale: f32,
}
#[cfg(feature = "UnityEngine+ModifiableMassProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ModifiableMassProperties =>
    "UnityEngine"."ModifiableMassProperties"
);
#[cfg(feature = "UnityEngine+ModifiableMassProperties")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ModifiableMassProperties {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ModifiableMassProperties")]
impl crate::UnityEngine::ModifiableMassProperties {}
