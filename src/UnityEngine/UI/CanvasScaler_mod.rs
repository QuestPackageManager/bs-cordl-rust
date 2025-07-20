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
    pub m_Canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub m_PrevScaleFactor: f32,
    pub m_PrevReferencePixelsPerUnit: f32,
    pub m_PresetInfoIsWorld: bool,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::CanvasScaler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "CanvasScaler";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("Canvas_preWillRenderCanvases")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "Canvas_preWillRenderCanvases", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Handle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "Handle", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleConstantPhysicalSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleConstantPhysicalSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "HandleConstantPhysicalSize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleConstantPixelSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleConstantPixelSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "HandleConstantPixelSize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleScaleWithScreenSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleScaleWithScreenSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "HandleScaleWithScreenSize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleWorldCanvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("HandleWorldCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "HandleWorldCanvas", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetReferencePixelsPerUnit(
        &mut self,
        referencePixelsPerUnit: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetReferencePixelsPerUnit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "SetReferencePixelsPerUnit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (referencePixelsPerUnit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetScaleFactor(
        &mut self,
        scaleFactor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("SetScaleFactor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "SetScaleFactor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (scaleFactor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSpriteDPI(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_defaultSpriteDPI")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_defaultSpriteDPI", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_dynamicPixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_dynamicPixelsPerUnit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_dynamicPixelsPerUnit", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_fallbackScreenDPI(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_fallbackScreenDPI")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_fallbackScreenDPI", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_matchWidthOrHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_matchWidthOrHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_matchWidthOrHeight", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_physicalUnit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::CanvasScaler_Unit> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UI::CanvasScaler_Unit,
                0usize,
            >("get_physicalUnit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_physicalUnit", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UI::CanvasScaler_Unit = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_referencePixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_referencePixelsPerUnit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_referencePixelsPerUnit", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_referenceResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("get_referenceResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_referenceResolution", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_scaleFactor(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_scaleFactor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_scaleFactor", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_screenMatchMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode,
                0usize,
            >("get_screenMatchMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_screenMatchMode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_uiScaleMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::CanvasScaler_ScaleMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UI::CanvasScaler_ScaleMode,
                0usize,
            >("get_uiScaleMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "get_uiScaleMode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UI::CanvasScaler_ScaleMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultSpriteDPI(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_defaultSpriteDPI")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_defaultSpriteDPI", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_dynamicPixelsPerUnit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_dynamicPixelsPerUnit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_dynamicPixelsPerUnit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_fallbackScreenDPI(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_fallbackScreenDPI")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_fallbackScreenDPI", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_matchWidthOrHeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_matchWidthOrHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_matchWidthOrHeight", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_physicalUnit(
        &mut self,
        value: crate::UnityEngine::UI::CanvasScaler_Unit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UI::CanvasScaler_Unit),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_physicalUnit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_physicalUnit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_referencePixelsPerUnit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_referencePixelsPerUnit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_referencePixelsPerUnit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_referenceResolution(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_referenceResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_referenceResolution", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_scaleFactor(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_scaleFactor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_scaleFactor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_screenMatchMode(
        &mut self,
        value: crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_screenMatchMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_screenMatchMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_uiScaleMode(
        &mut self,
        value: crate::UnityEngine::UI::CanvasScaler_ScaleMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UI::CanvasScaler_ScaleMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_uiScaleMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::CanvasScaler as quest_hook::libil2cpp::Type
                    > ::class(), "set_uiScaleMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CanvasScaler_ScaleMode {
    #[default]
    ConstantPhysicalSize = 2i32,
    ConstantPixelSize = 0i32,
    ScaleWithScreenSize = 1i32,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::CanvasScaler_ScaleMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "CanvasScaler/ScaleMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UI::CanvasScaler_ScaleMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UI::CanvasScaler_ScaleMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UI::CanvasScaler_ScaleMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScaleMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UI::CanvasScaler_ScaleMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CanvasScaler_ScreenMatchMode {
    #[default]
    Expand = 1i32,
    MatchWidthOrHeight = 0i32,
    Shrink = 2i32,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "CanvasScaler/ScreenMatchMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+ScreenMatchMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UI::CanvasScaler_ScreenMatchMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CanvasScaler_Unit {
    #[default]
    Centimeters = 0i32,
    Inches = 2i32,
    Millimeters = 1i32,
    Picas = 4i32,
    Points = 3i32,
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::CanvasScaler_Unit {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "CanvasScaler/Unit";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UI::CanvasScaler_Unit {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UI::CanvasScaler_Unit {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UI::CanvasScaler_Unit {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasScaler+Unit")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::UI::CanvasScaler_Unit {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
