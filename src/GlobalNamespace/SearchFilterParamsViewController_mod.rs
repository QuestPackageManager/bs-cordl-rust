#[cfg(feature = "SearchFilterParamsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SearchFilterParamsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _filterByOwnedToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _filterByNotOwnedToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _filterByCharacteristicToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _beatmapCharacteristicsDropdown: *mut BeatmapCharacteristicsDropdown,
    pub _filterByDifficultyToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _beatmapDifficultyDropdown: *mut BeatmapDifficultyDropdown,
    pub _filterBySongPacksToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _songPacksDropdown: *mut SongPacksDropdown,
    pub _filterByNotPlayedYetToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _filterByMinBpmToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _minBpmController: *mut FormattedFloatListSettingsController,
    pub _filterByMaxBpmToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _maxBpmController: *mut FormattedFloatListSettingsController,
    pub _filterBySensitivity: *mut PlayerSensitivityDropdown,
    pub _forcedSensitivityOptionNotice: *mut crate::UnityEngine::GameObject,
    pub _okButton: *mut crate::UnityEngine::UI::Button,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _currentSearchFilter: LevelFilter,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut SearchFilterParamsViewController,
        LevelFilter,
    >,
}
#[cfg(feature = "SearchFilterParamsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SearchFilterParamsViewController => ""
    ."SearchFilterParamsViewController"
);
#[cfg(feature = "SearchFilterParamsViewController")]
impl std::ops::Deref for SearchFilterParamsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SearchFilterParamsViewController")]
impl std::ops::DerefMut for SearchFilterParamsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SearchFilterParamsViewController")]
impl SearchFilterParamsViewController {
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut SearchFilterParamsViewController,
            LevelFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut SearchFilterParamsViewController,
            LevelFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OkButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OkButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn Refresh(
        &mut self,
        filter: quest_hook::libil2cpp::ByRefMut<LevelFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", (filter))?;
        Ok(__cordl_ret)
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
    pub fn MinBpmControllerValueDidChange(
        &mut self,
        minBpmController: *mut FormattedFloatListSettingsController,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MinBpmControllerValueDidChange", (minBpmController, value))?;
        Ok(__cordl_ret)
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
    pub fn HandleFilterByNotOwnedValueValueChanged(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFilterByNotOwnedValueValueChanged", (isOn))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        filter: quest_hook::libil2cpp::ByRefMut<LevelFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (filter))?;
        Ok(__cordl_ret)
    }
    pub fn MaxBpmControllerValueDidChange(
        &mut self,
        maxBpmController: *mut FormattedFloatListSettingsController,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MaxBpmControllerValueDidChange", (maxBpmController, value))?;
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
#[cfg(feature = "SearchFilterParamsViewController")]
impl quest_hook::libil2cpp::ObjectType for SearchFilterParamsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
