#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingState_InputActionState_Flags {
    ChainsWithNext = 1i32,
    Composite = 4i32,
    EndOfChain = 2i32,
    InitialStateCheckPending = 16i32,
    PartOfComposite = 8i32,
    WantsInitialStateCheck = 32i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::BindingState_InputActionState_Flags =>
    "UnityEngine.InputSystem"."InputActionState/BindingState/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub maps: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputActionMap,
    >,
    pub controls: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub interactions: *mut quest_hook::libil2cpp::Il2CppArray<Blacklisted>,
    pub processors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputProcessor,
    >,
    pub composites: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputBindingComposite,
    >,
    pub totalProcessorCount: i32,
    pub memory: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
    pub m_OnBeforeUpdateHooked: bool,
    pub m_OnAfterUpdateHooked: bool,
    pub m_InProcessControlStateChange: bool,
    pub m_CurrentlyProcessingThisEvent: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    pub m_OnBeforeUpdateDelegate: *mut crate::System::Action,
    pub m_OnAfterUpdateDelegate: *mut crate::System::Action,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputActionState =>
    "UnityEngine.InputSystem"."InputActionState"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputActionState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputActionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl crate::UnityEngine::InputSystem::InputActionState {
    pub const kInvalidIndex: i32 = -1i32;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
    pub type ActionMapIndices = crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
    pub type BindingState = crate::UnityEngine::InputSystem::InputActionState_BindingState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
    pub type GlobalState = crate::UnityEngine::InputSystem::InputActionState_GlobalState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
    pub type InteractionState = crate::UnityEngine::InputSystem::InputActionState_InteractionState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
    pub type TriggerState = crate::UnityEngine::InputSystem::InputActionState_TriggerState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
    pub type UnmanagedMemory = crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+__c")]
    pub type __c = crate::UnityEngine::InputSystem::InputActionState___c;
    pub fn AddToGlobalList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToGlobalList", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyProcessors<TValue>(
        &mut self,
        bindingIndex: i32,
        value: TValue,
        controlOfType: *mut crate::UnityEngine::InputSystem::InputControl_1<TValue>,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke("ApplyProcessors", (bindingIndex, value, controlOfType))?;
        Ok(__cordl_ret)
    }
    pub fn CallActionListeners(
        &mut self,
        actionIndex: i32,
        actionMap: *mut crate::UnityEngine::InputSystem::InputActionMap,
        phase: crate::UnityEngine::InputSystem::InputActionPhase,
        listeners: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                *mut crate::System::Action_1<
                    crate::UnityEngine::InputSystem::InputAction_CallbackContext,
                >,
            >,
        >,
        callbackName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CallActionListeners",
                (actionIndex, actionMap, phase, listeners, callbackName),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CanUseDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanUseDevice", (device))?;
        Ok(__cordl_ret)
    }
    pub fn ChangePhaseOfAction(
        &mut self,
        newPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        phaseAfterPerformedOrCanceled: crate::UnityEngine::InputSystem::InputActionPhase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ChangePhaseOfAction",
                (newPhase, trigger, phaseAfterPerformedOrCanceled),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ChangePhaseOfActionInternal(
        &mut self,
        actionIndex: i32,
        actionState: *mut quest_hook::libil2cpp::Il2CppObject,
        newPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ChangePhaseOfActionInternal",
                (actionIndex, actionState, newPhase, trigger),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ChangePhaseOfInteraction(
        &mut self,
        newPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        phaseAfterPerformed: crate::UnityEngine::InputSystem::InputActionPhase,
        processNextInteractionOnCancel: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ChangePhaseOfInteraction",
                (newPhase, trigger, phaseAfterPerformed, processNextInteractionOnCancel),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ClaimDataFrom(
        &mut self,
        resolver: crate::UnityEngine::InputSystem::InputBindingResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClaimDataFrom", (resolver))?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionState = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputeControlGroupingIfNecessary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeControlGroupingIfNecessary", ())?;
        Ok(__cordl_ret)
    }
    pub fn Destroy(
        &mut self,
        isFinalizing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", (isFinalizing))?;
        Ok(__cordl_ret)
    }
    pub fn DisableAllActions(
        &mut self,
        map: *mut crate::UnityEngine::InputSystem::InputActionMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableAllActions", (map))?;
        Ok(__cordl_ret)
    }
    pub fn DisableControls_InputAction1(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableControls", (action))?;
        Ok(__cordl_ret)
    }
    pub fn DisableControls_InputActionMap0(
        &mut self,
        map: *mut crate::UnityEngine::InputSystem::InputActionMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableControls", (map))?;
        Ok(__cordl_ret)
    }
    pub fn DisableControls_i32_i32_i32_2(
        &mut self,
        mapIndex: i32,
        controlStartIndex: i32,
        numControls: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableControls", (mapIndex, controlStartIndex, numControls))?;
        Ok(__cordl_ret)
    }
    pub fn DisableSingleAction(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableSingleAction", (action))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableAllActions(
        &mut self,
        map: *mut crate::UnityEngine::InputSystem::InputActionMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableAllActions", (map))?;
        Ok(__cordl_ret)
    }
    pub fn EnableControls_InputAction1(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableControls", (action))?;
        Ok(__cordl_ret)
    }
    pub fn EnableControls_InputActionMap0(
        &mut self,
        map: *mut crate::UnityEngine::InputSystem::InputActionMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableControls", (map))?;
        Ok(__cordl_ret)
    }
    pub fn EnableControls_i32_i32_i32_2(
        &mut self,
        mapIndex: i32,
        controlStartIndex: i32,
        numControls: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableControls", (mapIndex, controlStartIndex, numControls))?;
        Ok(__cordl_ret)
    }
    pub fn EnableSingleAction(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableSingleAction", (action))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateCompositePartMagnitude(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("EvaluateCompositePartMagnitude", (bindingIndex, partNumber))?;
        Ok(__cordl_ret)
    }
    pub fn FetchActionState(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        > = __cordl_object.invoke("FetchActionState", (action))?;
        Ok(__cordl_ret)
    }
    pub fn FetchMapIndices(
        &mut self,
        map: *mut crate::UnityEngine::InputSystem::InputActionMap,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices = __cordl_object
            .invoke("FetchMapIndices", (map))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindControlIndexOnBinding(
        &mut self,
        bindingIndex: i32,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindControlIndexOnBinding", (bindingIndex, control))?;
        Ok(__cordl_ret)
    }
    pub fn FinishBindingCompositeSetups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishBindingCompositeSetups", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishBindingResolution(
        &mut self,
        hasEnabledActions: bool,
        oldMemory: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
        activeControls: crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
        isFullResolve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "FinishBindingResolution",
                (hasEnabledActions, oldMemory, activeControls, isFullResolve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetActionBindingStartIndexAndCount(
        &mut self,
        actionIndex: i32,
        bindingCount: quest_hook::libil2cpp::ByRefMut<u16>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object
            .invoke("GetActionBindingStartIndexAndCount", (actionIndex, bindingCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetActionMap(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionMap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionMap = __cordl_object
            .invoke("GetActionMap", (bindingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetActionOrNoneString(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetActionOrNoneString", (trigger))?;
        Ok(__cordl_ret)
    }
    pub fn GetActionOrNull_ByRefMut1(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("GetActionOrNull", (trigger))?;
        Ok(__cordl_ret)
    }
    pub fn GetActionOrNull_i32_0(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("GetActionOrNull", (bindingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetBinding(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::InputSystem::InputBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object.invoke("GetBinding", (bindingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetBindingIndexInMap(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBindingIndexInMap", (bindingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetBindingIndexInState(
        &mut self,
        mapIndex: i32,
        bindingIndexInMap: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBindingIndexInState", (mapIndex, bindingIndexInMap))?;
        Ok(__cordl_ret)
    }
    pub fn GetBindingState(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_BindingState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_BindingState,
        > = __cordl_object.invoke("GetBindingState", (bindingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetCompositePartPressTime(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object
            .invoke("GetCompositePartPressTime", (bindingIndex, partNumber))?;
        Ok(__cordl_ret)
    }
    pub fn GetControl(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = __cordl_object
            .invoke("GetControl", (trigger))?;
        Ok(__cordl_ret)
    }
    pub fn GetInteractionOrNull(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<Blacklisted> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: Blacklisted = __cordl_object
            .invoke("GetInteractionOrNull", (trigger))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueSizeInBytes(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetValueSizeInBytes", (bindingIndex, controlIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueType(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetValueType", (bindingIndex, controlIndex))?;
        Ok(__cordl_ret)
    }
    pub fn HasEnabledActions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasEnabledActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn HookOnBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HookOnBeforeUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        resolver: crate::UnityEngine::InputSystem::InputBindingResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (resolver))?;
        Ok(__cordl_ret)
    }
    pub fn IsActionBoundToControlFromDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        actionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsActionBoundToControlFromDevice", (device, actionIndex))?;
        Ok(__cordl_ret)
    }
    pub fn IsActiveControl(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsActiveControl", (bindingIndex, controlIndex))?;
        Ok(__cordl_ret)
    }
    pub fn IsConflictingInput(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        actionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsConflictingInput", (trigger, actionIndex))?;
        Ok(__cordl_ret)
    }
    pub fn IsControlEnabled(
        &mut self,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsControlEnabled", (controlIndex))?;
        Ok(__cordl_ret)
    }
    pub fn IsUsingDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsUsingDevice", (device))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NotifyListenersOfActionChange(
        &mut self,
        change: crate::UnityEngine::InputSystem::InputActionChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyListenersOfActionChange", (change))?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeInitialUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeInitialUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn PrepareForBindingReResolution(
        &mut self,
        needFullResolve: bool,
        activeControls: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<
                *mut crate::UnityEngine::InputSystem::InputControl,
            >,
        >,
        hasEnabledActions: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PrepareForBindingReResolution",
                (needFullResolve, activeControls, hasEnabledActions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessButtonState(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        actionIndex: i32,
        bindingStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessButtonState", (trigger, actionIndex, bindingStatePtr))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessControlStateChange(
        &mut self,
        mapIndex: i32,
        controlIndex: i32,
        bindingIndex: i32,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessControlStateChange",
                (mapIndex, controlIndex, bindingIndex, _cordl_time, eventPtr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessDefaultInteraction(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        actionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDefaultInteraction", (trigger, actionIndex))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessInteractions(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        interactionStartIndex: i32,
        interactionCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessInteractions",
                (trigger, interactionStartIndex, interactionCount),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessTimeout(
        &mut self,
        _cordl_time: f64,
        mapIndex: i32,
        controlIndex: i32,
        bindingIndex: i32,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessTimeout",
                (_cordl_time, mapIndex, controlIndex, bindingIndex, interactionIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadCompositePartValueAsObject(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ReadCompositePartValueAsObject", (bindingIndex, partNumber))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCompositePartValue_ByRefMut_TComparer0<TValue, TComparer>(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
        buttonValuePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        controlIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        comparer: TComparer,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TComparer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke(
                "ReadCompositePartValue",
                (bindingIndex, partNumber, buttonValuePtr, controlIndex, comparer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadCompositePartValue_i32_1(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ReadCompositePartValue",
                (bindingIndex, partNumber, buffer, bufferSize),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValueAsButton(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadValueAsButton", (bindingIndex, controlIndex))?;
        Ok(__cordl_ret)
    }
    pub fn ReadValueAsObject(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
        ignoreComposites: bool,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke(
                "ReadValueAsObject",
                (bindingIndex, controlIndex, ignoreComposites),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue_Il2CppObject_i32__cordl_bool0(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferSize: i32,
        ignoreComposites: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReadValue",
                (bindingIndex, controlIndex, buffer, bufferSize, ignoreComposites),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue__cordl_bool1<TValue>(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
        ignoreComposites: bool,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke("ReadValue", (bindingIndex, controlIndex, ignoreComposites))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveMapFromGlobalList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveMapFromGlobalList", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetActionState(
        &mut self,
        actionIndex: i32,
        toPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        hardReset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetActionState", (actionIndex, toPhase, hardReset))?;
        Ok(__cordl_ret)
    }
    pub fn ResetActionStatesDrivenBy(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetActionStatesDrivenBy", (device))?;
        Ok(__cordl_ret)
    }
    pub fn ResetInteractionState(
        &mut self,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetInteractionState", (interactionIndex))?;
        Ok(__cordl_ret)
    }
    pub fn ResetInteractionStateAndCancelIfNecessary(
        &mut self,
        mapIndex: i32,
        bindingIndex: i32,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ResetInteractionStateAndCancelIfNecessary",
                (mapIndex, bindingIndex, interactionIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RestoreActionStatesAfterReResolvingBindings(
        &mut self,
        oldState: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
        activeControls: crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
        isFullResolve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RestoreActionStatesAfterReResolvingBindings",
                (oldState, activeControls, isFullResolve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetControlEnabled(
        &mut self,
        controlIndex: i32,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetControlEnabled", (controlIndex, state))?;
        Ok(__cordl_ret)
    }
    pub fn SetInitialStateCheckPending_Il2CppObject1(
        &mut self,
        bindingStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInitialStateCheckPending", (bindingStatePtr, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetInitialStateCheckPending_i32_0(
        &mut self,
        actionIndex: i32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInitialStateCheckPending", (actionIndex, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTotalTimeoutCompletionTime(
        &mut self,
        seconds: f32,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTotalTimeoutCompletionTime", (seconds, trigger))?;
        Ok(__cordl_ret)
    }
    pub fn SplitUpMapAndControlAndBindingIndex(
        &mut self,
        mapControlAndBindingIndex: i64,
        mapIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        controlIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        bindingIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SplitUpMapAndControlAndBindingIndex",
                (mapControlAndBindingIndex, mapIndex, controlIndex, bindingIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTimeout(
        &mut self,
        seconds: f32,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartTimeout", (seconds, trigger))?;
        Ok(__cordl_ret)
    }
    pub fn StopTimeout(
        &mut self,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopTimeout", (interactionIndex))?;
        Ok(__cordl_ret)
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("System.ICloneable.Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToCombinedMapAndControlAndBindingIndex(
        &mut self,
        mapIndex: i32,
        controlIndex: i32,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke(
                "ToCombinedMapAndControlAndBindingIndex",
                (mapIndex, controlIndex, bindingIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnhookOnBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnhookOnBeforeUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateChangeMonitor_NotifyControlStateChanged(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        mapControlAndBindingIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyControlStateChanged",
                (control, _cordl_time, eventPtr, mapControlAndBindingIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateChangeMonitor_NotifyTimerExpired(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        _cordl_time: f64,
        mapControlAndBindingIndex: i64,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyTimerExpired",
                (control, _cordl_time, mapControlAndBindingIndex, interactionIndex),
            )?;
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
    pub fn get_actionStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_actionStates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bindingStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_bindingStates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_controlGroupingAndComplexity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_controlGroupingAndComplexity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_controlIndexToBindingIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_controlIndexToBindingIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_controlMagnitudes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_controlMagnitudes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enabledControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_enabledControls", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_interactionStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_interactionStates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isProcessingControlStateChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isProcessingControlStateChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mapIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_mapIndices", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_totalActionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalActionCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_totalBindingCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalBindingCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_totalCompositeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalCompositeCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_totalControlCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalControlCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_totalInteractionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalInteractionCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_totalMapCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalMapCount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionState_ActionMapIndices {
    pub actionStartIndex: i32,
    pub actionCount: i32,
    pub controlStartIndex: i32,
    pub controlCount: i32,
    pub bindingStartIndex: i32,
    pub bindingCount: i32,
    pub interactionStartIndex: i32,
    pub interactionCount: i32,
    pub processorStartIndex: i32,
    pub processorCount: i32,
    pub compositeStartIndex: i32,
    pub compositeCount: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionState_ActionMapIndices =>
    "UnityEngine.InputSystem"."InputActionState/ActionMapIndices"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
impl crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionState_BindingState {
    padding: [u8; 32usize],
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionState_BindingState =>
    "UnityEngine.InputSystem"."InputActionState/BindingState"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_BindingState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
impl crate::UnityEngine::InputSystem::InputActionState_BindingState {
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags;
    pub fn get_actionIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_actionIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_chainsWithNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_chainsWithNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_compositeOrCompositeBindingIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_compositeOrCompositeBindingIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_controlCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_controlCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_controlStartIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_controlStartIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flags",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_initialStateCheckPending(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_initialStateCheckPending",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_interactionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_interactionCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_interactionStartIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_interactionStartIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isComposite",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isEndOfChain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isEndOfChain",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isPartOfChain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isPartOfChain",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isPartOfComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isPartOfComposite",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_mapIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mapIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_partIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_partIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_pressTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_processorCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_processorCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_processorStartIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_processorStartIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_triggerEventIdForComposite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_triggerEventIdForComposite",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_wantsInitialStateCheck(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wantsInitialStateCheck",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_actionIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_actionIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_chainsWithNext(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_chainsWithNext",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_compositeOrCompositeBindingIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_compositeOrCompositeBindingIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_controlCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_controlCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_controlStartIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_controlStartIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_flags(
        &mut self,
        value: crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flags",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_initialStateCheckPending(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_initialStateCheckPending",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_interactionCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_interactionCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_interactionStartIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_interactionStartIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isComposite(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isComposite",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isEndOfChain(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isEndOfChain",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isPartOfComposite(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isPartOfComposite",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_mapIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_mapIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_partIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_partIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_pressTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pressTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_processorCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_processorCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_processorStartIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_processorStartIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_triggerEventIdForComposite(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_triggerEventIdForComposite",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_wantsInitialStateCheck(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wantsInitialStateCheck",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionState_GlobalState {
    pub globalList: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::System::Runtime::InteropServices::GCHandle,
    >,
    pub onActionChange: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            crate::UnityEngine::InputSystem::InputActionChange,
        >,
    >,
    pub onActionControlsChanged: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionState_GlobalState => "UnityEngine.InputSystem"
    ."InputActionState/GlobalState"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_GlobalState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
impl crate::UnityEngine::InputSystem::InputActionState_GlobalState {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionState_InteractionState {
    padding: [u8; 48usize],
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionState_InteractionState =>
    "UnityEngine.InputSystem"."InputActionState/InteractionState"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_InteractionState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
impl crate::UnityEngine::InputSystem::InputActionState_InteractionState {
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::InteractionState_InputActionState_Flags;
    pub fn get_isTimerRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isTimerRunning",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_performedTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_performedTime",
            (),
        )?;
        Ok(__cordl_ret)
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
    pub fn get_timerDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_timerDuration",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_timerMonitorIndex(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_timerMonitorIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_timerStartTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_timerStartTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_totalTimeoutCompletionDone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalTimeoutCompletionDone",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_totalTimeoutCompletionTimeRemaining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalTimeoutCompletionTimeRemaining",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_triggerControlIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_triggerControlIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isTimerRunning(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isTimerRunning",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_performedTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_performedTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_phase(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_phase",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_startTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_timerDuration(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_timerDuration",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_timerMonitorIndex(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_timerMonitorIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_timerStartTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_timerStartTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_totalTimeoutCompletionDone(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_totalTimeoutCompletionDone",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_totalTimeoutCompletionTimeRemaining(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_totalTimeoutCompletionTimeRemaining",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_triggerControlIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_triggerControlIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionState_TriggerState {
    padding: [u8; 48usize],
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionState_TriggerState =>
    "UnityEngine.InputSystem"."InputActionState/TriggerState"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_TriggerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
impl crate::UnityEngine::InputSystem::InputActionState_TriggerState {
    pub const kMaxNumBindings: i32 = 65535i32;
    pub const kMaxNumControls: i32 = 65535i32;
    pub const kMaxNumMaps: i32 = 255i32;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags;
    pub fn get_bindingIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bindingIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_controlIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_controlIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flags",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_hasMultipleConcurrentActuations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasMultipleConcurrentActuations",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_haveMagnitude(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_haveMagnitude",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_inProcessing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_inProcessing",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_interactionIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_interactionIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isButton",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isCanceled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isDisabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isDisabled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isPassThrough(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isPassThrough",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isPerformed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isPerformed",
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
    pub fn get_isStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isStarted",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isWaiting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isWaiting",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_lastCanceledInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lastCanceledInUpdate",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_lastPerformedInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lastPerformedInUpdate",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_magnitude(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_magnitude",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_mapIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mapIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_mayNeedConflictResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mayNeedConflictResolution",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_pressedInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressedInUpdate",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_releasedInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_releasedInUpdate",
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
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_time",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_bindingIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bindingIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_controlIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_controlIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_flags(
        &mut self,
        value: crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flags",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_hasMultipleConcurrentActuations(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_hasMultipleConcurrentActuations",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_inProcessing(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_inProcessing",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_interactionIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_interactionIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isButton",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isPassThrough(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isPassThrough",
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
    pub fn set_lastCanceledInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_lastCanceledInUpdate",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_lastPerformedInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_lastPerformedInUpdate",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_magnitude(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_magnitude",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_mapIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_mapIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_mayNeedConflictResolution(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_mayNeedConflictResolution",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_phase(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_phase",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_pressedInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pressedInUpdate",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_releasedInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_releasedInUpdate",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_startTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_time(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_time",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionState_UnmanagedMemory {
    pub basePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    pub mapCount: i32,
    pub actionCount: i32,
    pub interactionCount: i32,
    pub bindingCount: i32,
    pub controlCount: i32,
    pub compositeCount: i32,
    pub actionStates: *mut quest_hook::libil2cpp::Il2CppObject,
    pub bindingStates: *mut quest_hook::libil2cpp::Il2CppObject,
    pub interactionStates: *mut quest_hook::libil2cpp::Il2CppObject,
    pub controlMagnitudes: *mut quest_hook::libil2cpp::Il2CppObject,
    pub compositeMagnitudes: *mut quest_hook::libil2cpp::Il2CppObject,
    pub enabledControls: *mut quest_hook::libil2cpp::Il2CppObject,
    pub actionBindingIndicesAndCounts: *mut quest_hook::libil2cpp::Il2CppObject,
    pub actionBindingIndices: *mut quest_hook::libil2cpp::Il2CppObject,
    pub controlIndexToBindingIndex: *mut quest_hook::libil2cpp::Il2CppObject,
    pub controlGroupingAndComplexity: *mut quest_hook::libil2cpp::Il2CppObject,
    pub controlGroupingInitialized: bool,
    pub mapIndices: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionState_UnmanagedMemory =>
    "UnityEngine.InputSystem"."InputActionState/UnmanagedMemory"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
impl crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    pub fn Allocate(
        &mut self,
        mapCount: i32,
        actionCount: i32,
        bindingCount: i32,
        controlCount: i32,
        interactionCount: i32,
        compositeCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Allocate",
            (
                mapCount,
                actionCount,
                bindingCount,
                controlCount,
                interactionCount,
                compositeCount,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clone",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CopyDataFrom(
        &mut self,
        memory: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyDataFrom",
            (memory),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isAllocated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isAllocated",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_sizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sizeInBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionState_InputActionState_Flags {
    TimerRunning = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InteractionState_InputActionState_Flags =>
    "UnityEngine.InputSystem"."InputActionState/InteractionState/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerState_InputActionState_Flags {
    Button = 32i32,
    HasMultipleConcurrentActuations = 8i32,
    HaveMagnitude = 1i32,
    InProcessing = 16i32,
    MayNeedConflictResolution = 4i32,
    PassThrough = 2i32,
    Pressed = 64i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::TriggerState_InputActionState_Flags =>
    "UnityEngine.InputSystem"."InputActionState/TriggerState/Flags"
);
