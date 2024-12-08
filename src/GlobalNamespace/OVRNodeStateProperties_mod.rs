#[cfg(feature = "OVRNodeStateProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNodeStateProperties {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRNodeStateProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRNodeStateProperties => ""
    ."OVRNodeStateProperties"
);
#[cfg(feature = "OVRNodeStateProperties")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNodeStateProperties {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNodeStateProperties")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRNodeStateProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNodeStateProperties")]
impl crate::GlobalNamespace::OVRNodeStateProperties {}
#[cfg(feature = "OVRNodeStateProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRNodeStateProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
