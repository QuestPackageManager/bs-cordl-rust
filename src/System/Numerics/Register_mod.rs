#[cfg(feature = "System+Numerics+Register")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Register {
    padding: [u8; 16usize],
}
#[cfg(feature = "System+Numerics+Register")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::Register => "System.Numerics"
    ."Register"
);
#[cfg(feature = "System+Numerics+Register")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Numerics::Register {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Numerics+Register")]
impl crate::System::Numerics::Register {}
