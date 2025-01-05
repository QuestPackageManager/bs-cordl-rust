#[cfg(feature = "BeatmapEventTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventTypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapEventTypeExtensions =>
    ""."BeatmapEventTypeExtensions"
);
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventTypeExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventTypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl crate::GlobalNamespace::BeatmapEventTypeExtensions {
    pub fn IsCoreLightIntensityChangeEvent(
        basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCoreLightIntensityChangeEvent", (basicBeatmapEventType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEventTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
