#[cfg(feature = "LeaderboardTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardTableCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _rankText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _playerNameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _scoreText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _fullComboText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _normalColor: crate::UnityEngine::Color,
    pub _specialScoreColor: crate::UnityEngine::Color,
    pub _separatorImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
}
#[cfg(feature = "LeaderboardTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LeaderboardTableCell => ""
    ."LeaderboardTableCell"
);
#[cfg(feature = "LeaderboardTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardTableCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardTableCell")]
impl crate::GlobalNamespace::LeaderboardTableCell {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn set_playerName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rank(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rank", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_score(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_score", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showFullCombo(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showFullCombo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showSeparator(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showSeparator", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_specialScore(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_specialScore", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LeaderboardTableCell")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LeaderboardTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
