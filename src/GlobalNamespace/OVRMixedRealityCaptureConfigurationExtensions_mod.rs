#[cfg(feature = "OVRMixedRealityCaptureConfigurationExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMixedRealityCaptureConfigurationExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl crate::GlobalNamespace::OVRMixedRealityCaptureConfigurationExtensions {
    pub fn ApplyTo(
        dest: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyTo", (dest, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFrom(
        dest: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
        source: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFrom", (dest, source))?;
        Ok(__cordl_ret.into())
    }
}
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
