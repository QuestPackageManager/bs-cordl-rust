#[cfg(feature = "VRPlatformUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct VRPlatformUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "VRPlatformUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRPlatformUtils => ""
    ."VRPlatformUtils"
);
#[cfg(feature = "VRPlatformUtils")]
impl std::ops::Deref for crate::GlobalNamespace::VRPlatformUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRPlatformUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRPlatformUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRPlatformUtils")]
impl crate::GlobalNamespace::VRPlatformUtils {
    pub const kMenuButtonLeftHand: &'static str = "OpenXRPrimaryButtonLeftHand";
    pub const kMenuButtonOculusTouch: &'static str = "MenuButtonOculusTouch";
    pub const kMenuButtonRightHand: &'static str = "OpenXRPrimaryButtonRightHand";
    pub const kTriggerLeftHand: &'static str = "TriggerLeftHand";
    pub const kTriggerRightHand: &'static str = "TriggerRightHand";
}
#[cfg(feature = "VRPlatformUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::VRPlatformUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
