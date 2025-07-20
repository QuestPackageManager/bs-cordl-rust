#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
#[repr(C)]
#[derive(Debug)]
pub struct IXboxOneRumble {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+IXboxOneRumble")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::XInput::IXboxOneRumble {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.XInput";
    const CLASS_NAME: &'static str = "IXboxOneRumble";
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (f32, f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetMotorSpeeds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetMotorSpeeds", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (lowFrequency, highFrequency, leftTrigger, rightTrigger),
                )?
        };
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
