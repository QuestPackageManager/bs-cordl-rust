#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
#[repr(C)]
#[derive(Debug)]
pub struct LightProbeProxyVolume {
    __cordl_parent: crate::UnityEngine::Behaviour,
}
#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::LightProbeProxyVolume {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "LightProbeProxyVolume";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+LightProbeProxyVolume")]
impl std::ops::Deref for crate::UnityEngine::LightProbeProxyVolume {
    type Target = crate::UnityEngine::Behaviour;
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
