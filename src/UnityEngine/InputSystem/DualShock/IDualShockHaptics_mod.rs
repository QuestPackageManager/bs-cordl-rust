#[cfg(feature = "cordl_class_UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
#[repr(C)]
#[derive(Debug)]
pub struct IDualShockHaptics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.DualShock";
    const CLASS_NAME: &'static str = "IDualShockHaptics";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    pub fn SetLightBarColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetLightBarColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetLightBarColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl AsRef<crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble>
for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl AsMut<crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble>
for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl AsRef<crate::UnityEngine::InputSystem::Haptics::IHaptics>
for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::Haptics::IHaptics {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+IDualShockHaptics")]
impl AsMut<crate::UnityEngine::InputSystem::Haptics::IHaptics>
for crate::UnityEngine::InputSystem::DualShock::IDualShockHaptics {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::InputSystem::Haptics::IHaptics {
        unsafe { std::mem::transmute(self) }
    }
}
