#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct StringHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::StringHelpers =>
    "UnityEngine.InputSystem.Utilities"."StringHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers+_Split_d__9")]
    pub type _Split_d__9 = crate::UnityEngine::InputSystem::Utilities::StringHelpers__Split_d__9;
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers+_Tokenize_d__8")]
    pub type _Tokenize_d__8 = crate::UnityEngine::InputSystem::Utilities::StringHelpers__Tokenize_d__8;
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
