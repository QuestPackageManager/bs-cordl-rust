#[cfg(feature = "TutorialSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct TutorialSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub colorScheme: *mut crate::GlobalNamespace::ColorScheme,
    pub playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
}
#[cfg(feature = "TutorialSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TutorialSceneSetupData => ""
    ."TutorialSceneSetupData"
);
#[cfg(feature = "TutorialSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::TutorialSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::TutorialSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TutorialSceneSetupData")]
impl crate::GlobalNamespace::TutorialSceneSetupData {
    pub fn New(
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorScheme, playerSpecificSettings))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorScheme, playerSpecificSettings))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TutorialSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TutorialSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
