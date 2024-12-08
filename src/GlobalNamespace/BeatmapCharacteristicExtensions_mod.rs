#[cfg(feature = "BeatmapCharacteristicExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapCharacteristicExtensions
    => ""."BeatmapCharacteristicExtensions"
);
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapCharacteristicExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapCharacteristicExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl crate::GlobalNamespace::BeatmapCharacteristicExtensions {}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCharacteristicExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
