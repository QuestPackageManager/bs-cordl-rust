#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_SpriteGlyph {
    __cordl_parent: crate::UnityEngine::TextCore::Glyph,
    pub sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_SpriteGlyph {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_SpriteGlyph";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
impl std::ops::Deref for crate::TMPro::TMP_SpriteGlyph {
    type Target = crate::UnityEngine::TextCore::Glyph;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
impl std::ops::DerefMut for crate::TMPro::TMP_SpriteGlyph {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SpriteGlyph")]
impl crate::TMPro::TMP_SpriteGlyph {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_GlyphMetrics_GlyphRect_f32_i32_1(
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index, metrics, glyphRect, scale, atlasIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_GlyphMetrics_GlyphRect_f32_i32_Sprite2(
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (index, metrics, glyphRect, scale, atlasIndex, sprite),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_GlyphMetrics_GlyphRect_f32_i32_1(
        &mut self,
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::UnityEngine::TextCore::GlyphMetrics,
                            crate::UnityEngine::TextCore::GlyphRect,
                            f32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (index, metrics, glyphRect, scale, atlasIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_GlyphMetrics_GlyphRect_f32_i32_Sprite2(
        &mut self,
        index: u32,
        metrics: crate::UnityEngine::TextCore::GlyphMetrics,
        glyphRect: crate::UnityEngine::TextCore::GlyphRect,
        scale: f32,
        atlasIndex: i32,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::UnityEngine::TextCore::GlyphMetrics,
                            crate::UnityEngine::TextCore::GlyphRect,
                            f32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (index, metrics, glyphRect, scale, atlasIndex, sprite),
                )?
        };
        Ok(__cordl_ret.into())
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
