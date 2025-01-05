#[cfg(feature = "VRPlatformUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct VRPlatformUtils {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "VRPlatformUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRPlatformUtils => ""
    ."VRPlatformUtils"
);
#[cfg(feature = "VRPlatformUtils")]
impl std::ops::Deref for crate::GlobalNamespace::VRPlatformUtils {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn GetAnyJoystickMaxAxisDefaultImplementation(
        vrPlatformHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVRPlatformHelper,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAnyJoystickMaxAxisDefaultImplementation", (vrPlatformHelper))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMenuButtonDefaultImplementation() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMenuButtonDefaultImplementation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMenuButtonDownDefaultImplementation() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMenuButtonDownDefaultImplementation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopXR(
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVerboseLogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopXR", (logger))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerValueDefaultImplementation(
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriggerValueDefaultImplementation", (node))?;
        Ok(__cordl_ret.into())
    }
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
