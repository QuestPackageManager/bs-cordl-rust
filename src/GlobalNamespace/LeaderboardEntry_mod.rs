#[cfg(feature = "LeaderboardEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scoreText: *mut crate::TMPro::TextMeshProUGUI,
    pub _playerNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _rankText: *mut crate::TMPro::TextMeshProUGUI,
    pub _color: crate::UnityEngine::Color,
}
#[cfg(feature = "LeaderboardEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LeaderboardEntry => ""."LeaderboardEntry"
);
#[cfg(feature = "LeaderboardEntry")]
impl std::ops::Deref for LeaderboardEntry {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardEntry")]
impl std::ops::DerefMut for LeaderboardEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardEntry")]
impl LeaderboardEntry {
    pub fn SetScore(
        &mut self,
        score: i32,
        playerName: *mut crate::System::String,
        rank: i32,
        highlighted: bool,
        showSeparator: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetScore", (score, playerName, rank, highlighted, showSeparator))?;
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
#[cfg(feature = "LeaderboardEntry")]
impl quest_hook::libil2cpp::ObjectType for LeaderboardEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
