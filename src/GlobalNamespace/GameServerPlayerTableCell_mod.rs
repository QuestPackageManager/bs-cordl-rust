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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameServerPlayerTableCell {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameServerPlayerTableCell";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerPlayerTableCell {
    type Target = crate::GlobalNamespace::TableCellWithSeparator;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerPlayerTableCell {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayerTableCell")]
impl crate::GlobalNamespace::GameServerPlayerTableCell {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Awake",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleKickPlayerButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("HandleKickPlayerButtonPressed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleKickPlayerButtonPressed", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleUseBeatmapButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("HandleUseBeatmapButtonPressed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleUseBeatmapButtonPressed", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleUseModifiersButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("HandleUseModifiersButtonPressed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleUseModifiersButtonPressed", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::EntitlementStatus,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetBeatmapUseButtonEnabledAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBeatmapUseButtonEnabledAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (getLevelEntitlementTask))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IConnectedPlayer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ILobbyPlayerData,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task_1<
                                    crate::GlobalNamespace::EntitlementStatus,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "SetData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        connectedPlayer,
                        playerData,
                        hasKickPermissions,
                        allowSelection,
                        getLevelEntitlementTask,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_kickPlayerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_kickPlayerEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_kickPlayerEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_useBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_useBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_useBeatmapEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_useModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_useModifiersEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_useModifiersEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_kickPlayerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_kickPlayerEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_kickPlayerEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_useBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_useBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_useBeatmapEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_useModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_useModifiersEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_useModifiersEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
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
