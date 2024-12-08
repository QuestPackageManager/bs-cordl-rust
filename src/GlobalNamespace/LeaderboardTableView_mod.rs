#[cfg(feature = "LeaderboardTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: *mut crate::HMUI::TableView,
    pub _cellPrefab: *mut LeaderboardTableCell,
    pub _rowHeight: f32,
    pub _scores: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LeaderboardTableView_ScoreData,
    >,
    pub _specialScorePos: i32,
}
#[cfg(feature = "LeaderboardTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LeaderboardTableView => ""."LeaderboardTableView"
);
#[cfg(feature = "LeaderboardTableView")]
impl std::ops::Deref for LeaderboardTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardTableView")]
impl std::ops::DerefMut for LeaderboardTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardTableView")]
impl LeaderboardTableView {
    pub const kCellIdentifier: &'static str = "Cell";
    #[cfg(feature = "LeaderboardTableView+ScoreData")]
    pub type ScoreData = crate::GlobalNamespace::LeaderboardTableView_ScoreData;
    pub fn CellForIdx(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        row: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForIdx", (tableView, row))?;
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
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetScores(
        &mut self,
        scores: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LeaderboardTableView_ScoreData,
        >,
        specialScorePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetScores", (scores, specialScorePos))?;
        Ok(__cordl_ret)
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
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
#[cfg(feature = "LeaderboardTableView")]
impl quest_hook::libil2cpp::ObjectType for LeaderboardTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LeaderboardTableView+ScoreData")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardTableView_ScoreData {
    __cordl_parent: crate::System::Object,
    pub _score_k__BackingField: i32,
    pub _playerName_k__BackingField: *mut crate::System::String,
    pub _rank_k__BackingField: i32,
    pub _fullCombo_k__BackingField: bool,
}
#[cfg(feature = "LeaderboardTableView+ScoreData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LeaderboardTableView_ScoreData
    => ""."LeaderboardTableView/ScoreData"
);
#[cfg(feature = "LeaderboardTableView+ScoreData")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardTableView_ScoreData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardTableView+ScoreData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardTableView_ScoreData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardTableView+ScoreData")]
impl crate::GlobalNamespace::LeaderboardTableView_ScoreData {
    pub fn get_playerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerName", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_fullCombo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fullCombo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rank(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_rank", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_score(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_score", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_playerName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        score: i32,
        playerName: *mut crate::System::String,
        rank: i32,
        fullCombo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (score, playerName, rank, fullCombo))?;
        Ok(__cordl_ret)
    }
    pub fn set_fullCombo(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fullCombo", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        score: i32,
        playerName: *mut crate::System::String,
        rank: i32,
        fullCombo: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (score, playerName, rank, fullCombo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LeaderboardTableView+ScoreData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LeaderboardTableView_ScoreData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
