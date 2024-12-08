#[cfg(feature = "MissionLevelModifiersViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelModifiersViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _gameplayModifiersModel: *mut GameplayModifiersModelSO,
    pub _gameplayModifierInfoListItemsList: *mut GameplayModifierInfoListItemsList,
    pub _modifiersPanel: *mut crate::UnityEngine::GameObject,
    pub _titleText: *mut crate::TMPro::TextMeshProUGUI,
    pub _gameplayModifiers: *mut GameplayModifiers,
}
#[cfg(feature = "MissionLevelModifiersViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionLevelModifiersViewController => ""
    ."MissionLevelModifiersViewController"
);
#[cfg(feature = "MissionLevelModifiersViewController")]
impl std::ops::Deref for MissionLevelModifiersViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelModifiersViewController")]
impl std::ops::DerefMut for MissionLevelModifiersViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelModifiersViewController")]
impl MissionLevelModifiersViewController {
    #[cfg(feature = "MissionLevelModifiersViewController+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::GlobalNamespace::MissionLevelModifiersViewController___c__DisplayClass7_0;
    pub fn RefreshContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        gameplayModifiers: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (gameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MissionLevelModifiersViewController")]
impl quest_hook::libil2cpp::ObjectType for MissionLevelModifiersViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
