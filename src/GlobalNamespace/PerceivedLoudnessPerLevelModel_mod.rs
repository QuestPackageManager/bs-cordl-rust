#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PerceivedLoudnessPerLevelModel {
    __cordl_parent: crate::System::Object,
    pub _loudnessLevelPerLevelId: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        f32,
    >,
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PerceivedLoudnessPerLevelModel => ""
    ."PerceivedLoudnessPerLevelModel"
);
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl std::ops::Deref for PerceivedLoudnessPerLevelModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl std::ops::DerefMut for PerceivedLoudnessPerLevelModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl PerceivedLoudnessPerLevelModel {
    pub const kDefaultLoudness: f32 = -6f32;
    pub const kPerceivedLoudnessTarget: f32 = -11f32;
    pub const kSfxLoudnessTarget: f32 = -10f32;
    pub fn GetLoudnessByLevelId(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetLoudnessByLevelId", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLoudnessByLevelIdOrNull(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f32> = __cordl_object
            .invoke("GetLoudnessByLevelIdOrNull", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLoudnessCorrectionByLevelId(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetLoudnessCorrectionByLevelId", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaxSfxVolumeByLevelId(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetMaxSfxVolumeByLevelId", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        loudnessPerLeveDataList: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut PerceivedLoudnessSO,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loudnessPerLeveDataList))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        loudnessPerLeveDataList: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut PerceivedLoudnessSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loudnessPerLeveDataList))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl quest_hook::libil2cpp::ObjectType for PerceivedLoudnessPerLevelModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
