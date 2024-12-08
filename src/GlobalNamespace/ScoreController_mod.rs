#[cfg(feature = "ScoreController")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameplayModifiersModel: *mut GameplayModifiersModelSO,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
    pub _gameEnergyCounter: *mut IGameEnergyCounter,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _goodCutScoringElementPool: *mut crate::GlobalNamespace::GoodCutScoringElement_Pool,
    pub _badCutScoringElementPool: *mut crate::GlobalNamespace::BadCutScoringElement_Pool,
    pub _missScoringElementPool: *mut crate::GlobalNamespace::MissScoringElement_Pool,
    pub _playerHeadAndObstacleInteraction: *mut PlayerHeadAndObstacleInteraction,
    pub scoreDidChangeEvent: *mut crate::System::Action_2<i32, i32>,
    pub multiplierDidChangeEvent: *mut crate::System::Action_2<i32, f32>,
    pub scoringForNoteStartedEvent: *mut crate::System::Action_1<*mut ScoringElement>,
    pub scoringForNoteFinishedEvent: *mut crate::System::Action_1<*mut ScoringElement>,
    pub _gameplayModifierParams: *mut crate::System::Collections::Generic::List_1<
        *mut GameplayModifierParamsSO,
    >,
    pub _invalidated: bool,
    pub _modifiedScore: i32,
    pub _multipliedScore: i32,
    pub _immediateMaxPossibleMultipliedScore: i32,
    pub _immediateMaxPossibleModifiedScore: i32,
    pub _prevMultiplierFromModifiers: f32,
    pub _maxScoreMultiplierCounter: *mut ScoreMultiplierCounter,
    pub _scoreMultiplierCounter: *mut ScoreMultiplierCounter,
    pub _sortedNoteTimesWithoutScoringElements: *mut crate::System::Collections::Generic::List_1<
        f32,
    >,
    pub _sortedScoringElementsWithoutMultiplier: *mut crate::System::Collections::Generic::List_1<
        *mut ScoringElement,
    >,
    pub _scoringElementsWithMultiplier: *mut crate::System::Collections::Generic::List_1<
        *mut ScoringElement,
    >,
    pub _scoringElementsToRemove: *mut crate::System::Collections::Generic::List_1<
        *mut ScoringElement,
    >,
}
#[cfg(feature = "ScoreController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ScoreController => ""."ScoreController"
);
#[cfg(feature = "ScoreController")]
impl std::ops::Deref for ScoreController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreController")]
impl std::ops::DerefMut for ScoreController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreController")]
impl ScoreController {
    pub fn remove_scoringForNoteStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_scoringForNoteStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_multipliedScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_multipliedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        noteController: *mut NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn get_immediateMaxPossibleModifiedScore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_immediateMaxPossibleModifiedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetEnabled(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEnabled", (enabled))?;
        Ok(__cordl_ret)
    }
    pub fn remove_multiplierDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_multiplierDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: *mut NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn DespawnScoringElement(
        &mut self,
        scoringElement: *mut ScoringElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnScoringElement", (scoringElement))?;
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
    pub fn get_invalidated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_invalidated", ())?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        noteController: *mut NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasSpawned", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerHeadDidEnterObstacles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerHeadDidEnterObstacles", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_multiplierDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_multiplierDidChangeEvent", (value))?;
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
    pub fn add_scoringForNoteFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_scoringForNoteFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_scoreDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_scoreDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_scoreDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_2<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_scoreDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_scoringForNoteFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_scoringForNoteFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_modifiedScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_modifiedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_immediateMaxPossibleMultipliedScore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_immediateMaxPossibleMultipliedScore", ())?;
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
    pub fn add_scoringForNoteStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ScoringElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_scoringForNoteStartedEvent", (value))?;
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
#[cfg(feature = "ScoreController")]
impl quest_hook::libil2cpp::ObjectType for ScoreController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
