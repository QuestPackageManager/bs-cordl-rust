#[cfg(feature = "HMUI+GradientImage")]
#[repr(C)]
#[derive(Debug)]
pub struct GradientImage {
    __cordl_parent: crate::UnityEngine::UI::Image,
    pub _color0: crate::UnityEngine::Color,
    pub _color1: crate::UnityEngine::Color,
    pub _curvedCanvasSettingsHelper: *mut crate::HMUI::CurvedCanvasSettingsHelper,
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
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetAdjustedBorders", (border, rect))?;
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
    pub fn get_color0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color0", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_color1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color1", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GenerateSimpleSprite(
        &mut self,
        vh: *mut crate::UnityEngine::UI::VertexHelper,
        lPreserveAspect: bool,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSimpleSprite", (vh, lPreserveAspect, curvedUIRadius))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSlicedSprite(
        &mut self,
        vh: *mut crate::UnityEngine::UI::VertexHelper,
        curvedUIRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateSlicedSprite", (vh, curvedUIRadius))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
