#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
#[repr(C)]
#[derive(Debug)]
pub struct IXboxOneRumble {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XInput::IXboxOneRumble
    => "UnityEngine.InputSystem.XInput"."IXboxOneRumble"
);
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    pub fn SetMotorSpeeds(
        &mut self,
        lowFrequency: f32,
        highFrequency: f32,
        leftTrigger: f32,
        rightTrigger: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetMotorSpeeds",
                (lowFrequency, highFrequency, leftTrigger, rightTrigger),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl AsRef<crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble>
for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl AsMut<crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble>
for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl AsRef<crate::UnityEngine::InputSystem::Haptics::IHaptics>
for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::Haptics::IHaptics {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
impl AsMut<crate::UnityEngine::InputSystem::Haptics::IHaptics>
for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::InputSystem::Haptics::IHaptics {
        unsafe { std::mem::transmute(self) }
    }
}
