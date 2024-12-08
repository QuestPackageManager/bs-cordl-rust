#[cfg(feature = "BeatmapEventTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventTypeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapEventTypeExtensions => ""
    ."BeatmapEventTypeExtensions"
);
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl std::ops::Deref for BeatmapEventTypeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl std::ops::DerefMut for BeatmapEventTypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl BeatmapEventTypeExtensions {}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for BeatmapEventTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
