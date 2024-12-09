#[cfg(feature = "OVRExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRExtensions => ""
    ."OVRExtensions"
);
#[cfg(feature = "OVRExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OVRExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExtensions")]
impl crate::GlobalNamespace::OVRExtensions {}
#[cfg(feature = "OVRExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
