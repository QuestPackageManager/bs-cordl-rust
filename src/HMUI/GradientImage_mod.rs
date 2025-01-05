#[cfg(feature = "HMUI+GradientImage")]
#[repr(C)]
#[derive(Debug)]
pub struct GradientImage {
    __cordl_parent: crate::UnityEngine::UI::Image,
    pub _color0: crate::UnityEngine::Color,
    pub _color1: crate::UnityEngine::Color,
    pub _curvedCanvasSettingsHelper: quest_hook::libil2cpp::Gc<
        crate::HMUI::CurvedCanvasSettingsHelper,
    >,
}
#[cfg(feature = "HMUI+GradientImage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::GradientImage => "HMUI"."GradientImage"
);
#[cfg(feature = "HMUI+GradientImage")]
impl std::ops::Deref for crate::HMUI::GradientImage {
    type Target = crate::UnityEngine::UI::Image;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+GradientImage")]
impl std::ops::DerefMut for crate::HMUI::GradientImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+GradientImage")]
impl crate::HMUI::GradientImage {
    pub fn AddQuad_Il2CppArray_Color32_Il2CppArray0(
        vertexHelper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        quadPositions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        color: crate::UnityEngine::Color32,
        quadUVs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddQuad", (vertexHelper, quadPositions, color, quadUVs))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddQuad_Vector2_Vector2_Color32_Color32_Vector2_Vector2_f32_f32_2(
        vertexHelper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        posMin: crate::UnityEngine::Vector2,
        posMax: crate::UnityEngine::Vector2,
        color0: crate::UnityEngine::Color32,
        color1: crate::UnityEngine::Color32,
        uv0Min: crate::UnityEngine::Vector2,
        uv0Max: crate::UnityEngine::Vector2,
        elementWidthScale: f32,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddQuad",
                (
                    vertexHelper,
                    posMin,
                    posMax,
                    color0,
                    color1,
                    uv0Min,
                    uv0Max,
                    elementWidthScale,
                    curvedUIRadius,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddQuad_Vector2_Vector2_Color32_Vector2_Vector2_1(
        vertexHelper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        posMin: crate::UnityEngine::Vector2,
        posMax: crate::UnityEngine::Vector2,
        color: crate::UnityEngine::Color32,
        uvMin: crate::UnityEngine::Vector2,
        uvMax: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddQuad", (vertexHelper, posMin, posMax, color, uvMin, uvMax))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateFilledSprite(
        &mut self,
        toFill: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        preserveAspect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateFilledSprite", (toFill, preserveAspect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSimpleSprite(
        &mut self,
        vh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        lPreserveAspect: bool,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSimpleSprite", (vh, lPreserveAspect, curvedUIRadius))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSlicedSprite(
        &mut self,
        vh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSlicedSprite", (vh, curvedUIRadius))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTiledSprite(
        &mut self,
        toFill: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTiledSprite", (toFill))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdjustedBorders(
        &mut self,
        border: crate::UnityEngine::Vector4,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetAdjustedBorders", (border, rect))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPopulateMesh(
        &mut self,
        toFill: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPopulateMesh", (toFill))?;
        Ok(__cordl_ret.into())
    }
    pub fn RadialCut_Il2CppArray0(
        xy: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        uv: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        fill: f32,
        invert: bool,
        corner: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RadialCut", (xy, uv, fill, invert, corner))?;
        Ok(__cordl_ret.into())
    }
    pub fn RadialCut_f32_1(
        xy: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        cos: f32,
        sin: f32,
        invert: bool,
        corner: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RadialCut", (xy, cos, sin, invert, corner))?;
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
    pub fn get_color0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color0(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color0", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color1(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color1", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+GradientImage")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::GradientImage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
