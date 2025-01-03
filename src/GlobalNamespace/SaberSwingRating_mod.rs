#[cfg(feature = "SaberSwingRating")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberSwingRating {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SaberSwingRating")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberSwingRating => ""
    ."SaberSwingRating"
);
#[cfg(feature = "SaberSwingRating")]
impl std::ops::Deref for crate::GlobalNamespace::SaberSwingRating {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberSwingRating")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberSwingRating {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberSwingRating")]
impl crate::GlobalNamespace::SaberSwingRating {
    pub const kAfterCutAngleFor1Rating: f32 = 60f32;
    pub const kBeforeCutAngleFor1Rating: f32 = 100f32;
    pub const kMaxAfterCutSwingDuration: f32 = 0.4f32;
    pub const kMaxBeforeCutSwingDuration: f32 = 0.4f32;
    pub const kMaxNormalAngleDiff: f32 = 90f32;
    pub const kToleranceNormalAngleDiff: f32 = 75f32;
    pub fn AfterCutStepRating(
        angleDiff: f32,
        normalDiff: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AfterCutStepRating", (angleDiff, normalDiff))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeforeCutStepRating(
        angleDiff: f32,
        normalDiff: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeforeCutStepRating", (angleDiff, normalDiff))?;
        Ok(__cordl_ret.into())
    }
    pub fn NormalRating(normalDiff: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NormalRating", (normalDiff))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SaberSwingRating")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberSwingRating {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
