#[cfg(feature = "System+InputRecord")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputRecord {
    pub EventType: i16,
    pub KeyDown: bool,
    pub RepeatCount: i16,
    pub VirtualKeyCode: i16,
    pub VirtualScanCode: i16,
    pub Character: char,
    pub ControlKeyState: i32,
    pub pad1: i32,
    pub pad2: bool,
}
#[cfg(feature = "System+InputRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::InputRecord => "System"."InputRecord"
);
#[cfg(feature = "System+InputRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::InputRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+InputRecord")]
impl crate::System::InputRecord {}
