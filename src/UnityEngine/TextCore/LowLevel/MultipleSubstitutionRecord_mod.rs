#[cfg(feature = "UnityEngine+TextCore+LowLevel+MultipleSubstitutionRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MultipleSubstitutionRecord {
    pub m_TargetGlyphID: u32,
    pub m_SubstituteGlyphIDs: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MultipleSubstitutionRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::MultipleSubstitutionRecord =>
    "UnityEngine.TextCore.LowLevel"."MultipleSubstitutionRecord"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MultipleSubstitutionRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::MultipleSubstitutionRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MultipleSubstitutionRecord")]
impl crate::UnityEngine::TextCore::LowLevel::MultipleSubstitutionRecord {}
