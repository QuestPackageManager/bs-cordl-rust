#[cfg(feature = "System+DateTimeToken")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DateTimeToken {
    pub dtt: crate::System::DateTimeParse_DTT,
    pub suffix: crate::System::TokenType,
    pub num: i32,
}
#[cfg(feature = "System+DateTimeToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeToken => "System"
    ."DateTimeToken"
);
#[cfg(feature = "System+DateTimeToken")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::DateTimeToken {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+DateTimeToken")]
impl crate::System::DateTimeToken {}
