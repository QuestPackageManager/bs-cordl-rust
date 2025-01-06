#[cfg(feature = "UnityEngine+JNINativeMethod")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JNINativeMethod {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub fnPtr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+JNINativeMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::JNINativeMethod => "UnityEngine"
    ."JNINativeMethod"
);
#[cfg(feature = "UnityEngine+JNINativeMethod")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::JNINativeMethod {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+JNINativeMethod")]
impl crate::UnityEngine::JNINativeMethod {}
