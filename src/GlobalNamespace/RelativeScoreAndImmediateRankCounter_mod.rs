#[cfg(feature = "RelativeScoreAndImmediateRankCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct RelativeScoreAndImmediateRankCounter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameplayModifiersModel: *mut GameplayModifiersModelSO,
    pub _scoreController: *mut IScoreController,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub relativeScoreOrImmediateRankDidChangeEvent: *mut crate::System::Action,
    pub _relativeScore_k__BackingField: f32,
    pub _immediateRank_k__BackingField: crate::GlobalNamespace::RankModel_Rank,
}
#[cfg(feature = "RelativeScoreAndImmediateRankCounter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RelativeScoreAndImmediateRankCounter => ""
    ."RelativeScoreAndImmediateRankCounter"
);
#[cfg(feature = "RelativeScoreAndImmediateRankCounter")]
impl std::ops::Deref for RelativeScoreAndImmediateRankCounter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RelativeScoreAndImmediateRankCounter")]
impl std::ops::DerefMut for RelativeScoreAndImmediateRankCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RelativeScoreAndImmediateRankCounter")]
impl RelativeScoreAndImmediateRankCounter {
    pub fn HandleScoreDidChange(
        &mut self,
        scoreWithoutModifiers: i32,
        scoreWithModifiers: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleScoreDidChange",
                (scoreWithoutModifiers, scoreWithModifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateRelativeScoreAndImmediateRank(
        &mut self,
        score: i32,
        modifiedScore: i32,
        maxPossibleScore: i32,
        maxPossibleModifiedScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateRelativeScoreAndImmediateRank",
                (score, modifiedScore, maxPossibleScore, maxPossibleModifiedScore),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_immediateRank(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::RankModel_Rank> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::RankModel_Rank = __cordl_object
            .invoke("get_immediateRank", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_relativeScoreOrImmediateRankDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_relativeScoreOrImmediateRankDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_relativeScore(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_relativeScore", (value))?;
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
    pub fn add_relativeScoreOrImmediateRankDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_relativeScoreOrImmediateRankDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_relativeScore(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_relativeScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_immediateRank(
        &mut self,
        value: crate::GlobalNamespace::RankModel_Rank,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_immediateRank", (value))?;
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
#[cfg(feature = "RelativeScoreAndImmediateRankCounter")]
impl quest_hook::libil2cpp::ObjectType for RelativeScoreAndImmediateRankCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
