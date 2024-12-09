#[cfg(feature = "System+Globalization+InternalEncodingDataItem")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InternalEncodingDataItem {
    pub webName: *mut quest_hook::libil2cpp::Il2CppString,
    pub codePage: u16,
}
#[cfg(feature = "System+Globalization+InternalEncodingDataItem")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::InternalEncodingDataItem
    => "System.Globalization"."InternalEncodingDataItem"
);
#[cfg(feature = "System+Globalization+InternalEncodingDataItem")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::InternalEncodingDataItem {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+InternalEncodingDataItem")]
impl crate::System::Globalization::InternalEncodingDataItem {}
