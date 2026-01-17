#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IDualMotorRumble {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Haptics";
    const CLASS_NAME: &'static str = "IDualMotorRumble";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    pub fn SetMotorSpeeds(
        &mut self,
        lowFrequency: f32,
        highFrequency: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32, f32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetMotorSpeeds",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetMotorSpeeds",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (lowFrequency, highFrequency))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl AsRef<crate::UnityEngine::InputSystem::Haptics::IHaptics>
    for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble
{
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::Haptics::IHaptics {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl AsMut<crate::UnityEngine::InputSystem::Haptics::IHaptics>
    for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::InputSystem::Haptics::IHaptics {
        unsafe { std::mem::transmute(self) }
    }
}
