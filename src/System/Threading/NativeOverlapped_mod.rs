#[cfg(feature = "System+Threading+NativeOverlapped")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NativeOverlapped {
    pub InternalLow: crate::System::IntPtr,
    pub InternalHigh: crate::System::IntPtr,
    pub OffsetLow: i32,
    pub OffsetHigh: i32,
    pub EventHandle: crate::System::IntPtr,
}
#[cfg(feature = "System+Threading+NativeOverlapped")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::NativeOverlapped =>
    "System.Threading"."NativeOverlapped"
);
#[cfg(feature = "System+Threading+NativeOverlapped")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::NativeOverlapped {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+NativeOverlapped")]
impl crate::System::Threading::NativeOverlapped {}
