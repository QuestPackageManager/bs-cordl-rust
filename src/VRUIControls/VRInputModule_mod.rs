#[cfg(feature = "VRUIControls+VRInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct VRInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseInputModule,
    pub _vrPointer: quest_hook::libil2cpp::Gc<crate::VRUIControls::VRPointer>,
    pub _rumblePreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _hapticFeedbackManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticFeedbackManager,
    >,
    pub _useMouseForPressInput_k__BackingField: bool,
    pub onProcessMousePressEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub pointerDidClickEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
        >,
    >,
    pub _pointerData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
        >,
    >,
    pub _componentList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        >,
    >,
    pub _mouseState: quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseState>,
}
#[cfg(feature = "VRUIControls+VRInputModule")]
unsafe impl quest_hook::libil2cpp::Type for crate::VRUIControls::VRInputModule {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "VRUIControls";
    const CLASS_NAME: &'static str = "VRInputModule";
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
#[cfg(feature = "VRUIControls+VRInputModule")]
impl std::ops::Deref for crate::VRUIControls::VRInputModule {
    type Target = crate::UnityEngine::EventSystems::BaseInputModule;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl std::ops::DerefMut for crate::VRUIControls::VRInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl crate::VRUIControls::VRInputModule {
    pub const kMinPressValue: f32 = 0.9f32;
    pub const kMouseLeftId: i32 = -1i32;
    pub fn ClearSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearSelection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "ClearSelection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeselectIfSelectionChanged(
        &mut self,
        currentOverGo: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::BaseEventData,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DeselectIfSelectionChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "DeselectIfSelectionChanged", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (currentOverGo, pointerEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLastPointerEventData(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >,
                1usize,
            >("GetLastPointerEventData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "GetLastPointerEventData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        > = unsafe { method.invoke_unchecked(self, (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMousePointerEventData(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseState>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseState>,
                1usize,
            >("GetMousePointerEventData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "GetMousePointerEventData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseState> = unsafe {
            method.invoke_unchecked(self, (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPointerData(
        &mut self,
        id: i32,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
        >,
        create: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::PointerEventData,
                        >,
                    >,
                    bool,
                ),
                bool,
                3usize,
            >("GetPointerData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "GetPointerData", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (id, data, create))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePointerExitAndEnter(
        &mut self,
        currentPointerData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        newEnterTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::PointerEventData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandlePointerExitAndEnter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "HandlePointerExitAndEnter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (currentPointerData, newEnterTarget))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPointerOverGameObject(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("IsPointerOverGameObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "IsPointerOverGameObject", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pointerId))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Process(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Process")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "Process", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDrag(
        &mut self,
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "ProcessDrag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pointerEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMousePress(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseButtonEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseButtonEventData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessMousePress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "ProcessMousePress", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMove(
        &mut self,
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessMove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "ProcessMove", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pointerEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastComparer(
        lhs: crate::UnityEngine::EventSystems::RaycastResult,
        rhs: crate::UnityEngine::EventSystems::RaycastResult,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::EventSystems::RaycastResult,
                    crate::UnityEngine::EventSystems::RaycastResult,
                ),
                i32,
                2usize,
            >("RaycastComparer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "RaycastComparer", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (lhs, rhs))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendUpdateEventToSelectedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("SendUpdateEventToSelectedObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "SendUpdateEventToSelectedObject", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldStartDrag(
        &mut self,
        pressPos: crate::UnityEngine::Vector2,
        currentPos: crate::UnityEngine::Vector2,
        threshold: f32,
        useDragThreshold: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2, f32, bool),
                bool,
                4usize,
            >("ShouldStartDrag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "ShouldStartDrag", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pressPos, currentPos, threshold, useDragThreshold),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onProcessMousePressEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onProcessMousePressEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "add_onProcessMousePressEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_pointerDidClickEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::PointerEventData,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_pointerDidClickEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "add_pointerDidClickEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_useMouseForPressInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_useMouseForPressInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "get_useMouseForPressInput", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_vrPointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::VRUIControls::VRPointer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::VRUIControls::VRPointer>,
                0usize,
            >("get_vrPointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "get_vrPointer", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::VRUIControls::VRPointer> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onProcessMousePressEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onProcessMousePressEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "remove_onProcessMousePressEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_pointerDidClickEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::PointerEventData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::PointerEventData,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_pointerDidClickEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "remove_pointerDidClickEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_useMouseForPressInput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_useMouseForPressInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::VRInputModule as quest_hook::libil2cpp::Type >
                    ::class(), "set_useMouseForPressInput", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::VRInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl AsRef<crate::GlobalNamespace::IVRInputModule>
for crate::VRUIControls::VRInputModule {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVRInputModule {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl AsMut<crate::GlobalNamespace::IVRInputModule>
for crate::VRUIControls::VRInputModule {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVRInputModule {
        unsafe { std::mem::transmute(self) }
    }
}
