#[cfg(feature = "TMPro+KerningPair")]
#[repr(C)]
#[derive(Debug)]
pub struct KerningPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_FirstGlyph: u32,
    pub m_FirstGlyphAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
    pub m_SecondGlyph: u32,
    pub m_SecondGlyphAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
    pub xOffset: f32,
    pub m_IgnoreSpacingAdjustments: bool,
}
#[cfg(feature = "TMPro+KerningPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::KerningPair => "TMPro"."KerningPair"
);
#[cfg(feature = "TMPro+KerningPair")]
impl std::ops::Deref for crate::TMPro::KerningPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+KerningPair")]
impl std::ops::DerefMut for crate::TMPro::KerningPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+KerningPair")]
impl crate::TMPro::KerningPair {
    pub fn ConvertLegacyKerningData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertLegacyKerningData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_GlyphValueRecord_Legacy_u32_GlyphValueRecord_Legacy2(
        firstGlyph: u32,
        firstGlyphAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
        secondGlyph: u32,
        secondGlyphAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (firstGlyph, firstGlyphAdjustments, secondGlyph, secondGlyphAdjustments),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_u32_f32_1(
        left: u32,
        right: u32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (left, right, offset))?;
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
    pub fn _ctor_u32_GlyphValueRecord_Legacy_u32_GlyphValueRecord_Legacy2(
        &mut self,
        firstGlyph: u32,
        firstGlyphAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
        secondGlyph: u32,
        secondGlyphAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (firstGlyph, firstGlyphAdjustments, secondGlyph, secondGlyphAdjustments),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_u32_f32_1(
        &mut self,
        left: u32,
        right: u32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (left, right, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_firstGlyph(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_firstGlyph", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_firstGlyphAdjustments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::GlyphValueRecord_Legacy> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::GlyphValueRecord_Legacy = __cordl_object
            .invoke("get_firstGlyphAdjustments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ignoreSpacingAdjustments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ignoreSpacingAdjustments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_secondGlyph(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_secondGlyph", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_secondGlyphAdjustments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::GlyphValueRecord_Legacy> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::GlyphValueRecord_Legacy = __cordl_object
            .invoke("get_secondGlyphAdjustments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_firstGlyph(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_firstGlyph", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_secondGlyph(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_secondGlyph", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+KerningPair")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::KerningPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
