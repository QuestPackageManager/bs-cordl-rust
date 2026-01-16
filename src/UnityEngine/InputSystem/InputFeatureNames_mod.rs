#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputFeatureNames")]
#[repr(C)]
#[derive(Debug)]
pub struct InputFeatureNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputFeatureNames")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InputSystem::InputFeatureNames {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputFeatureNames";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputFeatureNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputFeatureNames {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputFeatureNames")]
impl crate::UnityEngine::InputSystem::InputFeatureNames {
    pub const kDisableUnityRemoteSupport: &'static str = "DISABLE_UNITY_REMOTE_SUPPORT";
    pub const kParanoidReadValueCachingChecks: &'static str = "PARANOID_READ_VALUE_CACHING_CHECKS";
    pub const kRunPlayerUpdatesInEditMode: &'static str = "RUN_PLAYER_UPDATES_IN_EDIT_MODE";
    pub const kUseIMGUIEditorForAssets: &'static str = "USE_IMGUI_EDITOR_FOR_ASSETS";
    pub const kUseOptimizedControls: &'static str = "USE_OPTIMIZED_CONTROLS";
    pub const kUseReadValueCaching: &'static str = "USE_READ_VALUE_CACHING";
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputFeatureNames")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::InputFeatureNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
