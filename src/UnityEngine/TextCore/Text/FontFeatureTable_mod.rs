#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
#[repr(C)]
#[derive(Debug)]
pub struct FontFeatureTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_MultipleSubstitutionRecords: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::LowLevel::MultipleSubstitutionRecord,
        >,
    >,
    pub m_LigatureSubstitutionRecords: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
        >,
    >,
    pub m_GlyphPairAdjustmentRecords: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
        >,
    >,
    pub m_MarkToBaseAdjustmentRecords: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
        >,
    >,
    pub m_MarkToMarkAdjustmentRecords: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
        >,
    >,
    pub m_LigatureSubstitutionRecordLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            *mut crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        >,
    >,
    pub m_GlyphPairAdjustmentRecordLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
        >,
    >,
    pub m_MarkToBaseAdjustmentRecordLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
        >,
    >,
    pub m_MarkToMarkAdjustmentRecordLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
        >,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::FontFeatureTable =>
    "UnityEngine.TextCore.Text"."FontFeatureTable"
);
#[cfg(feature = "UnityEngine+TextCore+Text+FontFeatureTable")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::FontFeatureTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SortGlyphPairAdjustmentRecords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortGlyphPairAdjustmentRecords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SortMarkToBaseAdjustmentRecords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortMarkToBaseAdjustmentRecords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SortMarkToMarkAdjustmentRecords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortMarkToMarkAdjustmentRecords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
