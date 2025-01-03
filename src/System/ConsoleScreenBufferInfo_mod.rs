#[cfg(feature = "System+ConsoleScreenBufferInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ConsoleScreenBufferInfo {
    pub Size: crate::System::Coord,
    pub CursorPosition: crate::System::Coord,
    pub Attribute: i16,
    pub Window: crate::System::SmallRect,
    pub MaxWindowSize: crate::System::Coord,
}
#[cfg(feature = "System+ConsoleScreenBufferInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ConsoleScreenBufferInfo => "System"
    ."ConsoleScreenBufferInfo"
);
#[cfg(feature = "System+ConsoleScreenBufferInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ConsoleScreenBufferInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ConsoleScreenBufferInfo")]
impl crate::System::ConsoleScreenBufferInfo {}
