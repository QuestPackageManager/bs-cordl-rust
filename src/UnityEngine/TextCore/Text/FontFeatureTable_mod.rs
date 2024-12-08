#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
#[repr(C)]
#[derive(Debug)]
pub struct FontFeatureTable {
    __cordl_parent: crate::System::Object,
    pub m_MultipleSubstitutionRecords: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::LowLevel::MultipleSubstitutionRecord,
    >,
    pub m_LigatureSubstitutionRecords: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
    >,
    pub m_GlyphPairAdjustmentRecords: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    >,
    pub m_MarkToBaseAdjustmentRecords: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
    >,
    pub m_MarkToMarkAdjustmentRecords: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
    >,
    pub m_LigatureSubstitutionRecordLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
        >,
    >,
    pub m_GlyphPairAdjustmentRecordLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    >,
    pub m_MarkToBaseAdjustmentRecordLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
    >,
    pub m_MarkToMarkAdjustmentRecordLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::FontFeatureTable =>
    "UnityEngine.TextCore.Text"."FontFeatureTable"
);
#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::FontFeatureTable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::FontFeatureTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
impl crate::UnityEngine::TextCore::Text::FontFeatureTable {
    #[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable+__c")]
    pub type __c = crate::UnityEngine::TextCore::Text::FontFeatureTable___c;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn SortGlyphPairAdjustmentRecords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortGlyphPairAdjustmentRecords", ())?;
        Ok(__cordl_ret)
    }
    pub fn SortMarkToBaseAdjustmentRecords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortMarkToBaseAdjustmentRecords", ())?;
        Ok(__cordl_ret)
    }
    pub fn SortMarkToMarkAdjustmentRecords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortMarkToMarkAdjustmentRecords", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::FontFeatureTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
