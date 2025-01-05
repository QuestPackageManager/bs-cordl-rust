#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
#[repr(C)]
#[derive(Debug)]
pub struct InputFeatureNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputFeatureNames =>
    "UnityEngine.InputSystem"."InputFeatureNames"
);
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputFeatureNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputFeatureNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
impl crate::UnityEngine::InputSystem::InputFeatureNames {
    pub const kDisableUnityRemoteSupport: &'static str = "DISABLE_UNITY_REMOTE_SUPPORT";
    pub const kParanoidReadValueCachingChecks: &'static str = "PARANOID_READ_VALUE_CACHING_CHECKS";
    pub const kRunPlayerUpdatesInEditMode: &'static str = "RUN_PLAYER_UPDATES_IN_EDIT_MODE";
    pub const kUseOptimizedControls: &'static str = "USE_OPTIMIZED_CONTROLS";
    pub const kUseReadValueCaching: &'static str = "USE_READ_VALUE_CACHING";
    pub const kUseWindowsGamingInputBackend: &'static str = "USE_WINDOWS_GAMING_INPUT_BACKEND";
}
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputFeatureNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
