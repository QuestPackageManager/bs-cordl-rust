#[cfg(feature = "System+Globalization+InternalCodePageDataItem")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InternalCodePageDataItem {
    pub codePage: u16,
    pub uiFamilyCodePage: u16,
    pub flags: u32,
    pub Names: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Globalization+InternalCodePageDataItem")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::InternalCodePageDataItem
    => "System.Globalization"."InternalCodePageDataItem"
);
#[cfg(feature = "System+Globalization+InternalCodePageDataItem")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::InternalCodePageDataItem {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+InternalCodePageDataItem")]
impl crate::System::Globalization::InternalCodePageDataItem {}
