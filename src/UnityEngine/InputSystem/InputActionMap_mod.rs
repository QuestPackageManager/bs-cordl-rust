#[cfg(feature = "UnityEngine+InputSystem+InputActionMap")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Id: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Asset: *mut crate::UnityEngine::InputSystem::InputActionAsset,
    pub m_Actions: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_Bindings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_BindingsForEachAction: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_ControlsForEachAction: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_EnabledActionsCount: i32,
    pub m_SingletonAction: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_MapIndexInState: i32,
    pub m_State: *mut crate::UnityEngine::InputSystem::InputActionState,
    pub m_BindingMask: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_Flags: crate::UnityEngine::InputSystem::InputActionMap_Flags,
    pub m_ParameterOverridesCount: i32,
    pub m_ParameterOverrides: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
    >,
    pub m_Devices: crate::UnityEngine::InputSystem::InputActionMap_DeviceArray,
    pub m_ActionCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    >,
    pub m_ActionIndexByNameOrId: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        i32,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputActionMap =>
    "UnityEngine.InputSystem"."InputActionMap"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputActionMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputActionMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap")]
impl crate::UnityEngine::InputSystem::InputActionMap {
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingJson")]
    pub type BindingJson = crate::UnityEngine::InputSystem::InputActionMap_BindingJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideJson")]
    pub type BindingOverrideJson = crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideListJson")]
    pub type BindingOverrideListJson = crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideListJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+DeviceArray")]
    pub type DeviceArray = crate::UnityEngine::InputSystem::InputActionMap_DeviceArray;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::InputActionMap_Flags;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadActionJson")]
    pub type ReadActionJson = crate::UnityEngine::InputSystem::InputActionMap_ReadActionJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadFileJson")]
    pub type ReadFileJson = crate::UnityEngine::InputSystem::InputActionMap_ReadFileJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadMapJson")]
    pub type ReadMapJson = crate::UnityEngine::InputSystem::InputActionMap_ReadMapJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteActionJson")]
    pub type WriteActionJson = crate::UnityEngine::InputSystem::InputActionMap_WriteActionJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteFileJson")]
    pub type WriteFileJson = crate::UnityEngine::InputSystem::InputActionMap_WriteFileJson;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteMapJson")]
    pub type WriteMapJson = crate::UnityEngine::InputSystem::InputActionMap_WriteMapJson;
    pub fn ClearActionLookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearActionLookupTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearCachedActionData(
        &mut self,
        onlyControls: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCachedActionData", (onlyControls))?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionMap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionMap = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (action))?;
        Ok(__cordl_ret)
    }
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disable", ())?;
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
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enable", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindActionIndex_Guid1(
        &mut self,
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindActionIndex", (id))?;
        Ok(__cordl_ret)
    }
    pub fn FindActionIndex_Il2CppString0(
        &mut self,
        nameOrId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindActionIndex", (nameOrId))?;
        Ok(__cordl_ret)
    }
    pub fn FindAction_Guid1(
        &mut self,
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("FindAction", (id))?;
        Ok(__cordl_ret)
    }
    pub fn FindAction_Il2CppString__cordl_bool0(
        &mut self,
        actionNameOrId: *mut quest_hook::libil2cpp::Il2CppString,
        throwIfNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("FindAction", (actionNameOrId, throwIfNotFound))?;
        Ok(__cordl_ret)
    }
    pub fn FindBinding(
        &mut self,
        mask: crate::UnityEngine::InputSystem::InputBinding,
        action: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::InputSystem::InputAction,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindBinding", (mask, action))?;
        Ok(__cordl_ret)
    }
    pub fn FindBindingRelativeToMap(
        &mut self,
        mask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindBindingRelativeToMap", (mask))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateId", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBindingsForSingleAction(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
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
        > = __cordl_object.invoke("GetBindingsForSingleAction", (action))?;
        Ok(__cordl_ret)
    }
    pub fn GetControlsForSingleAction(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
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
        > = __cordl_object.invoke("GetControlsForSingleAction", (action))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::InputSystem::InputAction,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::InputSystem::InputAction,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsUsableWithDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsUsableWithDevice", (device))?;
        Ok(__cordl_ret)
    }
    pub fn LazyResolveBindings(
        &mut self,
        fullResolve: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LazyResolveBindings", (fullResolve))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString1(
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterDeserialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnBindingModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBindingModified", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnSetupChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSetupChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnWantToChangeSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnWantToChangeSetup", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveBindingsIfNecessary(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ResolveBindingsIfNecessary", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetUpActionLookupTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUpActionLookupTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetUpPerActionControlAndBindingArrays(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUpPerActionControlAndBindingArrays", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
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
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToJson", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_InputSystem_IInputActionCollection2_get_bindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object
            .invoke("UnityEngine.InputSystem.IInputActionCollection2.get_bindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn add_actionTriggered(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_actionTriggered", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        actionNameOrId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("get_Item", (actionNameOrId))?;
        Ok(__cordl_ret)
    }
    pub fn get_actions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputAction,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputAction,
        > = __cordl_object.invoke("get_actions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_asset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionAsset = __cordl_object
            .invoke("get_asset", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_bindingResolutionNeedsFullReResolve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_bindingResolutionNeedsFullReResolve", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_bindingsForEachActionInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_bindingsForEachActionInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_controlSchemes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputControlScheme,
        > = __cordl_object.invoke("get_controlSchemes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_controlsForEachActionInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_controlsForEachActionInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_devices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        > = __cordl_object.invoke("get_devices", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_idDontGenerate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("get_idDontGenerate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_needToResolveBindings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_needToResolveBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_actionTriggered(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_actionTriggered", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_bindingResolutionNeedsFullReResolve(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bindingResolutionNeedsFullReResolve", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_bindingsForEachActionInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bindingsForEachActionInitialized", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_controlsForEachActionInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controlsForEachActionInitialized", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_devices(
        &mut self,
        value: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_devices", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_needToResolveBindings(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_needToResolveBindings", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_BindingJson {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub id: *mut quest_hook::libil2cpp::Il2CppString,
    pub path: *mut quest_hook::libil2cpp::Il2CppString,
    pub interactions: *mut quest_hook::libil2cpp::Il2CppString,
    pub processors: *mut quest_hook::libil2cpp::Il2CppString,
    pub groups: *mut quest_hook::libil2cpp::Il2CppString,
    pub action: *mut quest_hook::libil2cpp::Il2CppString,
    pub isComposite: bool,
    pub isPartOfComposite: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_BindingJson => "UnityEngine.InputSystem"
    ."InputActionMap/BindingJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_BindingJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_BindingJson {
    pub fn ToBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::InputBinding> {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputBinding = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToBinding",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_BindingOverrideJson {
    pub action: *mut quest_hook::libil2cpp::Il2CppString,
    pub id: *mut quest_hook::libil2cpp::Il2CppString,
    pub path: *mut quest_hook::libil2cpp::Il2CppString,
    pub interactions: *mut quest_hook::libil2cpp::Il2CppString,
    pub processors: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_BindingOverrideJson =>
    "UnityEngine.InputSystem"."InputActionMap/BindingOverrideJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideJson {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideListJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_BindingOverrideListJson {
    pub bindings: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideJson,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideListJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_BindingOverrideListJson =>
    "UnityEngine.InputSystem"."InputActionMap/BindingOverrideListJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideListJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideListJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+BindingOverrideListJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideListJson {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+DeviceArray")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_DeviceArray {
    pub m_HaveValue: bool,
    pub m_DeviceCount: i32,
    pub m_DeviceArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+DeviceArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_DeviceArray => "UnityEngine.InputSystem"
    ."InputActionMap/DeviceArray"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+DeviceArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_DeviceArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+DeviceArray")]
impl crate::UnityEngine::InputSystem::InputActionMap_DeviceArray {
    pub fn Get(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "Get", ())?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IndexOf",
            (device),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (device),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Set(
        &mut self,
        devices: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (devices),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputActionMap_Flags {
    BindingResolutionNeedsFullReResolve = 2i32,
    BindingsForEachActionInitialized = 8i32,
    ControlsForEachActionInitialized = 4i32,
    NeedToResolveBindings = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputActionMap_Flags
    => "UnityEngine.InputSystem"."InputActionMap/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadActionJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_ReadActionJson {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _cordl_type: *mut quest_hook::libil2cpp::Il2CppString,
    pub id: *mut quest_hook::libil2cpp::Il2CppString,
    pub expectedControlType: *mut quest_hook::libil2cpp::Il2CppString,
    pub expectedControlLayout: *mut quest_hook::libil2cpp::Il2CppString,
    pub processors: *mut quest_hook::libil2cpp::Il2CppString,
    pub interactions: *mut quest_hook::libil2cpp::Il2CppString,
    pub passThrough: bool,
    pub initialStateCheck: bool,
    pub bindings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_BindingJson,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadActionJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_ReadActionJson =>
    "UnityEngine.InputSystem"."InputActionMap/ReadActionJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadActionJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_ReadActionJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadActionJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_ReadActionJson {
    pub fn ToAction(
        &mut self,
        actionName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToAction",
            (actionName),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadFileJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_ReadFileJson {
    pub actions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_ReadActionJson,
    >,
    pub maps: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_ReadMapJson,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadFileJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_ReadFileJson => "UnityEngine.InputSystem"
    ."InputActionMap/ReadFileJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadFileJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_ReadFileJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadFileJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_ReadFileJson {
    pub fn ToMaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::InputSystem::InputActionMap,
        >,
    > {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::InputSystem::InputActionMap,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToMaps", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadMapJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_ReadMapJson {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub id: *mut quest_hook::libil2cpp::Il2CppString,
    pub actions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_ReadActionJson,
    >,
    pub bindings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_BindingJson,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadMapJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_ReadMapJson => "UnityEngine.InputSystem"
    ."InputActionMap/ReadMapJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadMapJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_ReadMapJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+ReadMapJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_ReadMapJson {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteActionJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_WriteActionJson {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _cordl_type: *mut quest_hook::libil2cpp::Il2CppString,
    pub id: *mut quest_hook::libil2cpp::Il2CppString,
    pub expectedControlType: *mut quest_hook::libil2cpp::Il2CppString,
    pub processors: *mut quest_hook::libil2cpp::Il2CppString,
    pub interactions: *mut quest_hook::libil2cpp::Il2CppString,
    pub initialStateCheck: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteActionJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_WriteActionJson =>
    "UnityEngine.InputSystem"."InputActionMap/WriteActionJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteActionJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_WriteActionJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteActionJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_WriteActionJson {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteFileJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_WriteFileJson {
    pub maps: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_WriteMapJson,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteFileJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_WriteFileJson => "UnityEngine.InputSystem"
    ."InputActionMap/WriteFileJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteFileJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_WriteFileJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteFileJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_WriteFileJson {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteMapJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionMap_WriteMapJson {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub id: *mut quest_hook::libil2cpp::Il2CppString,
    pub actions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_WriteActionJson,
    >,
    pub bindings: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputActionMap_BindingJson,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteMapJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionMap_WriteMapJson => "UnityEngine.InputSystem"
    ."InputActionMap/WriteMapJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteMapJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionMap_WriteMapJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionMap+WriteMapJson")]
impl crate::UnityEngine::InputSystem::InputActionMap_WriteMapJson {}
