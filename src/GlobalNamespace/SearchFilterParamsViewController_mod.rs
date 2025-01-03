#[cfg(feature = "SearchFilterParamsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SearchFilterParamsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _filterByOwnedToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _filterByNotOwnedToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _filterByCharacteristicToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _beatmapCharacteristicsDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicsDropdown,
    >,
    pub _filterByDifficultyToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _beatmapDifficultyDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDifficultyDropdown,
    >,
    pub _filterBySongPacksToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _songPacksDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPacksDropdown,
    >,
    pub _filterByNotPlayedYetToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _filterByMinBpmToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _minBpmController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _filterByMaxBpmToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _maxBpmController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _filterBySensitivity: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSensitivityDropdown,
    >,
    pub _forcedSensitivityOptionNotice: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _okButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _currentSearchFilter: crate::GlobalNamespace::LevelFilter,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::SearchFilterParamsViewController,
            crate::GlobalNamespace::LevelFilter,
        >,
    >,
}
#[cfg(feature = "SearchFilterParamsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SearchFilterParamsViewController => ""
    ."SearchFilterParamsViewController"
);
#[cfg(feature = "SearchFilterParamsViewController")]
impl std::ops::Deref for crate::GlobalNamespace::SearchFilterParamsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SearchFilterParamsViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SearchFilterParamsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SearchFilterParamsViewController")]
impl crate::GlobalNamespace::SearchFilterParamsViewController {
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
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleFilterByNotOwnedValueValueChanged(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFilterByNotOwnedValueValueChanged", (isOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleFilterByOwnedValueValueChanged(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFilterByOwnedValueValueChanged", (isOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaxBpmControllerValueDidChange(
        &mut self,
        maxBpmController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MaxBpmControllerValueDidChange", (maxBpmController, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn MinBpmControllerValueDidChange(
        &mut self,
        minBpmController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MinBpmControllerValueDidChange", (minBpmController, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OkButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OkButtonPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
        filter: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::LevelFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", (filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        filter: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::LevelFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (filter))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::SearchFilterParamsViewController,
                crate::GlobalNamespace::LevelFilter,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::SearchFilterParamsViewController,
                crate::GlobalNamespace::LevelFilter,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SearchFilterParamsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SearchFilterParamsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
