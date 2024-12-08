#[cfg(feature = "BeatmapCharacteristicExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapCharacteristicExtensions => ""
    ."BeatmapCharacteristicExtensions"
);
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl std::ops::Deref for BeatmapCharacteristicExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl std::ops::DerefMut for BeatmapCharacteristicExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl BeatmapCharacteristicExtensions {}
#[cfg(feature = "BeatmapCharacteristicExtensions")]
impl quest_hook::libil2cpp::ObjectType for BeatmapCharacteristicExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
