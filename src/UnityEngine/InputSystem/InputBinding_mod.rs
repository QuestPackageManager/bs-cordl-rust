#[cfg(feature = "UnityEngine+InputSystem+InputBinding")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputBinding {
    pub m_Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Id: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Path: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Interactions: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Processors: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Groups: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Action: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Flags: crate::UnityEngine::InputSystem::InputBinding_Flags,
    pub m_OverridePath: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_OverrideInteractions: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_OverrideProcessors: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputBinding =>
    "UnityEngine.InputSystem"."InputBinding"
);
#[cfg(feature = "UnityEngine+InputSystem+InputBinding")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputBinding {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding")]
impl crate::UnityEngine::InputSystem::InputBinding {
    pub const Separator: char = ';';
    pub const kSeparatorString: &'static str = ";";
    #[cfg(feature = "UnityEngine+InputSystem+InputBinding+DisplayStringOptions")]
    pub type DisplayStringOptions = crate::UnityEngine::InputSystem::InputBinding_DisplayStringOptions;
    #[cfg(feature = "UnityEngine+InputSystem+InputBinding+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::InputBinding_Flags;
    #[cfg(feature = "UnityEngine+InputSystem+InputBinding+MatchOptions")]
    pub type MatchOptions = crate::UnityEngine::InputSystem::InputBinding_MatchOptions;
    #[cfg(feature = "UnityEngine+InputSystem+InputBinding+__c")]
    pub type __c = crate::UnityEngine::InputSystem::InputBinding___c;
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
    pub fn Equals_InputBinding0(
        &mut self,
        other: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GenerateId",
            (),
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
    pub fn GetNameOfComposite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetNameOfComposite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Matches_ByRefMut_InputBinding_MatchOptions1(
        &mut self,
        binding: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
        options: crate::UnityEngine::InputSystem::InputBinding_MatchOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Matches",
            (binding, options),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Matches_InputBinding0(
        &mut self,
        binding: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Matches",
            (binding),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveOverrides",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDisplayString_ByRefMut_ByRefMut_InputBinding_DisplayStringOptions_InputControl1(
        &mut self,
        deviceLayoutName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        controlPath: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        options: crate::UnityEngine::InputSystem::InputBinding_DisplayStringOptions,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDisplayString",
            (deviceLayoutName, controlPath, options, control),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDisplayString_InputBinding_DisplayStringOptions_InputControl0(
        &mut self,
        options: crate::UnityEngine::InputSystem::InputBinding_DisplayStringOptions,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDisplayString",
            (options, control),
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
    pub fn TriggersAction(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TriggersAction",
            (action),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (path, action, groups, processors, interactions, name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_action", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectiveInteractions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_effectiveInteractions",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectivePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_effectivePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectiveProcessors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_effectiveProcessors",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_groups", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasOverrides(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasOverrides",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_id",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interactions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_interactions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isComposite",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isEmpty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPartOfComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isPartOfComposite",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideInteractions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_overrideInteractions",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overridePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_overridePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideProcessors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_overrideProcessors",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_path", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_processors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_processors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_action(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_action",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_groups(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_groups",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_id(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_id",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_interactions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_interactions",
            (value),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_name",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideInteractions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_overrideInteractions",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overridePath(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_overridePath",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideProcessors(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_overrideProcessors",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_path(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_path",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_processors(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_processors",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::InputSystem::InputBinding>>
for crate::UnityEngine::InputSystem::InputBinding {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::InputSystem::InputBinding> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::InputSystem::InputBinding>>
for crate::UnityEngine::InputSystem::InputBinding {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::InputBinding,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding+DisplayStringOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputBinding_DisplayStringOptions {
    DontIncludeInteractions = 4i32,
    DontOmitDevice = 2i32,
    DontUseShortDisplayNames = 1i32,
    IgnoreBindingOverrides = 8i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding+DisplayStringOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputBinding_DisplayStringOptions =>
    "UnityEngine.InputSystem"."InputBinding/DisplayStringOptions"
);
#[cfg(feature = "UnityEngine+InputSystem+InputBinding+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputBinding_Flags {
    Composite = 4i32,
    None = 0i32,
    PartOfComposite = 8i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputBinding_Flags =>
    "UnityEngine.InputSystem"."InputBinding/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputBinding+MatchOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputBinding_MatchOptions {
    EmptyGroupMatchesAny = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputBinding+MatchOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputBinding_MatchOptions => "UnityEngine.InputSystem"
    ."InputBinding/MatchOptions"
);
