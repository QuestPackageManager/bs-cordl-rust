#[cfg(feature = "PlaybackHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct PlaybackHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PlaybackHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlaybackHelpers => ""
    ."PlaybackHelpers"
);
#[cfg(feature = "PlaybackHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::PlaybackHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlaybackHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlaybackHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlaybackHelpers")]
impl crate::GlobalNamespace::PlaybackHelpers {}
#[cfg(feature = "PlaybackHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlaybackHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
