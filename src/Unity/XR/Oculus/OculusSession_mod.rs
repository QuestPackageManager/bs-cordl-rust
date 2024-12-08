#[cfg(feature = "Unity+XR+Oculus+OculusSession")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusSession {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+XR+Oculus+OculusSession")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::OculusSession =>
    "Unity.XR.Oculus"."OculusSession"
);
#[cfg(feature = "Unity+XR+Oculus+OculusSession")]
impl std::ops::Deref for crate::Unity::XR::Oculus::OculusSession {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSession")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::OculusSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusSession")]
impl crate::Unity::XR::Oculus::OculusSession {}
#[cfg(feature = "Unity+XR+Oculus+OculusSession")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::OculusSession {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
