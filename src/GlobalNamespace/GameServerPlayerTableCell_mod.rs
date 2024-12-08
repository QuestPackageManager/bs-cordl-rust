#[cfg(feature = "GameServerPlayerTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerPlayerTableCell {
    __cordl_parent: TableCellWithSeparator,
    pub _playerNameText: *mut crate::HMUI::CurvedTextMeshPro,
    pub _localPlayerBackgroundImage: *mut crate::UnityEngine::UI::Image,
    pub _suggestedLevelText: *mut crate::HMUI::CurvedTextMeshPro,
    pub _suggestedCharacteristicIcon: *mut crate::HMUI::ImageView,
    pub _suggestedDifficultyText: *mut crate::TMPro::TextMeshProUGUI,
    pub _emptySuggestedLevelText: *mut crate::HMUI::CurvedTextMeshPro,
    pub _suggestedModifiersList: *mut GameplayModifierInfoListItemsList,
    pub _emptySuggestedModifiersText: *mut crate::HMUI::CurvedTextMeshPro,
    pub _mutePlayerButton: *mut crate::UnityEngine::UI::Button,
    pub _kickPlayerButton: *mut crate::UnityEngine::UI::Button,
    pub _useBeatmapButton: *mut crate::UnityEngine::UI::Button,
    pub _useModifiersButton: *mut crate::UnityEngine::UI::Button,
    pub _useBeatmapButtonHoverHint: *mut crate::HMUI::HoverHint,
    pub _muteToggle: *mut crate::HMUI::ButtonSpriteSwapToggle,
    pub _statusImageView: *mut crate::HMUI::ImageView,
    pub _readyIcon: *mut crate::UnityEngine::Sprite,
    pub _spectatingIcon: *mut crate::UnityEngine::Sprite,
    pub _hostIcon: *mut crate::UnityEngine::Sprite,
    pub _gameplayModifiers: *mut GameplayModifiersModelSO,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub kickPlayerEvent: *mut crate::System::Action_1<i32>,
    pub useBeatmapEvent: *mut crate::System::Action_1<i32>,
    pub useModifiersEvent: *mut crate::System::Action_1<i32>,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
    pub _getLevelEntitlementCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
}
#[cfg(feature = "GameServerPlayerTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameServerPlayerTableCell => ""
    ."GameServerPlayerTableCell"
);
#[cfg(feature = "GameServerPlayerTableCell")]
impl std::ops::Deref for GameServerPlayerTableCell {
    type Target = TableCellWithSeparator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl std::ops::DerefMut for GameServerPlayerTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl GameServerPlayerTableCell {
    #[cfg(feature = "GameServerPlayerTableCell+__c__DisplayClass30_0")]
    pub type __c__DisplayClass30_0 = crate::GlobalNamespace::GameServerPlayerTableCell___c__DisplayClass30_0;
    #[cfg(feature = "GameServerPlayerTableCell+_SetBeatmapUseButtonEnabledAsync_d__36")]
    pub type _SetBeatmapUseButtonEnabledAsync_d__36 = crate::GlobalNamespace::GameServerPlayerTableCell__SetBeatmapUseButtonEnabledAsync_d__36;
    pub fn add_useModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_useModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_kickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_kickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_useBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_useBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_useBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_useBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
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
    pub fn remove_useModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_useModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleUseBeatmapButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleUseBeatmapButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetBeatmapUseButtonEnabledAsync(
        &mut self,
        getLevelEntitlementTask: *mut crate::System::Threading::Tasks::Task_1<
            EntitlementStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeatmapUseButtonEnabledAsync", (getLevelEntitlementTask))?;
        Ok(__cordl_ret)
    }
    pub fn add_kickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_kickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
        playerData: *mut ILobbyPlayerData,
        hasKickPermissions: bool,
        allowSelection: bool,
        getLevelEntitlementTask: *mut crate::System::Threading::Tasks::Task_1<
            EntitlementStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    connectedPlayer,
                    playerData,
                    hasKickPermissions,
                    allowSelection,
                    getLevelEntitlementTask,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleKickPlayerButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleKickPlayerButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleUseModifiersButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleUseModifiersButtonPressed", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl quest_hook::libil2cpp::ObjectType for GameServerPlayerTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
