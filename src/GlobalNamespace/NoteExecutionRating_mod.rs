#[cfg(feature = "NoteExecutionRating")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteExecutionRating {
    __cordl_parent: BeatmapObjectExecutionRating,
    pub rating: crate::GlobalNamespace::NoteExecutionRating_Rating,
    pub cutScore: i32,
    pub beforeCutScore: i32,
    pub centerDistanceCutScore: i32,
    pub afterCutScore: i32,
    pub scoringType: crate::GlobalNamespace::NoteData_ScoringType,
}
#[cfg(feature = "NoteExecutionRating")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteExecutionRating => ""."NoteExecutionRating"
);
#[cfg(feature = "NoteExecutionRating")]
impl std::ops::Deref for NoteExecutionRating {
    type Target = BeatmapObjectExecutionRating;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteExecutionRating")]
impl std::ops::DerefMut for NoteExecutionRating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteExecutionRating")]
impl NoteExecutionRating {
    #[cfg(feature = "NoteExecutionRating+Rating")]
    pub type Rating = crate::GlobalNamespace::NoteExecutionRating_Rating;
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        rating: crate::GlobalNamespace::NoteExecutionRating_Rating,
        cutScore: i32,
        beforeCutScore: i32,
        centerDistanceCutScore: i32,
        afterCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    _cordl_time,
                    scoringType,
                    rating,
                    cutScore,
                    beforeCutScore,
                    centerDistanceCutScore,
                    afterCutScore,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        rating: crate::GlobalNamespace::NoteExecutionRating_Rating,
        cutScore: i32,
        beforeCutScore: i32,
        centerDistanceCutScore: i32,
        afterCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_time,
                    scoringType,
                    rating,
                    cutScore,
                    beforeCutScore,
                    centerDistanceCutScore,
                    afterCutScore,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "NoteExecutionRating")]
impl quest_hook::libil2cpp::ObjectType for NoteExecutionRating {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteExecutionRating+Rating")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteExecutionRating_Rating {
    BadCut = 2i32,
    GoodCut = 0i32,
    Miss = 1i32,
}
#[cfg(feature = "NoteExecutionRating+Rating")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteExecutionRating_Rating =>
    ""."NoteExecutionRating/Rating"
);
