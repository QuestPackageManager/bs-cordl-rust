#[cfg(feature = "UnityEngine+UI+Image+FillMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Image_FillMethod {
    Horizontal = 0i32,
    Radial180 = 3i32,
    Radial360 = 4i32,
    Radial90 = 2i32,
    Vertical = 1i32,
}
#[cfg(feature = "UnityEngine+UI+Image+FillMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image_FillMethod =>
    "UnityEngine.UI"."Image/FillMethod"
);
#[cfg(feature = "UnityEngine+UI+Image")]
#[repr(C)]
#[derive(Debug)]
pub struct Image {
    __cordl_parent: crate::UnityEngine::UI::MaskableGraphic,
    pub m_Sprite: *mut crate::UnityEngine::Sprite,
    pub m_OverrideSprite: *mut crate::UnityEngine::Sprite,
    pub m_Type: crate::UnityEngine::UI::Image_Type,
    pub m_PreserveAspect: bool,
    pub m_FillCenter: bool,
    pub m_FillMethod: crate::UnityEngine::UI::Image_FillMethod,
    pub m_FillAmount: f32,
    pub m_FillClockwise: bool,
    pub m_FillOrigin: i32,
    pub m_AlphaHitTestMinimumThreshold: f32,
    pub m_Tracked: bool,
    pub m_UseSpriteMesh: bool,
    pub m_PixelsPerUnitMultiplier: f32,
    pub m_CachedReferencePixelsPerUnit: f32,
}
#[cfg(feature = "UnityEngine+UI+Image")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image => "UnityEngine.UI"
    ."Image"
);
#[cfg(feature = "UnityEngine+UI+Image")]
impl std::ops::Deref for crate::UnityEngine::UI::Image {
    type Target = crate::UnityEngine::UI::MaskableGraphic;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Image")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Image")]
