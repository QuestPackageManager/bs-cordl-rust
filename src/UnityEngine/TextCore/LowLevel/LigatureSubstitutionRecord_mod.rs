#[cfg(feature = "UnityEngine+TextCore+LowLevel+LigatureSubstitutionRecord")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LigatureSubstitutionRecord {
    pub m_ComponentGlyphIDs: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_LigatureGlyphID: u32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+LigatureSubstitutionRecord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord =>
    "UnityEngine.TextCore.LowLevel"."LigatureSubstitutionRecord"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+LigatureSubstitutionRecord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+LigatureSubstitutionRecord")]
impl crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord {
    pub fn get_componentGlyphIDs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u32>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u32> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_componentGlyphIDs",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ligatureGlyphID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ligatureGlyphID",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
