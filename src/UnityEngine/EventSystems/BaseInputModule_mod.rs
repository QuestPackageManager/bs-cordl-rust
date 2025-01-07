#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_RaycastResultCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::EventSystems::RaycastResult,
        >,
    >,
    pub m_SendPointerHoverToParent: bool,
    pub m_AxisEventData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::AxisEventData,
    >,
    pub m_EventSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::EventSystem,
    >,
    pub m_BaseEventData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseEventData,
    >,
    pub m_InputOverride: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseInput,
    >,
    pub m_DefaultInput: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseInput,
    >,
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::BaseInputModule {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "BaseInputModule";
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
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::BaseInputModule {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::BaseInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl crate::UnityEngine::EventSystems::BaseInputModule {
    pub fn ActivateModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateModule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertUIToolkitPointerId(
        &mut self,
        sourcePointerData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ConvertUIToolkitPointerId", (sourcePointerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeactivateModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateModule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineMoveDirection_f32_1(
        x: f32,
        y: f32,
        deadZone: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventSystems::MoveDirection> {
        let __cordl_ret: crate::UnityEngine::EventSystems::MoveDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetermineMoveDirection", (x, y, deadZone))?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineMoveDirection_f32_f32_0(
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventSystems::MoveDirection> {
        let __cordl_ret: crate::UnityEngine::EventSystems::MoveDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetermineMoveDirection", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindCommonRoot(
        g1: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        g2: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindCommonRoot", (g1, g2))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstRaycast(
        candidates: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::EventSystems::RaycastResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventSystems::RaycastResult> {
        let __cordl_ret: crate::UnityEngine::EventSystems::RaycastResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstRaycast", (candidates))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxisEventData(
        &mut self,
        x: f32,
        y: f32,
        moveDeadZone: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::AxisEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::AxisEventData,
        > = __cordl_object.invoke("GetAxisEventData", (x, y, moveDeadZone))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        > = __cordl_object.invoke("GetBaseEventData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePointerExitAndEnter(
        &mut self,
        currentPointerData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        newEnterTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePointerExitAndEnter", (currentPointerData, newEnterTarget))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsModuleSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsModuleSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPointerOverGameObject(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPointerOverGameObject", (pointerId))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
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
    pub fn Process(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldActivateModule(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldActivateModule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateModule", ())?;
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
    pub fn get_eventSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::EventSystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::EventSystem,
        > = __cordl_object.invoke("get_eventSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_input(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseInput>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseInput,
        > = __cordl_object.invoke("get_input", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inputOverride(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseInput>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseInput,
        > = __cordl_object.invoke("get_inputOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sendPointerHoverToParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_sendPointerHoverToParent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_inputOverride(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseInput>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inputOverride", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sendPointerHoverToParent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sendPointerHoverToParent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::BaseInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
