#[cfg(feature = "ScoreModel")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ScoreModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ScoreModel => ""."ScoreModel"
);
#[cfg(feature = "ScoreModel")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel")]
impl crate::GlobalNamespace::ScoreModel {
    pub const kMaxAfterCutScore: i32 = 30i32;
    pub const kMaxBeforeCutScore: i32 = 70i32;
    pub const kMaxCenterDistanceCutScore: i32 = 15i32;
    pub const kMaxDistanceForDistanceToCenterScore: f32 = 0.3f32;
    pub const kMaxMultiplier: i32 = 8i32;
    #[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
    pub type MaxScoreCounterElement = crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement;
    #[cfg(feature = "ScoreModel+NoteScoreDefinition")]
    pub type NoteScoreDefinition = crate::GlobalNamespace::ScoreModel_NoteScoreDefinition;
    pub fn ComputeMaxMultipliedScoreForBeatmap(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeMaxMultipliedScoreForBeatmap", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeQuickInaccurateMaxMultipliedScoreForBeatmap(
        beatmapBasicData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapBasicData,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ComputeQuickInaccurateMaxMultipliedScoreForBeatmap",
                (beatmapBasicData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModifiedScoreForGameplayModifiersScoreMultiplier(
        multipliedScore: i32,
        gameplayModifiersScoreMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetModifiedScoreForGameplayModifiersScoreMultiplier",
                (multipliedScore, gameplayModifiersScoreMultiplier),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNoteScoreDefinition(
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ScoreModel_NoteScoreDefinition>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNoteScoreDefinition", (scoringType))?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "ScoreModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ScoreModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreModel_MaxScoreCounterElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub noteScoreDefinition: *mut crate::GlobalNamespace::ScoreModel_NoteScoreDefinition,
    pub _cordl_time: f32,
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScoreModel_MaxScoreCounterElement => ""
    ."ScoreModel/MaxScoreCounterElement"
);
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    pub fn CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scoringType, _cordl_time))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scoringType, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl AsRef<
    crate::System::IComparable_1<
        *mut crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
    >,
> for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        *mut crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ScoreModel+MaxScoreCounterElement")]
impl AsMut<
    crate::System::IComparable_1<
        *mut crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
    >,
> for crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        *mut crate::GlobalNamespace::ScoreModel_MaxScoreCounterElement,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreModel_NoteScoreDefinition {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub maxCenterDistanceCutScore: i32,
    pub minBeforeCutScore: i32,
    pub maxBeforeCutScore: i32,
    pub minAfterCutScore: i32,
    pub maxAfterCutScore: i32,
    pub fixedCutScore: i32,
}
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ScoreModel_NoteScoreDefinition
    => ""."ScoreModel/NoteScoreDefinition"
);
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
impl crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    pub fn New(
        maxCenterDistanceCutScore: i32,
        minBeforeCutScore: i32,
        maxBeforeCutScore: i32,
        minAfterCutScore: i32,
        maxAfterCutScore: i32,
        fixedCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    maxCenterDistanceCutScore,
                    minBeforeCutScore,
                    maxBeforeCutScore,
                    minAfterCutScore,
                    maxAfterCutScore,
                    fixedCutScore,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        maxCenterDistanceCutScore: i32,
        minBeforeCutScore: i32,
        maxBeforeCutScore: i32,
        minAfterCutScore: i32,
        maxAfterCutScore: i32,
        fixedCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    maxCenterDistanceCutScore,
                    minBeforeCutScore,
                    maxBeforeCutScore,
                    minAfterCutScore,
                    maxAfterCutScore,
                    fixedCutScore,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_executionOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_executionOrder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxCutScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxCutScore", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ScoreModel+NoteScoreDefinition")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScoreModel_NoteScoreDefinition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
