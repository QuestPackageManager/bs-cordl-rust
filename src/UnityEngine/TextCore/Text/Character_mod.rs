#[cfg(feature = "UnityEngine+TextCore+Text+Character")]
#[repr(C)]
#[derive(Debug)]
pub struct Character {
    __cordl_parent: crate::UnityEngine::TextCore::Text::TextElement,
}
#[cfg(feature = "UnityEngine+TextCore+Text+Character")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::Character =>
    "UnityEngine.TextCore.Text"."Character"
);
#[cfg(feature = "UnityEngine+TextCore+Text+Character")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::Character {
    type Target = crate::UnityEngine::TextCore::Text::TextElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+Character")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::Character {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+Character")]
impl crate::UnityEngine::TextCore::Text::Character {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_u32_FontAsset_Glyph1(
        unicode: u32,
        fontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
        glyph: *mut crate::UnityEngine::TextCore::Glyph,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unicode, fontAsset, glyph))?;
        Ok(__cordl_object)
    }
    pub fn New_u32_u32_2(
        unicode: u32,
        glyphIndex: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unicode, glyphIndex))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_FontAsset_Glyph1(
        &mut self,
        unicode: u32,
        fontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
        glyph: *mut crate::UnityEngine::TextCore::Glyph,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (unicode, fontAsset, glyph))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_u32_2(
        &mut self,
        unicode: u32,
        glyphIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (unicode, glyphIndex))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+Character")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::Character {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}