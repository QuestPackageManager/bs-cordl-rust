#[cfg(feature = "UnityEngine+TextCore+Glyph")]
#[repr(C)]
#[derive(Debug)]
pub struct Glyph {
    __cordl_parent: crate::System::Object,
    pub m_Index: u32,
    pub m_Metrics: crate::UnityEngine::TextCore::GlyphMetrics,
    pub m_GlyphRect: crate::UnityEngine::TextCore::GlyphRect,
    pub m_Scale: f32,
    pub m_AtlasIndex: i32,
    pub m_ClassDefinitionType: crate::UnityEngine::TextCore::GlyphClassDefinitionType,
}
#[cfg(feature = "UnityEngine+TextCore+Glyph")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Glyph =>
    "UnityEngine.TextCore"."Glyph"
);
#[cfg(feature = "UnityEngine+TextCore+Glyph")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Glyph {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Glyph")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Glyph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Glyph")]
impl crate::UnityEngine::TextCore::Glyph {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_GlyphMarshallingStruct1(
        glyphStruct: crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (glyphStruct))?;
        Ok(__cordl_object)
    }
    pub fn New_u32_GlyphMetrics_GlyphRect_f32_i32_2(
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
    pub fn _ctor_GlyphMarshallingStruct1(
        &mut self,
        glyphStruct: crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (glyphStruct))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_GlyphMetrics_GlyphRect_f32_i32_2(
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
    pub fn get_atlasIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_atlasIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_glyphRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::GlyphRect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::GlyphRect = __cordl_object
            .invoke("get_glyphRect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_index", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_metrics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::GlyphMetrics> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::GlyphMetrics = __cordl_object
            .invoke("get_metrics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scale", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_atlasIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlasIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_glyphRect(
        &mut self,
        value: crate::UnityEngine::TextCore::GlyphRect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_glyphRect", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_index(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_index", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_metrics(
        &mut self,
        value: crate::UnityEngine::TextCore::GlyphMetrics,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_metrics", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Glyph")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextCore::Glyph {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
