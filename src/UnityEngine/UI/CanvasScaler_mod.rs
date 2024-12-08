#[cfg(feature = "UnityEngine+UI+CanvasScaler")]
#[repr(C)]
#[derive(Debug)]
pub struct CanvasScaler {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_UiScaleMode: crate::UnityEngine::UI::CanvasScaler_ScaleMode,
    pub m_ReferencePixelsPerUnit: f32,
    pub m_ScaleFactor: f32,
    pub m_ReferenceResolution: crate::UnityEngine::Vector2,
    pub m_ScreenMatchMode: crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode,
    pub m_MatchWidthOrHeight: f32,
    pub m_PhysicalUnit: crate::UnityEngine::UI::CanvasScaler_Unit,
    pub m_FallbackScreenDPI: f32,
    pub m_DefaultSpriteDPI: f32,
    pub m_DynamicPixelsPerUnit: f32,
    pub m_Canvas: *mut crate::UnityEngine::Canvas,
    pub m_PrevScaleFactor: f32,
    pub m_PrevReferencePixelsPerUnit: f32,
    pub m_PresetInfoIsWorld: bool,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::CanvasScaler =>
    "UnityEngine.UI"."CanvasScaler"
);
#[cfg(feature = "UnityEngine+UI+CanvasScaler")]
impl std::ops::Deref for crate::UnityEngine::UI::CanvasScaler {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler")]
impl std::ops::DerefMut for crate::UnityEngine::UI::CanvasScaler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler")]
impl crate::UnityEngine::UI::CanvasScaler {
    pub const kLogBase: f32 = 2f32;
    #[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
    pub type ScaleMode = crate::UnityEngine::UI::CanvasScaler_ScaleMode;
    #[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
    pub type ScreenMatchMode = crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode;
    #[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
    pub type Unit = crate::UnityEngine::UI::CanvasScaler_Unit;
    pub fn Canvas_preWillRenderCanvases(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Canvas_preWillRenderCanvases", ())?;
        Ok(__cordl_ret)
    }
    pub fn Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Handle", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleConstantPhysicalSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConstantPhysicalSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleConstantPixelSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConstantPixelSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleScaleWithScreenSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScaleWithScreenSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleWorldCanvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleWorldCanvas", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn SetReferencePixelsPerUnit(
        &mut self,
        referencePixelsPerUnit: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReferencePixelsPerUnit", (referencePixelsPerUnit))?;
        Ok(__cordl_ret)
    }
    pub fn SetScaleFactor(
        &mut self,
        scaleFactor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetScaleFactor", (scaleFactor))?;
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
    pub fn get_defaultSpriteDPI(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultSpriteDPI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dynamicPixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_dynamicPixelsPerUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fallbackScreenDPI(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fallbackScreenDPI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_matchWidthOrHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_matchWidthOrHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_physicalUnit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::CanvasScaler_Unit> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::CanvasScaler_Unit = __cordl_object
            .invoke("get_physicalUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_referencePixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_referencePixelsPerUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_referenceResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_referenceResolution", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scaleFactor(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scaleFactor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenMatchMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode = __cordl_object
            .invoke("get_screenMatchMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_uiScaleMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::CanvasScaler_ScaleMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::CanvasScaler_ScaleMode = __cordl_object
            .invoke("get_uiScaleMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultSpriteDPI(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultSpriteDPI", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dynamicPixelsPerUnit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dynamicPixelsPerUnit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fallbackScreenDPI(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackScreenDPI", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_matchWidthOrHeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_matchWidthOrHeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_physicalUnit(
        &mut self,
        value: crate::UnityEngine::UI::CanvasScaler_Unit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_physicalUnit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_referencePixelsPerUnit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_referencePixelsPerUnit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_referenceResolution(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_referenceResolution", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_scaleFactor(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scaleFactor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_screenMatchMode(
        &mut self,
        value: crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_screenMatchMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_uiScaleMode(
        &mut self,
        value: crate::UnityEngine::UI::CanvasScaler_ScaleMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uiScaleMode", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::CanvasScaler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasScaler_ScaleMode {
    ConstantPhysicalSize = 2i32,
    ConstantPixelSize = 0i32,
    ScaleWithScreenSize = 1i32,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::CanvasScaler_ScaleMode =>
    "UnityEngine.UI"."CanvasScaler/ScaleMode"
);
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasScaler_ScreenMatchMode {
    Expand = 1i32,
    MatchWidthOrHeight = 0i32,
    Shrink = 2i32,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::CanvasScaler_ScreenMatchMode =>
    "UnityEngine.UI"."CanvasScaler/ScreenMatchMode"
);
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasScaler_Unit {
    Centimeters = 0i32,
    Inches = 2i32,
    Millimeters = 1i32,
    Picas = 4i32,
    Points = 3i32,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::CanvasScaler_Unit =>
    "UnityEngine.UI"."CanvasScaler/Unit"
);
