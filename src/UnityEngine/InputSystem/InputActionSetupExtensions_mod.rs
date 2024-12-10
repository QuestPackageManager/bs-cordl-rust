#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionSetupExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionSetupExtensions => "UnityEngine.InputSystem"
    ."InputActionSetupExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    #[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
    pub type BindingSyntax = crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax"
    )]
    pub type CompositeSyntax = crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
    )]
    pub type ControlSchemeSyntax = crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionSetupExtensions+__c__DisplayClass5_0"
    )]
    pub type __c__DisplayClass5_0 = crate::UnityEngine::InputSystem::InputActionSetupExtensions___c__DisplayClass5_0;
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionSetupExtensions_BindingSyntax {
    pub m_ActionMap: *mut crate::UnityEngine::InputSystem::InputActionMap,
    pub m_Action: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_BindingIndexInMap: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax =>
    "UnityEngine.InputSystem"."InputActionSetupExtensions/BindingSyntax"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    pub fn Erase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Erase",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertPartBinding(
        &mut self,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertPartBinding",
            (partName, path),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Iterate(
        &mut self,
        next: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Iterate",
            (next),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IterateCompositeBinding(
        &mut self,
        next: bool,
        compositeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IterateCompositeBinding",
            (next, compositeName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IteratePartBinding(
        &mut self,
        next: bool,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IteratePartBinding",
            (next, partName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextBinding",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextCompositeBinding(
        &mut self,
        compositeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextCompositeBinding",
            (compositeName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextPartBinding(
        &mut self,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextPartBinding",
            (partName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PreviousBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PreviousBinding",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PreviousCompositeBinding(
        &mut self,
        compositeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PreviousCompositeBinding",
            (compositeName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PreviousPartBinding(
        &mut self,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PreviousPartBinding",
            (partName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn To(
        &mut self,
        binding: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "To",
            (binding),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Triggering(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Triggering",
            (action),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithGroup",
            (group),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithGroups(
        &mut self,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithGroups",
            (groups),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithInteraction_1<TInteraction>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    >
    where
        TInteraction: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithInteraction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithInteraction_Il2CppString0(
        &mut self,
        interaction: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithInteraction",
            (interaction),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithInteractions(
        &mut self,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithInteractions",
            (interactions),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithName",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithPath(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithPath",
            (path),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessor_1<TProcessor>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    >
    where
        TProcessor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithProcessor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessor_Il2CppString0(
        &mut self,
        processor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithProcessor",
            (processor),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessors(
        &mut self,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithProcessors",
            (processors),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        bindingIndexInMap: i32,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (map, bindingIndexInMap, action),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_binding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::InputBinding> {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputBinding = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_binding",
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
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionSetupExtensions_CompositeSyntax {
    pub m_Action: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_ActionMap: *mut crate::UnityEngine::InputSystem::InputActionMap,
    pub m_BindingIndexInMap: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax =>
    "UnityEngine.InputSystem"."InputActionSetupExtensions/CompositeSyntax"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    pub fn With(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        binding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "With",
            (name, binding, groups, processors),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        compositeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (map, action, compositeIndex),
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
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionSetupExtensions_ControlSchemeSyntax {
    pub m_Asset: *mut crate::UnityEngine::InputSystem::InputActionAsset,
    pub m_ControlSchemeIndex: i32,
    pub m_ControlScheme: crate::UnityEngine::InputSystem::InputControlScheme,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax =>
    "UnityEngine.InputSystem"."InputActionSetupExtensions/ControlSchemeSyntax"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    pub fn AddDeviceEntry(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: crate::UnityEngine::InputSystem::DeviceRequirement_InputControlScheme_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddDeviceEntry",
            (controlPath, flags),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeviceTypeToControlPath<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DeviceTypeToControlPath",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Done(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Done",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OrWithOptionalDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OrWithOptionalDevice",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OrWithOptionalDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OrWithOptionalDevice",
            (controlPath),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OrWithRequiredDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OrWithRequiredDevice",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OrWithRequiredDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OrWithRequiredDevice",
            (controlPath),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithBindingGroup(
        &mut self,
        bindingGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithBindingGroup",
            (bindingGroup),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithOptionalDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithOptionalDevice",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithOptionalDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithOptionalDevice",
            (controlPath),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithRequiredDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithRequiredDevice",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithRequiredDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithRequiredDevice",
            (controlPath),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputActionAsset_i32_0(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (asset, index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputControlScheme1(
        &mut self,
        controlScheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (controlScheme),
        )?;
        Ok(__cordl_ret.into())
    }
}
