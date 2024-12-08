#[cfg(feature = "UnityEngine+Animations+AnimationStream")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnimationStream {
    pub m_AnimatorBindingsVersion: u32,
    pub constant: crate::System::IntPtr,
    pub input: crate::System::IntPtr,
    pub output: crate::System::IntPtr,
    pub workspace: crate::System::IntPtr,
    pub inputStreamAccessor: crate::System::IntPtr,
    pub animationHandleBinder: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Animations+AnimationStream")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animations::AnimationStream =>
    "UnityEngine.Animations"."AnimationStream"
);
#[cfg(feature = "UnityEngine+Animations+AnimationStream")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationStream {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationStream")]
impl crate::UnityEngine::Animations::AnimationStream {}
