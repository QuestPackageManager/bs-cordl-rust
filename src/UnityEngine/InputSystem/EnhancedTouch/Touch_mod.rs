#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Touch_ExtraDataPerTouchState {
    pub accumulatedDelta: crate::UnityEngine::Vector2,
    pub uniqueId: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState =>
    "UnityEngine.InputSystem.EnhancedTouch"."Touch/ExtraDataPerTouchState"
);
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState {}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Touch_FingerAndTouchState {
    pub updateMask: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    pub fingers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
    >,
    pub activeFingers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
    >,
    pub activeTouches: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    >,
    pub activeFingerCount: i32,
    pub activeTouchCount: i32,
    pub totalFingerCount: i32,
    pub lastId: u32,
    pub haveBuiltActiveTouches: bool,
    pub haveActiveTouchesNeedingRefreshNextUpdate: bool,
    pub activeTouchState: *mut crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<
        crate::UnityEngine::InputSystem::LowLevel::TouchState,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState =>
    "UnityEngine.InputSystem.EnhancedTouch"."Touch/FingerAndTouchState"
);
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState {
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Destroy",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateActiveFingers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UpdateActiveFingers",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddFingers(
        &mut self,
        screen: *mut crate::UnityEngine::InputSystem::Touchscreen,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddFingers",
            (screen),
        )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveFingers(
        &mut self,
        screen: *mut crate::UnityEngine::InputSystem::Touchscreen,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveFingers",
            (screen),
        )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateActiveTouches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UpdateActiveTouches",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Touch_GlobalState {
    pub touchscreens: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        *mut crate::UnityEngine::InputSystem::Touchscreen,
    >,
    pub historyLengthPerFinger: i32,
    pub onFingerDown: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
        >,
    >,
    pub onFingerMove: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
        >,
    >,
    pub onFingerUp: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
        >,
    >,
    pub playerState: crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState =>
    "UnityEngine.InputSystem.EnhancedTouch"."Touch/GlobalState"
);
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState {}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Touch {
    pub m_Finger: *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
    pub m_TouchRecord: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
        crate::UnityEngine::InputSystem::LowLevel::TouchState,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::EnhancedTouch::Touch
    => "UnityEngine.InputSystem.EnhancedTouch"."Touch"
);
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
    #[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
    pub type FingerAndTouchState = crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState;
    #[cfg(
        feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState"
    )]
    pub type ExtraDataPerTouchState = crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState;
    #[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+__c")]
    pub type __c = crate::UnityEngine::InputSystem::EnhancedTouch::Touch___c;
    #[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
    pub type GlobalState = crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState;
    pub fn get_startScreenPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startScreenPosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_finger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_finger",
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
    pub fn get_displayIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_displayIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_delta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_delta",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ended(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ended",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isTap(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isTap",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isInProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isInProgress",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_updateStepCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_updateStepCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_uniqueId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_uniqueId",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::TouchState,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::TouchState,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_state", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tapCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_tapCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Touch0(
        &mut self,
        other: crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_extraData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_extraData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_inProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_inProgress",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_began(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_began",
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
    pub fn get_touchId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_touchId",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::TouchPhase> {
        let __cordl_ret: crate::UnityEngine::InputSystem::TouchPhase = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_phase",
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
    pub fn get_screen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Touchscreen,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Touchscreen = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_screen",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        finger: *mut crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
        touchRecord: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            crate::UnityEngine::InputSystem::LowLevel::TouchState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (finger, touchRecord),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_time",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_history(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_history",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
