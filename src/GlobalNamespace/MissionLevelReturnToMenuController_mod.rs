#[cfg(feature = "MissionLevelReturnToMenuController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelReturnToMenuController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionLevelSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    >,
    pub _prepareLevelCompletionResults: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PrepareLevelCompletionResults,
    >,
    pub _missionObjectiveCheckersManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionObjectiveCheckersManager,
    >,
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionLevelReturnToMenuController => ""
    ."MissionLevelReturnToMenuController"
);
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl crate::GlobalNamespace::MissionLevelReturnToMenuController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReturnToMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnToMenu", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl AsRef<crate::GlobalNamespace::IReturnToMenuController>
for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReturnToMenuController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MissionLevelReturnToMenuController")]
impl AsMut<crate::GlobalNamespace::IReturnToMenuController>
for crate::GlobalNamespace::MissionLevelReturnToMenuController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReturnToMenuController {
        unsafe { std::mem::transmute(self) }
    }
}