impl crate::UnityEngine::UI::Image {
    #[cfg(feature = "UnityEngine+UI+Image+FillMethod")]
    pub type FillMethod = crate::UnityEngine::UI::Image_FillMethod;
    #[cfg(feature = "UnityEngine+UI+Image+Origin180")]
    pub type Origin180 = crate::UnityEngine::UI::Image_Origin180;
    #[cfg(feature = "UnityEngine+UI+Image+Origin360")]
    pub type Origin360 = crate::UnityEngine::UI::Image_Origin360;
    #[cfg(feature = "UnityEngine+UI+Image+Origin90")]
    pub type Origin90 = crate::UnityEngine::UI::Image_Origin90;
    #[cfg(feature = "UnityEngine+UI+Image+OriginHorizontal")]
    pub type OriginHorizontal = crate::UnityEngine::UI::Image_OriginHorizontal;
    #[cfg(feature = "UnityEngine+UI+Image+OriginVertical")]
    pub type OriginVertical = crate::UnityEngine::UI::Image_OriginVertical;
    #[cfg(feature = "UnityEngine+UI+Image+Type")]
    pub type Type = crate::UnityEngine::UI::Image_Type;
    pub fn CalculateLayoutInputHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputHorizontal", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateLayoutInputVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputVertical", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisableSpriteOptimizations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableSpriteOptimizations", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateFilledSprite(
        &mut self,
        toFill: *mut crate::UnityEngine::UI::VertexHelper,
        preserveAspect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateFilledSprite", (toFill, preserveAspect))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSimpleSprite(
        &mut self,
        vh: *mut crate::UnityEngine::UI::VertexHelper,
        lPreserveAspect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSimpleSprite", (vh, lPreserveAspect))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSlicedSprite(
        &mut self,
        toFill: *mut crate::UnityEngine::UI::VertexHelper,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSlicedSprite", (toFill))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSprite(
        &mut self,
        vh: *mut crate::UnityEngine::UI::VertexHelper,
        lPreserveAspect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSprite", (vh, lPreserveAspect))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTiledSprite(
        &mut self,
        toFill: *mut crate::UnityEngine::UI::VertexHelper,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTiledSprite", (toFill))?;
        Ok(__cordl_ret)
    }
    pub fn GetAdjustedBorders(
        &mut self,
        border: crate::UnityEngine::Vector4,
        adjustedRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetAdjustedBorders", (border, adjustedRect))?;
        Ok(__cordl_ret)
    }
    pub fn GetDrawingDimensions(
        &mut self,
        shouldPreserveAspect: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetDrawingDimensions", (shouldPreserveAspect))?;
        Ok(__cordl_ret)
    }
    pub fn IsRaycastLocationValid(
        &mut self,
        screenPoint: crate::UnityEngine::Vector2,
        eventCamera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsRaycastLocationValid", (screenPoint, eventCamera))?;
        Ok(__cordl_ret)
    }
    pub fn MapCoordinate(
        &mut self,
        local: crate::UnityEngine::Vector2,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("MapCoordinate", (local, rect))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterDeserialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnCanvasHierarchyChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCanvasHierarchyChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDidApplyAnimationProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDidApplyAnimationProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPopulateMesh(
        &mut self,
        toFill: *mut crate::UnityEngine::UI::VertexHelper,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPopulateMesh", (toFill))?;
        Ok(__cordl_ret)
    }
    pub fn PreserveSpriteAspectRatio(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        spriteSize: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreserveSpriteAspectRatio", (rect, spriteSize))?;
        Ok(__cordl_ret)
    }
    pub fn SetNativeSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNativeSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn TrackSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _set_sprite_g__ResetAlphaHitThresholdIfNeeded_11_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<set_sprite>g__ResetAlphaHitThresholdIfNeeded|11_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _set_sprite_g__SpriteSupportsAlphaHitTest_11_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<set_sprite>g__SpriteSupportsAlphaHitTest|11_1", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_activeSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_activeSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_alphaHitTestMinimumThreshold(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_alphaHitTestMinimumThreshold", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eventAlphaThreshold(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_eventAlphaThreshold", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fillAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fillAmount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fillCenter(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fillCenter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fillClockwise(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fillClockwise", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fillMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::Image_FillMethod> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::Image_FillMethod = __cordl_object
            .invoke("get_fillMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fillOrigin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_fillOrigin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexibleHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexibleWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasBorder(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasBorder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layoutPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layoutPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mainTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("get_mainTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_material(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_material", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multipliedPixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_multipliedPixelsPerUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overrideSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_overrideSprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pixelsPerUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pixelsPerUnitMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pixelsPerUnitMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preserveAspect(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_preserveAspect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_sprite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::Image_Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::Image_Type = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useSpriteMesh(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useSpriteMesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_alphaHitTestMinimumThreshold(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alphaHitTestMinimumThreshold", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_eventAlphaThreshold(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventAlphaThreshold", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fillAmount(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fillAmount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fillCenter(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fillCenter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fillClockwise(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fillClockwise", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fillMethod(
        &mut self,
        value: crate::UnityEngine::UI::Image_FillMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fillMethod", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fillOrigin(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fillOrigin", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_material(
        &mut self,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_material", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_overrideSprite(
        &mut self,
        value: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideSprite", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pixelsPerUnitMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pixelsPerUnitMultiplier", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_preserveAspect(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_preserveAspect", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sprite(
        &mut self,
        value: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sprite", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_type(
        &mut self,
        value: crate::UnityEngine::UI::Image_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_type", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useSpriteMesh(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useSpriteMesh", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+Image")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Image {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Image+Origin180")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Image_Origin180 {
    Bottom = 0i32,
    Left = 1i32,
    Right = 3i32,
    Top = 2i32,
}
#[cfg(feature = "UnityEngine+UI+Image+Origin180")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image_Origin180 =>
    "UnityEngine.UI"."Image/Origin180"
);
#[cfg(feature = "UnityEngine+UI+Image+Origin360")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Image_Origin360 {
    Bottom = 0i32,
    Left = 3i32,
    Right = 1i32,
    Top = 2i32,
}
#[cfg(feature = "UnityEngine+UI+Image+Origin360")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image_Origin360 =>
    "UnityEngine.UI"."Image/Origin360"
);
#[cfg(feature = "UnityEngine+UI+Image+Origin90")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Image_Origin90 {
    BottomLeft = 0i32,
    BottomRight = 3i32,
    TopLeft = 1i32,
    TopRight = 2i32,
}
#[cfg(feature = "UnityEngine+UI+Image+Origin90")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image_Origin90 =>
    "UnityEngine.UI"."Image/Origin90"
);
#[cfg(feature = "UnityEngine+UI+Image+OriginHorizontal")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Image_OriginHorizontal {
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "UnityEngine+UI+Image+OriginHorizontal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image_OriginHorizontal =>
    "UnityEngine.UI"."Image/OriginHorizontal"
);
#[cfg(feature = "UnityEngine+UI+Image+OriginVertical")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Image_OriginVertical {
    Bottom = 0i32,
    Top = 1i32,
}
#[cfg(feature = "UnityEngine+UI+Image+OriginVertical")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image_OriginVertical =>
    "UnityEngine.UI"."Image/OriginVertical"
);
#[cfg(feature = "UnityEngine+UI+Image+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Image_Type {
    Filled = 3i32,
    Simple = 0i32,
    Sliced = 1i32,
    Tiled = 2i32,
}
#[cfg(feature = "UnityEngine+UI+Image+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Image_Type => "UnityEngine.UI"
    ."Image/Type"
);
