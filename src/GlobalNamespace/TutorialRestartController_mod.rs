#[cfg(feature = "TutorialRestartController")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialRestartController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tutorialSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TutorialScenesTransitionSetupDataSO,
    >,
}
#[cfg(feature = "TutorialRestartController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TutorialRestartController => ""
    ."TutorialRestartController"
);
#[cfg(feature = "TutorialRestartController")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialRestartController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialRestartController")]
impl std::ops::DerefMut for crate::GlobalNamespace::TutorialRestartController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialRestartController")]
impl crate::GlobalNamespace::TutorialRestartController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RestartLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestartLevel", ())?;
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
#[cfg(feature = "TutorialRestartController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialRestartController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TutorialRestartController")]
impl AsRef<crate::GlobalNamespace::ILevelRestartController>
for crate::GlobalNamespace::TutorialRestartController {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILevelRestartController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TutorialRestartController")]
impl AsMut<crate::GlobalNamespace::ILevelRestartController>
for crate::GlobalNamespace::TutorialRestartController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILevelRestartController {
        unsafe { std::mem::transmute(self) }
    }
}
