#[cfg(feature = "OVRMixedReality")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMixedReality {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRMixedReality")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMixedReality => ""
    ."OVRMixedReality"
);
#[cfg(feature = "OVRMixedReality")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMixedReality {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedReality")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMixedReality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedReality")]
impl crate::GlobalNamespace::OVRMixedReality {}
#[cfg(feature = "OVRMixedReality")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRMixedReality {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
