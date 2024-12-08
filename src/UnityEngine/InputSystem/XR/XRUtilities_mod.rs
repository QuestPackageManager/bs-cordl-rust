#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct XRUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::XRUtilities =>
    "UnityEngine.InputSystem.XR"."XRUtilities"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::XRUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XR::XRUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
impl crate::UnityEngine::InputSystem::XR::XRUtilities {
    pub const InterfaceCurrent: &'static str = "XRInputV1";
    pub const InterfaceMatchAnyVersion: &'static str = "^(XRInput)";
    pub const InterfaceV1: &'static str = "XRInput";
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XR::XRUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
