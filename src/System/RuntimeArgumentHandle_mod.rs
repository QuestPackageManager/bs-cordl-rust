#[cfg(feature = "System+RuntimeArgumentHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RuntimeArgumentHandle {
    pub args: crate::System::IntPtr,
}
#[cfg(feature = "System+RuntimeArgumentHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::RuntimeArgumentHandle => "System"
    ."RuntimeArgumentHandle"
);
#[cfg(feature = "System+RuntimeArgumentHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::RuntimeArgumentHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+RuntimeArgumentHandle")]
impl crate::System::RuntimeArgumentHandle {}
