#[cfg(feature = "TMPro+TMP_Character")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Character {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_TextElement>,
}
#[cfg(feature = "TMPro+TMP_Character")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Character => "TMPro"."TMP_Character"
);
#[cfg(feature = "TMPro+TMP_Character")]
impl std::ops::Deref for crate::TMPro::TMP_Character {
    type Target = quest_hook::libil2cpp::Gc<crate::TMPro::TMP_TextElement>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Character")]
impl std::ops::DerefMut for crate::TMPro::TMP_Character {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Character")]
impl crate::TMPro::TMP_Character {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_Gc1(
        unicode: u32,
        glyph: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unicode, glyph))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_Gc_Gc2(
        unicode: u32,
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        glyph: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unicode, fontAsset, glyph))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_u32_3(
        unicode: u32,
        glyphIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unicode, glyphIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_Gc1(
        &mut self,
        unicode: u32,
        glyph: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (unicode, glyph))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_Gc_Gc2(
        &mut self,
        unicode: u32,
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        glyph: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (unicode, fontAsset, glyph))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_u32_3(
        &mut self,
        unicode: u32,
        glyphIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (unicode, glyphIndex))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_Character")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Character {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
