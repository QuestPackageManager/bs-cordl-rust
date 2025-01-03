#[cfg(feature = "GameServerPlayerTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerPlayerTableCell {
    __cordl_parent: crate::GlobalNamespace::TableCellWithSeparator,
    pub _playerNameText: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedTextMeshPro>,
    pub _localPlayerBackgroundImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub _suggestedLevelText: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedTextMeshPro>,
    pub _suggestedCharacteristicIcon: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _suggestedDifficultyText: quest_hook::libil2cpp::Gc<
        crate::TMPro::TextMeshProUGUI,
    >,
    pub _emptySuggestedLevelText: quest_hook::libil2cpp::Gc<
        crate::HMUI::CurvedTextMeshPro,
    >,
    pub _suggestedModifiersList: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierInfoListItemsList,
    >,
    pub _emptySuggestedModifiersText: quest_hook::libil2cpp::Gc<
        crate::HMUI::CurvedTextMeshPro,
    >,
    pub _mutePlayerButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _kickPlayerButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _useBeatmapButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _useModifiersButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _useBeatmapButtonHoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    pub _muteToggle: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonSpriteSwapToggle>,
    pub _statusImageView: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _readyIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _spectatingIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _hostIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiersModelSO,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub kickPlayerEvent: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    pub useBeatmapEvent: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    pub useModifiersEvent: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    pub _getLevelEntitlementCancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
}
#[cfg(feature = "GameServerPlayerTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerPlayerTableCell => ""
    ."GameServerPlayerTableCell"
);
#[cfg(feature = "GameServerPlayerTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerPlayerTableCell {
    type Target = crate::GlobalNamespace::TableCellWithSeparator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerPlayerTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl crate::GlobalNamespace::GameServerPlayerTableCell {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleKickPlayerButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleKickPlayerButtonPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleUseBeatmapButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleUseBeatmapButtonPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleUseModifiersButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleUseModifiersButtonPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetBeatmapUseButtonEnabledAsync(
        &mut self,
        getLevelEntitlementTask: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeatmapUseButtonEnabledAsync", (getLevelEntitlementTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        hasKickPermissions: bool,
        allowSelection: bool,
        getLevelEntitlementTask: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
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
    pub fn add_kickPlayerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_kickPlayerEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_useBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_useBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_useModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_useModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_kickPlayerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_kickPlayerEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_useBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_useBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_useModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_useModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServerPlayerTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
