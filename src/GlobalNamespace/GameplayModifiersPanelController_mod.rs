#[cfg(feature = "GameplayModifiersPanelController")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersPanelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameplayModifiersModel: *mut GameplayModifiersModelSO,
    pub _totalMultiplierValueText: *mut crate::TMPro::TextMeshProUGUI,
    pub _maxRankValueText: *mut crate::TMPro::TextMeshProUGUI,
    pub _positiveColor: crate::UnityEngine::Color,
    pub _negativeColor: crate::UnityEngine::Color,
    pub didChangeGameplayModifiersEvent: *mut crate::System::Action,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub _gameplayModifierToggles: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut GameplayModifierToggle,
    >,
    pub _changingGameplayModifierToggles: bool,
    pub _toggleForGameplayModifierParam: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut GameplayModifierParamsSO,
        *mut crate::UnityEngine::UI::Toggle,
    >,
}
#[cfg(feature = "GameplayModifiersPanelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameplayModifiersPanelController => ""
    ."GameplayModifiersPanelController"
);
#[cfg(feature = "GameplayModifiersPanelController")]
impl std::ops::Deref for GameplayModifiersPanelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersPanelController")]
impl std::ops::DerefMut for GameplayModifiersPanelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersPanelController")]
impl GameplayModifiersPanelController {
    #[cfg(feature = "GameplayModifiersPanelController+__c__DisplayClass16_0")]
    pub type __c__DisplayClass16_0 = crate::GlobalNamespace::GameplayModifiersPanelController___c__DisplayClass16_0;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetToggleValueWithGameplayModifierParams(
        &mut self,
        gameplayModifierParams: *mut GameplayModifierParamsSO,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetToggleValueWithGameplayModifierParams",
                (gameplayModifierParams),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IRefreshable_Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IRefreshable.Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshTotalMultiplierAndRankUI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshTotalMultiplierAndRankUI", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        newGameplayModifiers: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (newGameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn SetToggleValueWithGameplayModifierParams(
        &mut self,
        gameplayModifierParams: *mut GameplayModifierParamsSO,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetToggleValueWithGameplayModifierParams",
                (gameplayModifierParams, value),
            )?;
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
    pub fn add_didChangeGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameplayModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameplayModifiers = __cordl_object
            .invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangeGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameplayModifiersPanelController")]
impl quest_hook::libil2cpp::ObjectType for GameplayModifiersPanelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
