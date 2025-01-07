#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Touch {
    pub m_Finger: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
    >,
    pub m_TouchRecord: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
        crate::UnityEngine::InputSystem::LowLevel::TouchState,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "Touch";
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
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
    #[cfg(
        feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState"
    )]
    pub type ExtraDataPerTouchState = crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState;
    #[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
    pub type FingerAndTouchState = crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState;
    #[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
    pub type GlobalState = crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState;
    pub fn AddTouchscreen(
        screen: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTouchscreen", (screen))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginUpdate() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGlobalState() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateGlobalState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveTouchscreen(
        screen: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveTouchscreen", (screen))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAndResetState() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::ISavedState,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::ISavedState,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAndResetState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        finger: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
        >,
        touchRecord: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            crate::UnityEngine::InputSystem::LowLevel::TouchState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (finger, touchRecord),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onFingerDown(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onFingerDown", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onFingerMove(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onFingerMove", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onFingerUp(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onFingerUp", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeFingers() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_activeFingers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeTouches() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_activeTouches", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_began(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_began",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_delta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_delta",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displayIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_displayIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ended(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ended",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_finger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::EnhancedTouch::Finger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_finger", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fingers() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_fingers", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_inProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_inProgress",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isInProgress",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isTap(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isTap",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxHistoryLengthPerFinger() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxHistoryLengthPerFinger", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::TouchPhase> {
        let __cordl_ret: crate::UnityEngine::InputSystem::TouchPhase = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_phase",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressure(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressure",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_radius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_radius",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Touchscreen,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_screen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screenPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_screenPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screens() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_screens", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startScreenPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startScreenPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_startTime",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_tapCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_tapCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_time",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_touchId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uniqueId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_uniqueId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateStepCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_updateStepCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onFingerDown(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onFingerDown", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onFingerMove(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onFingerMove", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onFingerUp(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onFingerUp", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::EnhancedTouch::Touch>,
> for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::EnhancedTouch::Touch>,
> for crate::UnityEngine::InputSystem::EnhancedTouch::Touch {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Touch_ExtraDataPerTouchState {
    pub accumulatedDelta: crate::UnityEngine::Vector2,
    pub uniqueId: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "ExtraDataPerTouchState";
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+ExtraDataPerTouchState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_ExtraDataPerTouchState {
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Touch_FingerAndTouchState {
    pub updateMask: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    pub fingers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
            >,
        >,
    >,
    pub activeFingers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
            >,
        >,
    >,
    pub activeTouches: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
        >,
    >,
    pub activeFingerCount: i32,
    pub activeTouchCount: i32,
    pub totalFingerCount: i32,
    pub lastId: u32,
    pub haveBuiltActiveTouches: bool,
    pub haveActiveTouchesNeedingRefreshNextUpdate: bool,
    pub activeTouchState: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<
            crate::UnityEngine::InputSystem::LowLevel::TouchState,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "FingerAndTouchState";
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+FingerAndTouchState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState {
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
    pub fn AddFingers(
        &mut self,
        screen: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddFingers",
            (screen),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Destroy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFingers(
        &mut self,
        screen: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveFingers",
            (screen),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateActiveFingers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UpdateActiveFingers",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateActiveTouches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UpdateActiveTouches",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Touch_GlobalState {
    pub touchscreens: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    >,
    pub historyLengthPerFinger: i32,
    pub onFingerDown: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    >,
    pub onFingerMove: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    >,
    pub onFingerUp: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::EnhancedTouch::Finger,
                >,
            >,
        >,
    >,
    pub playerState: crate::UnityEngine::InputSystem::EnhancedTouch::Touch_FingerAndTouchState,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "GlobalState";
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState {
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Touch+GlobalState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::EnhancedTouch::Touch_GlobalState {
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
