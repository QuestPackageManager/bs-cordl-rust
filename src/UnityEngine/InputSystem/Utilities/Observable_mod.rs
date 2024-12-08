#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
#[repr(C)]
#[derive(Debug)]
pub struct Observable {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::Observable
    => "UnityEngine.InputSystem.Utilities"."Observable"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::Observable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::Observable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl crate::UnityEngine::InputSystem::Utilities::Observable {
    #[cfg(
        feature = "UnityEngine+InputSystem+Utilities+Observable+__c__DisplayClass6_0_1"
    )]
    pub type __c__DisplayClass6_0_1<TValue: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputSystem::Utilities::Observable___c__DisplayClass6_0_1<
        TValue,
    >;
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::Observable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
