#[cfg(feature = "UnityEngine+Yoga+YogaSize")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct YogaSize {
    pub width: f32,
    pub height: f32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaSize")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaSize =>
    "UnityEngine.Yoga"."YogaSize"
);
#[cfg(feature = "UnityEngine+Yoga+YogaSize")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Yoga::YogaSize {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaSize")]
impl crate::UnityEngine::Yoga::YogaSize {}
