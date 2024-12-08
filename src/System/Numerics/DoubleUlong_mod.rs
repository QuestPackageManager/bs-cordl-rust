#[cfg(feature = "System+Numerics+DoubleUlong")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DoubleUlong {
    padding: [u8; 8usize],
}
#[cfg(feature = "System+Numerics+DoubleUlong")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::DoubleUlong =>
    "System.Numerics"."DoubleUlong"
);
#[cfg(feature = "System+Numerics+DoubleUlong")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Numerics::DoubleUlong {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Numerics+DoubleUlong")]
impl crate::System::Numerics::DoubleUlong {}
