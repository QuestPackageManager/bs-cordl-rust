#[cfg(feature = "UnityEngineInternal+Input+NativeInputEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NativeInputEvent {
    padding: quest_hook::libil2cpp::ValueTypePadding<20usize>,
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::Input::NativeInputEvent =>
    "UnityEngineInternal.Input"."NativeInputEvent"
);
#[cfg(feature = "UnityEngineInternal+Input+NativeInputEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngineInternal::Input::NativeInputEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputEvent")]
impl crate::UnityEngineInternal::Input::NativeInputEvent {
    pub const structSize: i32 = 20i32;
}
