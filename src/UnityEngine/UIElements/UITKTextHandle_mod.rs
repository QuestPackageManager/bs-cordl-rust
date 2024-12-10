#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct UITKTextHandle {
    __cordl_parent: crate::UnityEngine::TextCore::Text::TextHandle,
    pub _MeasuredSizes_k__BackingField: crate::UnityEngine::Vector2,
    pub _RoundedSizes_k__BackingField: crate::UnityEngine::Vector2,
    pub m_TextElement: *mut crate::UnityEngine::UIElements::TextElement,
    pub isOverridingCursor: bool,
    pub currentLinkIDHash: i32,
    pub hasLinkTag: bool,
    pub hasATag: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UITKTextHandle =>
    "UnityEngine.UIElements"."UITKTextHandle"
);
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UITKTextHandle {
    type Target = crate::UnityEngine::TextCore::Text::TextHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UITKTextHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl crate::UnityEngine::UIElements::UITKTextHandle {
    pub fn ATagOnPointerMove(
        &mut self,
        pme: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerMoveEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ATagOnPointerMove", (pme))?;
        Ok(__cordl_ret.into())
    }
    pub fn ATagOnPointerOut(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerOutEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ATagOnPointerOut", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn ATagOnPointerOver(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerOverEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ATagOnPointerOver", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn ATagOnPointerUp(
        &mut self,
        pue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ATagOnPointerUp", (pue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTextHeight(
        &mut self,
        textToMeasure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ComputeTextHeight", (textToMeasure, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTextWidth(
        &mut self,
        textToMeasure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        wordWrap: bool,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ComputeTextWidth", (textToMeasure, wordWrap, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertUssToTextGenerationSettings(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertUssToTextGenerationSettings", (tgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextEffectPadding(
        &mut self,
        fontAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetTextEffectPadding", (fontAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextOverflowMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::TextOverflowMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextCore::Text::TextOverflowMode = __cordl_object
            .invoke("GetTextOverflowMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleATag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleATag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLinkTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLinkTag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerDown(
        &mut self,
        pde: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerDownEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LinkTagOnPointerDown", (pde))?;
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerMove(
        &mut self,
        pme: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerMoveEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LinkTagOnPointerMove", (pme))?;
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerOut(
        &mut self,
        poe: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerOutEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LinkTagOnPointerOut", (poe))?;
        Ok(__cordl_ret.into())
    }
    pub fn LinkTagOnPointerUp(
        &mut self,
        pue: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LinkTagOnPointerUp", (pue))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (te))?;
        Ok(__cordl_object.into())
    }
    pub fn TextLibraryCanElide(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TextLibraryCanElide", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = __cordl_object.invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (te))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MeasuredSizes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_MeasuredSizes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RoundedSizes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_RoundedSizes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MeasuredSizes(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MeasuredSizes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RoundedSizes(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RoundedSizes", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UITKTextHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
