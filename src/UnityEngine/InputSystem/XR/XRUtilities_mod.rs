#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct XRUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::XR::XRUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.XR";
    const CLASS_NAME: &'static str = "XRUtilities";
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
#[cfg(feature = "UnityEngine+InputSystem+XR+XRUtilities")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::XRUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
