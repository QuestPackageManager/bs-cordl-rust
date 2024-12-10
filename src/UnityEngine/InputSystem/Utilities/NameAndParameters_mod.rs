#[cfg(feature = "UnityEngine+InputSystem+Utilities+NameAndParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NameAndParameters {
    pub _name_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _parameters_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NameAndParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::NameAndParameters =>
    "UnityEngine.InputSystem.Utilities"."NameAndParameters"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NameAndParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::NameAndParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NameAndParameters")]
impl crate::UnityEngine::InputSystem::Utilities::NameAndParameters {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+NameAndParameters+__c")]
    pub type __c = crate::UnityEngine::InputSystem::Utilities::NameAndParameters___c;
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
    pub fn get_parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_parameters", ())?;
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
    pub fn set_parameters(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_parameters",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
