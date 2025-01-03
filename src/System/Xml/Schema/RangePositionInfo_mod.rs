#[cfg(feature = "System+Xml+Schema+RangePositionInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RangePositionInfo {
    pub curpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub rangeCounters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Decimal>,
    >,
}
#[cfg(feature = "System+Xml+Schema+RangePositionInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::RangePositionInfo =>
    "System.Xml.Schema"."RangePositionInfo"
);
#[cfg(feature = "System+Xml+Schema+RangePositionInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::RangePositionInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+RangePositionInfo")]
impl crate::System::Xml::Schema::RangePositionInfo {}
