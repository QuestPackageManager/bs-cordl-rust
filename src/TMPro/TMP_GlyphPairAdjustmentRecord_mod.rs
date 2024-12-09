#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_GlyphPairAdjustmentRecord {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_FirstAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    pub m_SecondAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    pub m_FeatureLookupFlags: crate::TMPro::FontFeatureLookupFlags,
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_GlyphPairAdjustmentRecord => "TMPro"
    ."TMP_GlyphPairAdjustmentRecord"
);
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl std::ops::Deref for crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl std::ops::DerefMut for crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    pub fn New_GlyphPairAdjustmentRecord1(
        glyphPairAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (glyphPairAdjustmentRecord))?;
        Ok(__cordl_object)
    }
    pub fn New_TMP_GlyphAdjustmentRecord_TMP_GlyphAdjustmentRecord0(
        firstAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
        secondAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (firstAdjustmentRecord, secondAdjustmentRecord))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_GlyphPairAdjustmentRecord1(
        &mut self,
        glyphPairAdjustmentRecord: crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (glyphPairAdjustmentRecord))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TMP_GlyphAdjustmentRecord_TMP_GlyphAdjustmentRecord0(
        &mut self,
        firstAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
        secondAdjustmentRecord: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (firstAdjustmentRecord, secondAdjustmentRecord))?;
        Ok(__cordl_ret)
    }
    pub fn get_featureLookupFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::FontFeatureLookupFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::FontFeatureLookupFlags = __cordl_object
            .invoke("get_featureLookupFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_firstAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_GlyphAdjustmentRecord> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TMP_GlyphAdjustmentRecord = __cordl_object
            .invoke("get_firstAdjustmentRecord", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_secondAdjustmentRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_GlyphAdjustmentRecord> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TMP_GlyphAdjustmentRecord = __cordl_object
            .invoke("get_secondAdjustmentRecord", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_featureLookupFlags(
        &mut self,
        value: crate::TMPro::FontFeatureLookupFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_featureLookupFlags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_firstAdjustmentRecord(
        &mut self,
        value: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_firstAdjustmentRecord", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_secondAdjustmentRecord(
        &mut self,
        value: crate::TMPro::TMP_GlyphAdjustmentRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_secondAdjustmentRecord", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_GlyphPairAdjustmentRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_GlyphPairAdjustmentRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
