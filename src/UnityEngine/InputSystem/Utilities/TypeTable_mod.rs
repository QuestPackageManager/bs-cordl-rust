#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeTable")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TypeTable {
    pub table: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
        *mut crate::System::Type,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeTable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::TypeTable
    => "UnityEngine.InputSystem.Utilities"."TypeTable"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeTable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::TypeTable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeTable")]
impl crate::UnityEngine::InputSystem::Utilities::TypeTable {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeTable+__c")]
    pub type __c = crate::UnityEngine::InputSystem::Utilities::TypeTable___c;
    pub fn AddTypeRegistration(
        &mut self,
        name: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddTypeRegistration",
            (name, _cordl_type),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FindNameForType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FindNameForType",
            (_cordl_type),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Initialize",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn LookupTypeRegistration(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_ret: *mut crate::System::Type = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LookupTypeRegistration",
            (name),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_internedNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_internedNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_names(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_names", ())?;
        Ok(__cordl_ret)
    }
}