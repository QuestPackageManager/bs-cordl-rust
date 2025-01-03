#[cfg(feature = "UnityEngine+RenderBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RenderBuffer {
    pub m_RenderTextureInstanceID: i32,
    pub m_BufferPtr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+RenderBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderBuffer => "UnityEngine"
    ."RenderBuffer"
);
#[cfg(feature = "UnityEngine+RenderBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::RenderBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+RenderBuffer")]
impl crate::UnityEngine::RenderBuffer {}
