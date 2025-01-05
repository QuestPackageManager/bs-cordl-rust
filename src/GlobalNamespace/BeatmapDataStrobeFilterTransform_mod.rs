#[cfg(feature = "BeatmapDataStrobeFilterTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataStrobeFilterTransform {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDataStrobeFilterTransform => ""
    ."BeatmapDataStrobeFilterTransform"
);
#[cfg(feature = "BeatmapDataStrobeFilterTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataStrobeFilterTransform {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataStrobeFilterTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform")]
impl crate::GlobalNamespace::BeatmapDataStrobeFilterTransform {
    pub const kMaxSecondsToConsiderStrobe: f32 = 0.1f32;
    #[cfg(feature = "BeatmapDataStrobeFilterTransform+StrobeStreakData")]
    pub type StrobeStreakData = crate::GlobalNamespace::BeatmapDataStrobeFilterTransform_StrobeStreakData;
    pub fn CreateTransformedData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        environmentIntensityReductionOptions: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentIntensityReductionOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateTransformedData",
                (beatmapData, environmentIntensityReductionOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFlashAndFadeToBlackEventDataValue(
        lightColorType: crate::GlobalNamespace::EnvironmentColorType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFlashAndFadeToBlackEventDataValue", (lightColorType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOnEventDataValue(
        lightColorType: crate::GlobalNamespace::EnvironmentColorType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOnEventDataValue", (lightColorType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataStrobeFilterTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform+StrobeStreakData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataStrobeFilterTransform_StrobeStreakData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub isActive: bool,
    pub strobeStartTime: f32,
    pub startColorType: crate::GlobalNamespace::EnvironmentColorType,
    pub lastSwitchTime: f32,
    pub lastColorType: crate::GlobalNamespace::EnvironmentColorType,
    pub lastIsOn: bool,
    pub originalBasicBeatmapEventData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BasicBeatmapEventData,
    >,
    pub _foundFirstColoredEventData: bool,
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform+StrobeStreakData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDataStrobeFilterTransform_StrobeStreakData => ""
    ."BeatmapDataStrobeFilterTransform/StrobeStreakData"
);
#[cfg(feature = "BeatmapDataStrobeFilterTransform+StrobeStreakData")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapDataStrobeFilterTransform_StrobeStreakData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform+StrobeStreakData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapDataStrobeFilterTransform_StrobeStreakData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataStrobeFilterTransform+StrobeStreakData")]
impl crate::GlobalNamespace::BeatmapDataStrobeFilterTransform_StrobeStreakData {
    pub fn AddStrobeData(
        &mut self,
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStrobeData", (basicBeatmapEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartPotentialStrobe(
        &mut self,
        startBasicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartPotentialStrobe", (startBasicBeatmapEventData))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatmapDataStrobeFilterTransform+StrobeStreakData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataStrobeFilterTransform_StrobeStreakData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
