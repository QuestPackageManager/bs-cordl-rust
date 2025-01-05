#[cfg(feature = "UnityEngine+UIElements+Image")]
#[repr(C)]
#[derive(Debug)]
pub struct Image {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_ScaleMode: crate::UnityEngine::ScaleMode,
    pub m_Image: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub m_Sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_VectorImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VectorImage,
    >,
    pub m_UV: crate::UnityEngine::Rect,
    pub m_TintColor: crate::UnityEngine::Color,
    pub m_ImageIsInline: bool,
    pub m_ScaleModeIsInline: bool,
    pub m_TintColorIsInline: bool,
}
#[cfg(feature = "UnityEngine+UIElements+Image")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Image =>
    "UnityEngine.UIElements"."Image"
);
#[cfg(feature = "UnityEngine+UIElements+Image")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Image {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image")]
impl crate::UnityEngine::UIElements::Image {
    #[cfg(feature = "UnityEngine+UIElements+Image+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::Image_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+Image+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::Image_UxmlTraits;
    pub fn ClearProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearProperty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoMeasure(
        &mut self,
        desiredWidth: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        desiredHeight: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("DoMeasure", (desiredWidth, widthMode, desiredHeight, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSourceRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetSourceRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureDisplaySize_Sprite1(
        &mut self,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureDisplaySize", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureDisplaySize_Texture0(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureDisplaySize", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCustomStyleResolved(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CustomStyleResolvedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCustomStyleResolved", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnGenerateVisualContent(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGenerateVisualContent", (mgc))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCustomProperties(
        &mut self,
        customStyleProvider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICustomStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadCustomProperties", (customStyleProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProperty<T0, T1, T2>(
        &mut self,
        src: T0,
        dst: quest_hook::libil2cpp::ByRefMut<T0>,
        alt0: quest_hook::libil2cpp::ByRefMut<T1>,
        alt1: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProperty", (src, dst, alt0, alt1))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetScaleMode(
        &mut self,
        mode: crate::UnityEngine::ScaleMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetScaleMode", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTintColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTintColor", (color))?;
        Ok(__cordl_ret.into())
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
    pub fn get_image(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = __cordl_object
            .invoke("get_image", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scaleMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ScaleMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ScaleMode = __cordl_object
            .invoke("get_scaleMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sourceRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_sourceRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = __cordl_object
            .invoke("get_sprite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tintColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_tintColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv(&mut self) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object.invoke("get_uv", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vectorImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VectorImage,
        > = __cordl_object.invoke("get_vectorImage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_image(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_image", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sprite", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vectorImage(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vectorImage", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Image {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Image_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Image>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Image_UxmlTraits>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Image_UxmlFactory =>
    "UnityEngine.UIElements"."Image/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Image_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Image>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Image_UxmlTraits>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Image_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlFactory")]
impl crate::UnityEngine::UIElements::Image_UxmlFactory {
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
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Image_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct Image_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Image_UxmlTraits =>
    "UnityEngine.UIElements"."Image/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Image_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::VisualElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Image_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlTraits")]
impl crate::UnityEngine::UIElements::Image_UxmlTraits {
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
}
#[cfg(feature = "UnityEngine+UIElements+Image+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Image_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
