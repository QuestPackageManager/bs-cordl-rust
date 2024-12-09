#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PointerModel {
    pub changedThisFrame: bool,
    pub leftButton: crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState,
    pub rightButton: crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState,
    pub middleButton: crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState,
    pub eventData: *mut crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
    pub m_ScreenPosition: crate::UnityEngine::Vector2,
    pub m_ScrollDelta: crate::UnityEngine::Vector2,
    pub m_WorldPosition: crate::UnityEngine::Vector3,
    pub m_WorldOrientation: crate::UnityEngine::Quaternion,
    pub m_Pressure: f32,
    pub m_AzimuthAngle: f32,
    pub m_AltitudeAngle: f32,
    pub m_Twist: f32,
    pub m_Radius: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::UI::PointerModel =>
    "UnityEngine.InputSystem.UI"."PointerModel"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::UI::PointerModel {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel")]
impl crate::UnityEngine::InputSystem::UI::PointerModel {
    #[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel+ButtonState")]
    pub type ButtonState = crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState;
    pub fn CopyTouchOrPenStateFrom(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyTouchOrPenStateFrom",
            (eventData),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OnFrameFinished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OnFrameFinished",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        eventData: *mut crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (eventData),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_altitudeAngle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_altitudeAngle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_azimuthAngle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_azimuthAngle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_pointerType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::UI::UIPointerType,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::UI::UIPointerType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pointerType",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_pressure(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressure",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_radius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_radius",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_screenPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_screenPosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_scrollDelta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_scrollDelta",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_twist(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_twist",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_worldOrientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_worldOrientation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_worldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_worldPosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_altitudeAngle(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_altitudeAngle",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_azimuthAngle(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_azimuthAngle",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_pressure(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pressure",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_radius(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_radius",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_screenPosition(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_screenPosition",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_scrollDelta(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_scrollDelta",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_twist(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_twist",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_worldOrientation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_worldOrientation",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_worldPosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_worldPosition",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel+ButtonState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PointerModel_ButtonState {
    pub m_IsPressed: bool,
    pub m_FramePressState: crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
    pub m_PressTime: f32,
    pub m_PressRaycast: crate::UnityEngine::EventSystems::RaycastResult,
    pub m_PressObject: *mut crate::UnityEngine::GameObject,
    pub m_RawPressObject: *mut crate::UnityEngine::GameObject,
    pub m_LastPressObject: *mut crate::UnityEngine::GameObject,
    pub m_DragObject: *mut crate::UnityEngine::GameObject,
    pub m_PressPosition: crate::UnityEngine::Vector2,
    pub m_ClickTime: f32,
    pub m_ClickCount: i32,
    pub m_Dragging: bool,
    pub m_ClickedOnSameGameObject: bool,
    pub m_IgnoreNextClick: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel+ButtonState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::PointerModel_ButtonState =>
    "UnityEngine.InputSystem.UI"."PointerModel/ButtonState"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel+ButtonState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+PointerModel+ButtonState")]
impl crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState {
    pub fn CopyPressStateFrom(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyPressStateFrom",
            (eventData),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CopyPressStateTo(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyPressStateTo",
            (eventData),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OnEndFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OnEndFrame",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_clickedOnSameGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_clickedOnSameGameObject",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ignoreNextClick(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ignoreNextClick",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isPressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isPressed",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_pressTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_wasPressedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wasPressedThisFrame",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_wasReleasedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wasReleasedThisFrame",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_clickedOnSameGameObject(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_clickedOnSameGameObject",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_ignoreNextClick(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ignoreNextClick",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isPressed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isPressed",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_pressTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pressTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
