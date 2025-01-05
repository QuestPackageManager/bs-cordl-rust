#[cfg(feature = "MultiplayerSettingsPanelController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerSettingsPanelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _serverCodeView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ServerCodeView,
    >,
    pub _spectateToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _connectionSettingsWrapper: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _spectateSettingsWrapper: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _toggleBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ToggleBinder>,
    pub _lobbyPlayerData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ILobbyPlayerData,
    >,
    pub _refreshed: bool,
    pub playerActiveStateChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<bool>,
    >,
}
#[cfg(feature = "MultiplayerSettingsPanelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerSettingsPanelController => ""
    ."MultiplayerSettingsPanelController"
);
#[cfg(feature = "MultiplayerSettingsPanelController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerSettingsPanelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSettingsPanelController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerSettingsPanelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerSettingsPanelController")]
impl crate::GlobalNamespace::MultiplayerSettingsPanelController {
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
    pub fn HideConnectionSettings(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideConnectionSettings", (hide))?;
        Ok(__cordl_ret.into())
    }
    pub fn HideSpectateSettings(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideSpectateSettings", (hide))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbyCode(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyCode", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbyPlayerDataModel(
        &mut self,
        lobbyPlayerData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILobbyPlayerData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyPlayerDataModel", (lobbyPlayerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLocalPlayerIsActiveState(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLocalPlayerIsActiveState", (isActive))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__14_0(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__14_0", (on))?;
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
    pub fn add_playerActiveStateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerActiveStateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerActiveStateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerActiveStateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerSettingsPanelController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerSettingsPanelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerSettingsPanelController")]
impl AsRef<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::MultiplayerSettingsPanelController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerSettingsPanelController")]
impl AsMut<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::MultiplayerSettingsPanelController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
