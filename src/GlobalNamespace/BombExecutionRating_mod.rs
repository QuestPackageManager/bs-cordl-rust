#[cfg(feature = "BombExecutionRating")]
#[repr(C)]
#[derive(Debug)]
pub struct BombExecutionRating {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectExecutionRating,
    pub _rating_k__BackingField: crate::GlobalNamespace::BombExecutionRating_Rating,
}
#[cfg(feature = "BombExecutionRating")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BombExecutionRating => ""
    ."BombExecutionRating"
);
#[cfg(feature = "BombExecutionRating")]
impl std::ops::Deref for crate::GlobalNamespace::BombExecutionRating {
    type Target = crate::GlobalNamespace::BeatmapObjectExecutionRating;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BombExecutionRating")]
impl std::ops::DerefMut for crate::GlobalNamespace::BombExecutionRating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BombExecutionRating")]
impl crate::GlobalNamespace::BombExecutionRating {
    #[cfg(feature = "BombExecutionRating+Rating")]
    pub type Rating = crate::GlobalNamespace::BombExecutionRating_Rating;
    pub fn New(
        _cordl_time: f32,
        rating: crate::GlobalNamespace::BombExecutionRating_Rating,
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
        rating: crate::GlobalNamespace::BombExecutionRating_Rating,
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
        crate::GlobalNamespace::BombExecutionRating_Rating,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BombExecutionRating_Rating = __cordl_object
            .invoke("get_rating", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BombExecutionRating")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BombExecutionRating {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BombExecutionRating+Rating")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BombExecutionRating_Rating {
    #[default]
    NotGood = 1i32,
    OK = 0i32,
}
#[cfg(feature = "BombExecutionRating+Rating")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BombExecutionRating_Rating =>
    ""."BombExecutionRating/Rating"
);
