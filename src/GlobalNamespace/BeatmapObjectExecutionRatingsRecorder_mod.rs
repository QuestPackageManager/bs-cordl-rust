#[cfg(feature = "BeatmapObjectExecutionRatingsRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectExecutionRatingsRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scoreController: *mut IScoreController,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
    pub _playerHeadAndObstacleInteraction: *mut PlayerHeadAndObstacleInteraction,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _beatmapObjectExecutionRatings: *mut crate::System::Collections::Generic::List_1<
        *mut BeatmapObjectExecutionRating,
    >,
    pub _hitObstacles: *mut crate::System::Collections::Generic::HashSet_1<
        *mut ObstacleController,
    >,
}
#[cfg(feature = "BeatmapObjectExecutionRatingsRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapObjectExecutionRatingsRecorder => ""
    ."BeatmapObjectExecutionRatingsRecorder"
);
#[cfg(feature = "BeatmapObjectExecutionRatingsRecorder")]
impl std::ops::Deref for BeatmapObjectExecutionRatingsRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectExecutionRatingsRecorder")]
impl std::ops::DerefMut for BeatmapObjectExecutionRatingsRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectExecutionRatingsRecorder")]
impl BeatmapObjectExecutionRatingsRecorder {
    pub fn HandleObstacleDidPassAvoidedMark(
        &mut self,
        obstacleController: *mut ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleDidPassAvoidedMark", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerHeadDidEnterObstacle(
        &mut self,
        obstacleController: *mut ObstacleController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerHeadDidEnterObstacle", (obstacleController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleScoringForNoteDidFinish(
        &mut self,
        scoringElement: *mut ScoringElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScoringForNoteDidFinish", (scoringElement))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_beatmapObjectExecutionRatings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut BeatmapObjectExecutionRating,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut BeatmapObjectExecutionRating,
        > = __cordl_object.invoke("get_beatmapObjectExecutionRatings", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectExecutionRatingsRecorder")]
impl quest_hook::libil2cpp::ObjectType for BeatmapObjectExecutionRatingsRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
