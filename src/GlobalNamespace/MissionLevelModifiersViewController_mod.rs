#[cfg(feature = "MissionLevelModifiersViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelModifiersViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _gameplayModifiersModel: *mut crate::GlobalNamespace::GameplayModifiersModelSO,
    pub _gameplayModifierInfoListItemsList: *mut crate::GlobalNamespace::GameplayModifierInfoListItemsList,
    pub _modifiersPanel: *mut crate::UnityEngine::GameObject,
    pub _titleText: *mut crate::TMPro::TextMeshProUGUI,
    pub _gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
}
#[cfg(feature = "MissionLevelModifiersViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionLevelModifiersViewController => ""
    ."MissionLevelModifiersViewController"
);
#[cfg(feature = "MissionLevelModifiersViewController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelModifiersViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelModifiersViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionLevelModifiersViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelModifiersViewController")]
impl crate::GlobalNamespace::MissionLevelModifiersViewController {
    #[cfg(feature = "MissionLevelModifiersViewController+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::GlobalNamespace::MissionLevelModifiersViewController___c__DisplayClass7_0;
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (gameplayModifiers))?;
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
#[cfg(feature = "MissionLevelModifiersViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelModifiersViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
