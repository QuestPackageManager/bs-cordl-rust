#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PerceivedLoudnessPerLevelModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _loudnessLevelPerLevelId: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            f32,
        >,
    >,
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerceivedLoudnessPerLevelModel
    => ""."PerceivedLoudnessPerLevelModel"
);
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl std::ops::Deref for crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    pub const kDefaultLoudness: f32 = -6f32;
    pub const kPerceivedLoudnessTarget: f32 = -11f32;
    pub const kSfxLoudnessTarget: f32 = -10f32;
    pub fn GetLoudnessByLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetLoudnessByLevelId", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoudnessByLevelIdOrNull(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f32> = __cordl_object
            .invoke("GetLoudnessByLevelIdOrNull", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoudnessCorrectionByLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetLoudnessCorrectionByLevelId", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxSfxVolumeByLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetMaxSfxVolumeByLevelId", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        loudnessPerLeveDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerceivedLoudnessSO>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loudnessPerLeveDataList))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        loudnessPerLeveDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerceivedLoudnessSO>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loudnessPerLeveDataList))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PerceivedLoudnessPerLevelModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerceivedLoudnessPerLevelModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
