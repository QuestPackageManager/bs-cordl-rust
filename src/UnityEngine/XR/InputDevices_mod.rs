#[cfg(feature = "UnityEngine+XR+InputDevices")]
#[repr(C)]
#[derive(Debug)]
pub struct InputDevices {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputDevices =>
    "UnityEngine.XR"."InputDevices"
);
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl std::ops::Deref for crate::UnityEngine::XR::InputDevices {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl std::ops::DerefMut for crate::UnityEngine::XR::InputDevices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl crate::UnityEngine::XR::InputDevices {}
#[cfg(feature = "UnityEngine+XR+InputDevices")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::InputDevices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}