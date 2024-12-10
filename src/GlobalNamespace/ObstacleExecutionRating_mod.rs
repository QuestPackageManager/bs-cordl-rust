#[cfg(feature = "ObstacleExecutionRating")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleExecutionRating {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectExecutionRating,
    pub _rating_k__BackingField: crate::GlobalNamespace::ObstacleExecutionRating_Rating,
}
#[cfg(feature = "ObstacleExecutionRating")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObstacleExecutionRating => ""
    ."ObstacleExecutionRating"
);
#[cfg(feature = "ObstacleExecutionRating")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleExecutionRating {
    type Target = crate::GlobalNamespace::BeatmapObjectExecutionRating;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleExecutionRating")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleExecutionRating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleExecutionRating")]
impl crate::GlobalNamespace::ObstacleExecutionRating {
    #[cfg(feature = "ObstacleExecutionRating+Rating")]
    pub type Rating = crate::GlobalNamespace::ObstacleExecutionRating_Rating;
    pub fn New(
        _cordl_time: f32,
        rating: crate::GlobalNamespace::ObstacleExecutionRating_Rating,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, rating))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        rating: crate::GlobalNamespace::ObstacleExecutionRating_Rating,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, rating))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rating(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ObstacleExecutionRating_Rating,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ObstacleExecutionRating_Rating = __cordl_object
            .invoke("get_rating", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObstacleExecutionRating")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleExecutionRating {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ObstacleExecutionRating+Rating")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleExecutionRating_Rating {
    NotGood = 1i32,
    OK = 0i32,
}
#[cfg(feature = "ObstacleExecutionRating+Rating")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObstacleExecutionRating_Rating
    => ""."ObstacleExecutionRating/Rating"
);
