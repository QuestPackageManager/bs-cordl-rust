#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
#[repr(C)]
#[derive(Debug)]
pub struct InputAction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Type: crate::UnityEngine::InputSystem::InputActionType,
    pub m_ExpectedControlType: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Id: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Processors: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Interactions: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_SingletonActionBindings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_Flags: crate::UnityEngine::InputSystem::InputAction_ActionFlags,
    pub m_BindingMask: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_BindingsStartIndex: i32,
    pub m_BindingsCount: i32,
    pub m_ControlStartIndex: i32,
    pub m_ControlCount: i32,
    pub m_ActionIndexInState: i32,
    pub m_ActionMap: *mut crate::UnityEngine::InputSystem::InputActionMap,
    pub m_OnStarted: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    >,
    pub m_OnCanceled: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    >,
    pub m_OnPerformed: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputAction =>
    "UnityEngine.InputSystem"."InputAction"
);
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputAction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl crate::UnityEngine::InputSystem::InputAction {
    #[cfg(feature = "UnityEngine+InputSystem+InputAction+ActionFlags")]
    pub type ActionFlags = crate::UnityEngine::InputSystem::InputAction_ActionFlags;
    #[cfg(feature = "UnityEngine+InputSystem+InputAction+CallbackContext")]
    pub type CallbackContext = crate::UnityEngine::InputSystem::InputAction_CallbackContext;
    pub fn ActiveControlIsValid(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ActiveControlIsValid", (control))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindingIndexOnActionToBindingIndexOnMap(
        &mut self,
        indexOfBindingOnAction: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "BindingIndexOnActionToBindingIndexOnMap",
                (indexOfBindingOnAction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BindingIndexOnMapToBindingIndexOnAction(
        &mut self,
        indexOfBindingOnMap: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BindingIndexOnMapToBindingIndexOnAction", (indexOfBindingOnMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInternalActionMapForSingletonAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateInternalActionMapForSingletonAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindEffectiveBindingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object.invoke("FindEffectiveBindingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateActionMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = __cordl_object.invoke("GetOrCreateActionMap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeoutCompletionPercentage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetTimeoutCompletionPercentage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeSureIdIsInPlace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("MakeSureIdIsInPlace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_InputActionType_Il2CppString_Il2CppString_Il2CppString_Il2CppString1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::UnityEngine::InputSystem::InputActionType,
        binding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expectedControlType: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    name,
                    _cordl_type,
                    binding,
                    interactions,
                    processors,
                    expectedControlType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ReadValue<TValue>(&mut self) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("ReadValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueAsObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadValueAsObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestInitialStateCheckOnEnabledAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestInitialStateCheckOnEnabledAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.ICloneable.Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WasPerformedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WasPerformedThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WasPressedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WasPressedThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WasReleasedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WasReleasedThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_InputActionType_Il2CppString_Il2CppString_Il2CppString_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::UnityEngine::InputSystem::InputActionType,
        binding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expectedControlType: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    name,
                    _cordl_type,
                    binding,
                    interactions,
                    processors,
                    expectedControlType,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_canceled(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_canceled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_performed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_performed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_started(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_started", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actionMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = __cordl_object.invoke("get_actionMap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = __cordl_object.invoke("get_activeControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object.invoke("get_bindingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object.invoke("get_bindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        > = __cordl_object.invoke("get_controls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionState_TriggerState = __cordl_object
            .invoke("get_currentState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_expectedControlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_expectedControlType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_idDontGenerate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("get_idDontGenerate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_inProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interactions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_interactions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSingletonAction(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSingletonAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionPhase,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionPhase = __cordl_object
            .invoke("get_phase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_processors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_processors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triggered(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_triggered", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionType = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wantsInitialStateCheck(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wantsInitialStateCheck", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_canceled(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_canceled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_performed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_performed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_started(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_started", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bindingMask(
        &mut self,
        value: crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputBinding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bindingMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_expectedControlType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_expectedControlType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wantsInitialStateCheck(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_wantsInitialStateCheck", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::InputAction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl AsRef<crate::System::ICloneable> for crate::UnityEngine::InputSystem::InputAction {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl AsMut<crate::System::ICloneable> for crate::UnityEngine::InputSystem::InputAction {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::InputSystem::InputAction {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::InputSystem::InputAction {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction+ActionFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction_ActionFlags {
    WantsInitialStateCheck = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction+ActionFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputAction_ActionFlags => "UnityEngine.InputSystem"
    ."InputAction/ActionFlags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputAction+CallbackContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputAction_CallbackContext {
    pub m_State: *mut crate::UnityEngine::InputSystem::InputActionState,
    pub m_ActionIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction+CallbackContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputAction_CallbackContext => "UnityEngine.InputSystem"
    ."InputAction/CallbackContext"
);
#[cfg(feature = "UnityEngine+InputSystem+InputAction+CallbackContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputAction_CallbackContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputAction+CallbackContext")]
impl crate::UnityEngine::InputSystem::InputAction_CallbackContext {
    pub fn ReadValueAsButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValueAsButton",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueAsObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ReadValueAsObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_1<TValue>(&mut self) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_Il2CppObject_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (buffer, bufferSize),
        )?;
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
    pub fn get_action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_action", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actionIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_actionIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindingIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bindingIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_canceled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_control", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_controlIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_duration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interaction(&mut self) -> quest_hook::libil2cpp::Result<Blacklisted> {
        let __cordl_ret: Blacklisted = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_interaction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interactionIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_interactionIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_performed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_performed",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionPhase,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionPhase = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_phase",
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
    pub fn get_started(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_started",
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
    pub fn get_valueSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valueSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_valueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valueType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
