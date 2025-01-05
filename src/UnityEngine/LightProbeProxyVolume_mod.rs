#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
#[repr(C)]
#[derive(Debug)]
pub struct LightProbeProxyVolume {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Behaviour>,
}
#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightProbeProxyVolume =>
    "UnityEngine"."LightProbeProxyVolume"
);
#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
impl std::ops::Deref for crate::UnityEngine::LightProbeProxyVolume {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Behaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
impl std::ops::DerefMut for crate::UnityEngine::LightProbeProxyVolume {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
impl crate::UnityEngine::LightProbeProxyVolume {}
#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::LightProbeProxyVolume {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
