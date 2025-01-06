#[cfg(feature = "System+SmallRect")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SmallRect {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
#[cfg(feature = "System+SmallRect")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::SmallRect => "System"."SmallRect"
);
#[cfg(feature = "System+SmallRect")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::SmallRect {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+SmallRect")]
impl crate::System::SmallRect {}
