#[cfg(feature = "OVRMixedRealityCaptureConfigurationExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMixedRealityCaptureConfigurationExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRMixedRealityCaptureConfigurationExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRMixedRealityCaptureConfigurationExtensions => ""
    ."OVRMixedRealityCaptureConfigurationExtensions"
);
#[cfg(feature = "OVRMixedRealityCaptureConfigurationExtensions")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRMixedRealityCaptureConfigurationExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureConfigurationExtensions")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRMixedRealityCaptureConfigurationExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureConfigurationExtensions")]
impl crate::GlobalNamespace::OVRMixedRealityCaptureConfigurationExtensions {}
#[cfg(feature = "OVRMixedRealityCaptureConfigurationExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRMixedRealityCaptureConfigurationExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
