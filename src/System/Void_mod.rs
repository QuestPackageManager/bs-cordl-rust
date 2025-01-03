#[cfg(feature = "System+Void")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Void {}
#[cfg(feature = "System+Void")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Void => "System"."Void"
);
#[cfg(feature = "System+Void")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Void {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Void")]
impl crate::System::Void {}
