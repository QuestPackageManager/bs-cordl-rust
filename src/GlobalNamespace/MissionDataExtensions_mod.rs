#[cfg(feature = "MissionDataExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionDataExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MissionDataExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionDataExtensions => ""
    ."MissionDataExtensions"
);
#[cfg(feature = "MissionDataExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::MissionDataExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionDataExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataExtensions")]
impl crate::GlobalNamespace::MissionDataExtensions {}
#[cfg(feature = "MissionDataExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionDataExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
