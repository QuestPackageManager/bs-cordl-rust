#[cfg(feature = "HMUI+ImageView")]
#[repr(C)]
#[derive(Debug)]
pub struct ImageView {
    __cordl_parent: crate::HMUI::ImageViewBase,
    pub _useScriptableObjectColors: bool,
    pub _colorSo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _color0So: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _color1So: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _skew: f32,
    pub _gradient: bool,
    pub _color0: crate::UnityEngine::Color,
    pub _color1: crate::UnityEngine::Color,
    pub _gradientDirection: crate::HMUI::ImageView_GradientDirection,
    pub _flipGradientColors: bool,
    pub _curvedCanvasSettingsHelper: quest_hook::libil2cpp::Gc<
        crate::HMUI::CurvedCanvasSettingsHelper,
    >,
}
#[cfg(feature = "HMUI+ImageView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ImageView => "HMUI"."ImageView"
);
#[cfg(feature = "HMUI+ImageView")]
impl std::ops::Deref for crate::HMUI::ImageView {
    type Target = crate::HMUI::ImageViewBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ImageView")]
impl std::ops::DerefMut for crate::HMUI::ImageView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ImageView")]
impl crate::HMUI::ImageView {
    #[cfg(feature = "HMUI+ImageView+GradientDirection")]
    pub type GradientDirection = crate::HMUI::ImageView_GradientDirection;
    pub fn AddQuadWithHorizontalGradient(
        vertexHelper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        posMin: crate::UnityEngine::Vector2,
        posMax: crate::UnityEngine::Vector2,
        color0: crate::UnityEngine::Color32,
        color1: crate::UnityEngine::Color32,
        uv0Min: crate::UnityEngine::Vector2,
        uv0Max: crate::UnityEngine::Vector2,
        uv1Min: crate::UnityEngine::Vector2,
        uv1Max: crate::UnityEngine::Vector2,
        elementWidthScale: f32,
        curvedUIRadius: f32,
        skewFactor: f32,
        skewOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddQuadWithHorizontalGradient",
                (
                    vertexHelper,
                    posMin,
                    posMax,
                    color0,
                    color1,
                    uv0Min,
                    uv0Max,
                    uv1Min,
                    uv1Max,
                    elementWidthScale,
                    curvedUIRadius,
                    skewFactor,
                    skewOffset,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddQuadWithVerticalGradient(
        vertexHelper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        posMin: crate::UnityEngine::Vector2,
        posMax: crate::UnityEngine::Vector2,
        color0: crate::UnityEngine::Color32,
        color1: crate::UnityEngine::Color32,
        uv0Min: crate::UnityEngine::Vector2,
        uv0Max: crate::UnityEngine::Vector2,
        uv1Min: crate::UnityEngine::Vector2,
        uv1Max: crate::UnityEngine::Vector2,
        elementWidthScale: f32,
        curvedUIRadius: f32,
        skewFactor: f32,
        skewOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddQuadWithVerticalGradient",
                (
                    vertexHelper,
                    posMin,
                    posMax,
                    color0,
                    color1,
                    uv0Min,
                    uv0Max,
                    uv1Min,
                    uv1Max,
                    elementWidthScale,
                    curvedUIRadius,
                    skewFactor,
                    skewOffset,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn AddQuad_Vector2_Vector2_Color32_Vector2_Vector2_Vector2_Vector2_f32_f32_f32_f32_2(
        vertexHelper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        posMin: crate::UnityEngine::Vector2,
        posMax: crate::UnityEngine::Vector2,
        color: crate::UnityEngine::Color32,
        uv0Min: crate::UnityEngine::Vector2,
        uv0Max: crate::UnityEngine::Vector2,
        uv1Min: crate::UnityEngine::Vector2,
        uv1Max: crate::UnityEngine::Vector2,
        elementWidthScale: f32,
        curvedUIRadius: f32,
        skewFactor: f32,
        skewOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddQuad",
                (
                    vertexHelper,
                    posMin,
                    posMax,
                    color,
                    uv0Min,
                    uv0Max,
                    uv1Min,
                    uv1Max,
                    elementWidthScale,
                    curvedUIRadius,
                    skewFactor,
                    skewOffset,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddQuad_Vector2_Vector2_Color32_Vector2_Vector2_f32_f32_1(
        vertexHelper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        posMin: crate::UnityEngine::Vector2,
        posMax: crate::UnityEngine::Vector2,
        color: crate::UnityEngine::Color32,
        uvMin: crate::UnityEngine::Vector2,
        uvMax: crate::UnityEngine::Vector2,
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
                    color,
                    uvMin,
                    uvMax,
                    elementWidthScale,
                    curvedUIRadius,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateFilledSprite(
        &mut self,
        toFill: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        preserveAspect: bool,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateFilledSprite", (toFill, preserveAspect, curvedUIRadius))?;
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
        toFill: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSlicedSprite", (toFill, curvedUIRadius))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTiledSprite(
        &mut self,
        toFill: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::VertexHelper>,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTiledSprite", (toFill, curvedUIRadius))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdjustedBorders(
        border: crate::UnityEngine::Vector4,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
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
    pub fn GetNumberOfElements(
        curvedUIRadius: f32,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNumberOfElements", (curvedUIRadius, width))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
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
    pub fn __Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__Refresh", ())?;
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
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
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
    pub fn get_gradient(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_gradient", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_skew(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_skew", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useScriptableObjectColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useScriptableObjectColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
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
    pub fn set_gradient(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gradient", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useScriptableObjectColors(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useScriptableObjectColors", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ImageView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ImageView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ImageView")]
impl AsRef<crate::GlobalNamespace::IComponentRefresher> for crate::HMUI::ImageView {
    fn as_ref(&self) -> &crate::GlobalNamespace::IComponentRefresher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+ImageView")]
impl AsMut<crate::GlobalNamespace::IComponentRefresher> for crate::HMUI::ImageView {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IComponentRefresher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+ImageView+GradientDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageView_GradientDirection {
    Horizontal = 0i32,
    Vertical = 1i32,
}
#[cfg(feature = "HMUI+ImageView+GradientDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ImageView_GradientDirection => "HMUI"
    ."ImageView/GradientDirection"
);
