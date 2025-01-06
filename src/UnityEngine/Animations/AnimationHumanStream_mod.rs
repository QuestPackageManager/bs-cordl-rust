#[cfg(feature = "UnityEngine+Animations+AnimationHumanStream")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationHumanStream {
    pub stream: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Animations+AnimationHumanStream")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animations::AnimationHumanStream =>
    "UnityEngine.Animations"."AnimationHumanStream"
);
#[cfg(feature = "UnityEngine+Animations+AnimationHumanStream")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationHumanStream {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationHumanStream")]
impl crate::UnityEngine::Animations::AnimationHumanStream {}
