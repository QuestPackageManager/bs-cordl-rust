#[cfg(feature = "Mono+ValueTuple")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ValueTuple {}
#[cfg(feature = "Mono+ValueTuple")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::ValueTuple => "Mono"."ValueTuple"
);
#[cfg(feature = "Mono+ValueTuple")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::ValueTuple {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+ValueTuple")]
impl crate::Mono::ValueTuple {}
