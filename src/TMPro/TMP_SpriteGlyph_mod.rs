#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_SpriteGlyph {
    __cordl_parent: crate::UnityEngine::TextCore::Glyph,
    pub sprite: *mut crate::UnityEngine::Sprite,
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_SpriteGlyph => "TMPro"
    ."TMP_SpriteGlyph"
);
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
impl std::ops::Deref for crate::TMPro::TMP_SpriteGlyph {
    type Target = crate::UnityEngine::TextCore::Glyph;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
impl std::ops::DerefMut for crate::TMPro::TMP_SpriteGlyph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
impl crate::TMPro::TMP_SpriteGlyph {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_u32_GlyphMetrics_GlyphRect_f32_i32_1(
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index, metrics, glyphRect, scale, atlasIndex))?;
        Ok(__cordl_object)
    }
    pub fn New_u32_GlyphMetrics_GlyphRect_f32_i32_Sprite2(
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
        sprite: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (index, metrics, glyphRect, scale, atlasIndex, sprite),
            )?;
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
    pub fn _ctor_u32_GlyphMetrics_GlyphRect_f32_i32_1(
        &mut self,
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index, metrics, glyphRect, scale, atlasIndex))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_GlyphMetrics_GlyphRect_f32_i32_Sprite2(
        &mut self,
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
        sprite: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index, metrics, glyphRect, scale, atlasIndex, sprite))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_SpriteGlyph {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
