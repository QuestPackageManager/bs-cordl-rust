#[cfg(feature = "System+BRECORD")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BRECORD {
    pub pvRecord: crate::System::IntPtr,
    pub pRecInfo: crate::System::IntPtr,
}
#[cfg(feature = "System+BRECORD")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::BRECORD => "System"."BRECORD"
);
#[cfg(feature = "System+BRECORD")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::BRECORD {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+BRECORD")]
impl crate::System::BRECORD {}