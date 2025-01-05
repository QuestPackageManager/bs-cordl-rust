#[cfg(feature = "MultiplayerLeaderboardPanelItem")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLeaderboardPanelItem {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _playerNameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _scoreText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _positionText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _backgroundImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub _normalPlayerTextColor: crate::UnityEngine::Color,
    pub _failedPlayerTextColor: crate::UnityEngine::Color,
    pub _firstPlayerBackgroundColor: crate::UnityEngine::Color,
    pub _lastPlayerBackgroundColor: crate::UnityEngine::Color,
    pub _prevPosition: i32,
    pub _prevPlayerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _prevScore: i32,
    pub _prevFailed: bool,
    pub _prevNumberOfPlayers: i32,
}
#[cfg(feature = "MultiplayerLeaderboardPanelItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerLeaderboardPanelItem
    => ""."MultiplayerLeaderboardPanelItem"
);
#[cfg(feature = "MultiplayerLeaderboardPanelItem")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLeaderboardPanelItem {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLeaderboardPanelItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLeaderboardPanelItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLeaderboardPanelItem")]
impl crate::GlobalNamespace::MultiplayerLeaderboardPanelItem {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetData(
        &mut self,
        position: i32,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: i32,
        failed: bool,
        numberOfPlayers: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (position, playerName, score, failed, numberOfPlayers))?;
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
    pub fn set_hide(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hide", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerLeaderboardPanelItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLeaderboardPanelItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
