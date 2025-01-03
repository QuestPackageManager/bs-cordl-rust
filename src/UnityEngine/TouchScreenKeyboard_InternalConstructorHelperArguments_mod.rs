#[cfg(feature = "UnityEngine+TouchScreenKeyboard_InternalConstructorHelperArguments")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TouchScreenKeyboard_InternalConstructorHelperArguments {
    pub keyboardType: u32,
    pub autocorrection: u32,
    pub multiline: u32,
    pub secure: u32,
    pub alert: u32,
    pub characterLimit: i32,
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard_InternalConstructorHelperArguments")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TouchScreenKeyboard_InternalConstructorHelperArguments =>
    "UnityEngine"."TouchScreenKeyboard_InternalConstructorHelperArguments"
);
#[cfg(feature = "UnityEngine+TouchScreenKeyboard_InternalConstructorHelperArguments")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TouchScreenKeyboard_InternalConstructorHelperArguments {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TouchScreenKeyboard_InternalConstructorHelperArguments")]
impl crate::UnityEngine::TouchScreenKeyboard_InternalConstructorHelperArguments {}
