#[cfg(feature = "TMPro+TMP_TextElement")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_TextElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ElementType: crate::TMPro::TextElementType,
    pub m_Unicode: u32,
    pub m_TextAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Asset>,
    pub m_Glyph: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    pub m_GlyphIndex: u32,
    pub m_Scale: f32,
}
#[cfg(feature = "TMPro+TMP_TextElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_TextElement => "TMPro"
    ."TMP_TextElement"
);
#[cfg(feature = "TMPro+TMP_TextElement")]
impl std::ops::Deref for crate::TMPro::TMP_TextElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextElement")]
impl std::ops::DerefMut for crate::TMPro::TMP_TextElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextElement")]
impl crate::TMPro::TMP_TextElement {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_elementType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextElementType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TextElementType = __cordl_object
            .invoke("get_elementType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_glyph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Glyph,
        > = __cordl_object.invoke("get_glyph", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_glyphIndex(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_glyphIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Asset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Asset> = __cordl_object
            .invoke("get_textAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unicode(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_unicode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_glyph(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_glyph", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_glyphIndex(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_glyphIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_textAsset(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Asset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textAsset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_unicode(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_unicode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_TextElement")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_TextElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
