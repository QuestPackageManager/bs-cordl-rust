#[cfg(feature = "Unity+XR+OpenVR+ViveLighthouse")]
#[repr(C)]
#[derive(Debug)]
pub struct ViveLighthouse {
    __cordl_parent: crate::UnityEngine::InputSystem::TrackedDevice,
}
#[cfg(feature = "Unity+XR+OpenVR+ViveLighthouse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::OpenVR::ViveLighthouse =>
    "Unity.XR.OpenVR"."ViveLighthouse"
);
#[cfg(feature = "Unity+XR+OpenVR+ViveLighthouse")]
impl std::ops::Deref for crate::Unity::XR::OpenVR::ViveLighthouse {
    type Target = crate::UnityEngine::InputSystem::TrackedDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+OpenVR+ViveLighthouse")]
impl std::ops::DerefMut for crate::Unity::XR::OpenVR::ViveLighthouse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+OpenVR+ViveLighthouse")]
impl crate::Unity::XR::OpenVR::ViveLighthouse {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+OpenVR+ViveLighthouse")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::OpenVR::ViveLighthouse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
