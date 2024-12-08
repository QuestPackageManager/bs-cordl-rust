#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct MiscHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::MiscHelpers
    => "UnityEngine.InputSystem.Utilities"."MiscHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers+_EveryNth_d__1_1")]
    pub type _EveryNth_d__1_1<TValue: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputSystem::Utilities::MiscHelpers__EveryNth_d__1_1<
        TValue,
    >;
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
