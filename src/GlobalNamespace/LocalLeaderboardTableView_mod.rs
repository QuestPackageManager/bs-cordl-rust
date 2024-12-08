#[cfg(feature = "LocalLeaderboardTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardTableView {
    __cordl_parent: LeaderboardTableView,
}
#[cfg(feature = "LocalLeaderboardTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LocalLeaderboardTableView => ""
    ."LocalLeaderboardTableView"
);
#[cfg(feature = "LocalLeaderboardTableView")]
impl std::ops::Deref for LocalLeaderboardTableView {
    type Target = LeaderboardTableView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardTableView")]
impl std::ops::DerefMut for LocalLeaderboardTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardTableView")]
impl LocalLeaderboardTableView {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetScores(
        &mut self,
        scores: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::LocalLeaderboardsModel_ScoreData,
        >,
        specialScorePos: i32,
        maxNumberOfCells: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetScores", (scores, specialScorePos, maxNumberOfCells))?;
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
}
#[cfg(feature = "LocalLeaderboardTableView")]
impl quest_hook::libil2cpp::ObjectType for LocalLeaderboardTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
