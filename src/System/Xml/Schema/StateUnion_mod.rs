#[cfg(feature = "System+Xml+Schema+StateUnion")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StateUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<4usize>,
}
#[cfg(feature = "System+Xml+Schema+StateUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::StateUnion =>
    "System.Xml.Schema"."StateUnion"
);
#[cfg(feature = "System+Xml+Schema+StateUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::StateUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+StateUnion")]
impl crate::System::Xml::Schema::StateUnion {}
