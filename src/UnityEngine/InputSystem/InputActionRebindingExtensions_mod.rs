#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionRebindingExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
    )]
    pub type DeferBindingResolutionWrapper = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
    pub type Parameter = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
    )]
    pub type ParameterEnumerable = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
    )]
    pub type ParameterEnumerator = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
    )]
    pub type ParameterOverride = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
    )]
    pub type RebindingOperation = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation;
    pub fn AddBindingOverrideJsonTo(
        actions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection2,
        >,
        binding: crate::UnityEngine::InputSystem::InputBinding,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::InputActionMap_BindingOverrideJson,
            >,
        >,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddBindingOverrideJsonTo", (actions, binding, list, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverride_InputActionMap_InputBinding4(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        bindingOverride: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverride", (actionMap, bindingOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverride_InputActionMap_i32_InputBinding5(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        bindingIndex: i32,
        bindingOverride: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverride", (actionMap, bindingIndex, bindingOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverride_InputAction_Il2CppString_Il2CppString_Il2CppString0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        newPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverride", (action, newPath, group, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverride_InputAction_InputBinding1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingOverride: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverride", (action, bindingOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverride_InputAction_i32_Il2CppString3(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingIndex: i32,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverride", (action, bindingIndex, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverride_InputAction_i32_InputBinding2(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingIndex: i32,
        bindingOverride: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverride", (action, bindingIndex, bindingOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverrides(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        overrides: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputBinding,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverrides", (actionMap, overrides))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverridesOnMatchingControls_InputAction0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverridesOnMatchingControls", (action, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBindingOverridesOnMatchingControls_InputActionMap1(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBindingOverridesOnMatchingControls", (actionMap, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputActionAsset_Expression_1_TValue_InputBinding2<
        TObject,
        TValue,
    >(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<
                *mut crate::System::Func_2<TObject, TValue>,
            >,
        >,
        value: TValue,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyParameterOverride", (asset, expr, value, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputActionAsset_Il2CppString_PrimitiveValue_InputBinding4(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyParameterOverride", (asset, name, value, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputActionMap_Expression_1_TValue_InputBinding1<
        TObject,
        TValue,
    >(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<
                *mut crate::System::Func_2<TObject, TValue>,
            >,
        >,
        value: TValue,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyParameterOverride", (actionMap, expr, value, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputActionMap_Il2CppString_PrimitiveValue_InputBinding3(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyParameterOverride", (actionMap, name, value, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputActionState_i32_ByRefMut_ByRefMut_InputActionRebindingExtensions_ParameterOverride7(
        state: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionState,
        >,
        mapIndex: i32,
        parameterOverrides: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
            >,
        >,
        parameterOverridesCount: quest_hook::libil2cpp::ByRefMut<i32>,
        parameterOverride: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ApplyParameterOverride",
                (
                    state,
                    mapIndex,
                    parameterOverrides,
                    parameterOverridesCount,
                    parameterOverride,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputAction_Expression_1_TValue_InputBinding0<
        TObject,
        TValue,
    >(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<
                *mut crate::System::Func_2<TObject, TValue>,
            >,
        >,
        value: TValue,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyParameterOverride", (action, expr, value, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputAction_Il2CppString_PrimitiveValue_InputBinding5(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyParameterOverride", (action, name, value, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameterOverride_InputAction_Il2CppString_PrimitiveValue_i32_6(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyParameterOverride", (action, name, value, bindingIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeferBindingResolution() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeferBindingResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractParameterOverride<TObject, TValue>(
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<
                *mut crate::System::Func_2<TObject, TValue>,
            >,
        >,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractParameterOverride", (expr, bindingMask, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingDisplayString_InputBinding_DisplayStringOptions_Il2CppString0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        options: crate::UnityEngine::InputSystem::InputBinding_DisplayStringOptions,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingDisplayString", (action, options, group))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingDisplayString_InputBinding_InputBinding_DisplayStringOptions1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
        options: crate::UnityEngine::InputSystem::InputBinding_DisplayStringOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingDisplayString", (action, bindingMask, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingDisplayString_i32_ByRefMut_ByRefMut_InputBinding_DisplayStringOptions3(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingIndex: i32,
        deviceLayoutName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        controlPath: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        options: crate::UnityEngine::InputSystem::InputBinding_DisplayStringOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBindingDisplayString",
                (action, bindingIndex, deviceLayoutName, controlPath, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingDisplayString_i32_InputBinding_DisplayStringOptions2(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingIndex: i32,
        options: crate::UnityEngine::InputSystem::InputBinding_DisplayStringOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingDisplayString", (action, bindingIndex, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingForControl(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputBinding>,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingForControl", (action, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingIndexForControl(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingIndexForControl", (action, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingIndex_InputActionMap_InputBinding1(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingIndex", (actionMap, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingIndex_InputAction_Il2CppString_Il2CppString2(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingIndex", (action, group, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingIndex_InputAction_InputBinding0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBindingIndex", (action, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterValue_Expression_1_InputBinding3<TObject, TValue>(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        expr: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<
                *mut crate::System::Func_2<TObject, TValue>,
            >,
        >,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<TValue>>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Nullable_1<TValue> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParameterValue", (action, expr, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterValue_Il2CppString_InputBinding0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParameterValue", (action, name, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterValue_Il2CppString_i32_2(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParameterValue", (action, name, bindingIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterValue_InputActionRebindingExtensions_ParameterOverride1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        parameterOverride: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParameterValue", (action, parameterOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBindingOverridesFromJsonInternal(
        actions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection2,
        >,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBindingOverridesFromJsonInternal", (actions, json))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBindingOverridesFromJson_IInputActionCollection2_0(
        actions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection2,
        >,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        removeExisting: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBindingOverridesFromJson", (actions, json, removeExisting))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBindingOverridesFromJson_InputAction1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        removeExisting: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBindingOverridesFromJson", (action, json, removeExisting))?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformInteractiveRebinding(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PerformInteractiveRebinding", (action, bindingIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAllBindingOverrides_IInputActionCollection2_0(
        actions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveAllBindingOverrides", (actions))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAllBindingOverrides_InputAction1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveAllBindingOverrides", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveBindingOverride_InputActionMap_InputBinding2(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveBindingOverride", (actionMap, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveBindingOverride_InputAction_InputBinding1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveBindingOverride", (action, bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveBindingOverride_InputAction_i32_0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveBindingOverride", (action, bindingIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveBindingOverrides(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        overrides: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputBinding,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveBindingOverrides", (actionMap, overrides))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveBindingOverridesAsJson_IInputActionCollection2_0(
        actions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveBindingOverridesAsJson", (actions))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveBindingOverridesAsJson_InputAction1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveBindingOverridesAsJson", (action))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper
    => "UnityEngine.InputSystem"
    ."InputActionRebindingExtensions/DeferBindingResolutionWrapper"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    pub fn Acquire(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Acquire", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputActionRebindingExtensions_Parameter {
    pub instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    pub bindingIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/Parameter"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter {}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputActionRebindingExtensions_ParameterEnumerable {
    pub m_State: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionState,
    >,
    pub m_Parameter: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
    pub m_MapIndex: i32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/ParameterEnumerable"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_UnityEngine_InputSystem_InputActionRebindingExtensions_Parameter__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<UnityEngine.InputSystem.InputActionRebindingExtensions.Parameter>.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionState,
        >,
        parameter: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        mapIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state, parameter, mapIndex),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    >,
>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    >,
>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputActionRebindingExtensions_ParameterEnumerator {
    pub m_State: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionState,
    >,
    pub m_MapIndex: i32,
    pub m_BindingCurrentIndex: i32,
    pub m_BindingEndIndex: i32,
    pub m_InteractionCurrentIndex: i32,
    pub m_InteractionEndIndex: i32,
    pub m_ProcessorCurrentIndex: i32,
    pub m_ProcessorEndIndex: i32,
    pub m_BindingMask: crate::UnityEngine::InputSystem::InputBinding,
    pub m_ObjectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_ParameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_MayBeInteraction: bool,
    pub m_MayBeProcessor: bool,
    pub m_MayBeComposite: bool,
    pub m_CurrentBindingIsComposite: bool,
    pub m_CurrentObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_CurrentParameter: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::FieldInfo,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/ParameterEnumerator"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindParameter(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FindParameter",
            (instance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToNextBinding(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveToNextBinding",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToNextInteraction(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveToNextInteraction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToNextProcessor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveToNextProcessor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionState,
        >,
        parameter: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        mapIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state, parameter, mapIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    >,
>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    >,
>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputActionRebindingExtensions_ParameterOverride {
    pub objectRegistrationName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub parameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    pub value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/ParameterOverride"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride {
    pub fn Find_Il2CppArray_i32_ByRefMut_Il2CppString1(
        overrides: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
            >,
        >,
        overrideCount: i32,
        binding: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectRegistrationName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Find",
                (
                    overrides,
                    overrideCount,
                    binding,
                    parameterName,
                    objectRegistrationName,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Find_InputActionMap_ByRefMut_Il2CppString0(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        binding: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectRegistrationName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Find",
                (actionMap, binding, parameterName, objectRegistrationName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PickMoreSpecificOne(
        first: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        >,
        second: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PickMoreSpecificOne", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_InputBinding_PrimitiveValue1(
        &mut self,
        objectRegistrationName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (objectRegistrationName, parameterName, bindingMask, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputBinding_PrimitiveValue0(
        &mut self,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (parameterName, bindingMask, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_objectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_objectType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionRebindingExtensions_RebindingOperation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ActionToRebind: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_BindingMask: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_ControlType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_ExpectedLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub m_IncludePathCount: i32,
    pub m_IncludePaths: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_ExcludePathCount: i32,
    pub m_ExcludePaths: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_TargetBindingIndex: i32,
    pub m_BindingGroupForNewBinding: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_CancelBinding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_MagnitudeThreshold: f32,
    pub m_Scores: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub m_Magnitudes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub m_LastMatchTime: f64,
    pub m_StartTime: f64,
    pub m_Timeout: f32,
    pub m_WaitSecondsAfterMatch: f32,
    pub m_Candidates: crate::UnityEngine::InputSystem::InputControlList_1<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_OnComplete: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    >,
    pub m_OnCancel: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    >,
    pub m_OnPotentialMatch: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    >,
    pub m_OnGeneratePath: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            *mut crate::UnityEngine::InputSystem::InputControl,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub m_OnComputeScore: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            *mut crate::UnityEngine::InputSystem::InputControl,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            f32,
        >,
    >,
    pub m_OnApplyBinding: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub m_OnEventDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    >,
    pub m_OnAfterUpdateDelegate: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_LayoutCache: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
    pub m_PathBuilder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub m_Flags: crate::UnityEngine::InputSystem::RebindingOperation_InputActionRebindingExtensions_Flags,
    pub m_StartingActuations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::InputSystem::InputControl,
            f32,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/RebindingOperation"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    pub const kDefaultMagnitudeThreshold: f32 = 0.2f32;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation+Flags"
    )]
    pub type Flags = crate::UnityEngine::InputSystem::RebindingOperation_InputActionRebindingExtensions_Flags;
    pub fn AddCandidate(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        score: f32,
        magnitude: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCandidate", (control, score, magnitude))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cancel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cancel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", ())?;
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
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePathForControl(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GeneratePathForControl", (control))?;
        Ok(__cordl_ret.into())
    }
    pub fn HavePathMatch(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        paths: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        pathCount: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HavePathMatch", (control, paths, pathCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn HookOnAfterUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HookOnAfterUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HookOnEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HookOnEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnAfterUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnApplyBinding(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("OnApplyBinding", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCancel_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCancel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCancel_Action_1_0(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("OnCancel", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnComplete_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnComplete_Action_1_0(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("OnComplete", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnComputeScore(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut crate::UnityEngine::InputSystem::InputControl,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("OnComputeScore", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEvent", (eventPtr, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnGeneratePath(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::UnityEngine::InputSystem::InputControl,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("OnGeneratePath", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnMatchWaitForAnother(
        &mut self,
        seconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("OnMatchWaitForAnother", (seconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPotentialMatch(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("OnPotentialMatch", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCandidate(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCandidate", (control))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetAfterMatchCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAfterMatchCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SortCandidatesByScore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortCandidatesByScore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfRebindInProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfRebindInProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnhookOnAfterUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnhookOnAfterUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnhookOnEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnhookOnEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithAction(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithAction", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithBindingGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithBindingGroup", (group))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithBindingMask(
        &mut self,
        bindingMask: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithBindingMask", (bindingMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithCancelingThrough_Il2CppString0(
        &mut self,
        binding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithCancelingThrough", (binding))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithCancelingThrough_InputControl1(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithCancelingThrough", (control))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithControlsExcluding(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithControlsExcluding", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithControlsHavingToMatchPath(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithControlsHavingToMatchPath", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithExpectedControlType_2<TControl>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithExpectedControlType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithExpectedControlType_Il2CppString0(
        &mut self,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithExpectedControlType", (layoutName))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithExpectedControlType_Type1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithExpectedControlType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithMagnitudeHavingToBeGreaterThan(
        &mut self,
        magnitude: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithMagnitudeHavingToBeGreaterThan", (magnitude))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithMatchingEventsBeingSuppressed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithMatchingEventsBeingSuppressed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithRebindAddingNewBinding(
        &mut self,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithRebindAddingNewBinding", (group))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithTargetBinding(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithTargetBinding", (bindingIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithTimeout(
        &mut self,
        timeInSeconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithTimeout", (timeInSeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithoutGeneralizingPathOfSelectedControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithoutGeneralizingPathOfSelectedControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithoutIgnoringNoisyControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        > = __cordl_object.invoke("WithoutIgnoringNoisyControls", ())?;
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
    pub fn get_action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = __cordl_object.invoke("get_action", ())?;
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
    pub fn get_canceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canceled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_candidates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        > = __cordl_object.invoke("get_candidates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_completed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_completed", ())?;
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
    pub fn get_magnitudes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<f32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            f32,
        > = __cordl_object.invoke("get_magnitudes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scores(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<f32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            f32,
        > = __cordl_object.invoke("get_scores", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = __cordl_object.invoke("get_selectedControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_startTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_started(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_started", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeout(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeout", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation+Flags"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RebindingOperation_InputActionRebindingExtensions_Flags {
    #[default]
    AddNewBinding = 256i32,
    Canceled = 4i32,
    Completed = 2i32,
    DontGeneralizePathOfSelectedControl = 128i32,
    DontIgnoreNoisyControls = 64i32,
    OnAfterUpdateHooked = 16i32,
    OnEventHooked = 8i32,
    Started = 1i32,
    SuppressMatchingEvents = 512i32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation+Flags"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::RebindingOperation_InputActionRebindingExtensions_Flags
    => "UnityEngine.InputSystem"
    ."InputActionRebindingExtensions/RebindingOperation/Flags"
);
