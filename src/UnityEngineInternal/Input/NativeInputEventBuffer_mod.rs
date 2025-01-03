#[cfg(feature = "UnityEngineInternal+Input+NativeInputEventBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NativeInputEventBuffer {
    padding: [u8; 20usize],
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputEventBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngineInternal::Input::NativeInputEventBuffer => "UnityEngineInternal.Input"
    ."NativeInputEventBuffer"
);
#[cfg(feature = "UnityEngineInternal+Input+NativeInputEventBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngineInternal::Input::NativeInputEventBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputEventBuffer")]
impl crate::UnityEngineInternal::Input::NativeInputEventBuffer {}
