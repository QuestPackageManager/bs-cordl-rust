#[cfg(feature = "System+Coord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Coord {
    pub X: i16,
    pub Y: i16,
}
#[cfg(feature = "System+Coord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Coord => "System"."Coord"
);
#[cfg(feature = "System+Coord")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Coord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Coord")]
impl crate::System::Coord {}
