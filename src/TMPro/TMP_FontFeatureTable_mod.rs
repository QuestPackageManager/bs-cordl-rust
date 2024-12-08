#[cfg(feature = "TMPro+TMP_FontFeatureTable")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontFeatureTable {
    __cordl_parent: crate::System::Object,
    pub m_GlyphPairAdjustmentRecords: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_GlyphPairAdjustmentRecord,
    >,
    pub m_GlyphPairAdjustmentRecordLookupDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        u32,
        *mut crate::TMPro::TMP_GlyphPairAdjustmentRecord,
    >,
}
#[cfg(feature = "TMPro+TMP_FontFeatureTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontFeatureTable => "TMPro"
    ."TMP_FontFeatureTable"
);
#[cfg(feature = "TMPro+TMP_FontFeatureTable")]
impl std::ops::Deref for crate::TMPro::TMP_FontFeatureTable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontFeatureTable")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontFeatureTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontFeatureTable")]
impl crate::TMPro::TMP_FontFeatureTable {
    #[cfg(feature = "TMPro+TMP_FontFeatureTable+__c")]
    pub type __c = crate::TMPro::TMP_FontFeatureTable___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_glyphPairAdjustmentRecords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_GlyphPairAdjustmentRecord,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_GlyphPairAdjustmentRecord,
        > = __cordl_object.invoke("get_glyphPairAdjustmentRecords", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_glyphPairAdjustmentRecords(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::TMPro::TMP_GlyphPairAdjustmentRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_glyphPairAdjustmentRecords", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_FontFeatureTable")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_FontFeatureTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
