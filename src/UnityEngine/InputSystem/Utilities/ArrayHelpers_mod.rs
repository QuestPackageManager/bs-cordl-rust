#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::ArrayHelpers =>
    "UnityEngine.InputSystem.Utilities"."ArrayHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    #[cfg(
        feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers+__c__DisplayClass33_0_1"
    )]
    pub type __c__DisplayClass33_0_1<TValue: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputSystem::Utilities::ArrayHelpers___c__DisplayClass33_0_1<
        TValue,
    >;
    #[cfg(
        feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers+__c__DisplayClass34_0_1"
    )]
    pub type __c__DisplayClass34_0_1<TValue: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputSystem::Utilities::ArrayHelpers___c__DisplayClass34_0_1<
        TValue,
    >;
    #[cfg(
        feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers+__c__DisplayClass34_1_1"
    )]
    pub type __c__DisplayClass34_1_1<TValue: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputSystem::Utilities::ArrayHelpers___c__DisplayClass34_1_1<
        TValue,
    >;
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
