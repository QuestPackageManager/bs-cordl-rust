#[cfg(feature = "Libraries+HM+HMLib+VR+HapticPresetSO")]
#[repr(C)]
#[derive(Debug)]
pub struct HapticPresetSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _duration: f32,
    pub _strength: f32,
    pub _frequency: f32,
    pub _continuous: bool,
    pub _useAdvancedHapticsOnSupportedPlatforms: bool,
    pub _ps5HapticsClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _hasPS5HapticsClip: crate::System::Nullable_1<bool>,
    pub _oculusHapticsClip: quest_hook::libil2cpp::Gc<
        crate::Oculus::Haptics::HapticClip,
    >,
    pub _priority: u32,
    pub _overrideForTouchController: bool,
    pub _touchControllerOverrideHapticsClip: quest_hook::libil2cpp::Gc<
        crate::Oculus::Haptics::HapticClip,
    >,
    pub _hasOculusHapticsClip: crate::System::Nullable_1<bool>,
}
#[cfg(feature = "Libraries+HM+HMLib+VR+HapticPresetSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Libraries::HM::HMLib::VR::HapticPresetSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Libraries.HM.HMLib.VR";
    const CLASS_NAME: &'static str = "HapticPresetSO";
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
#[cfg(feature = "Libraries+HM+HMLib+VR+HapticPresetSO")]
impl std::ops::Deref for crate::Libraries::HM::HMLib::VR::HapticPresetSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Libraries+HM+HMLib+VR+HapticPresetSO")]
impl std::ops::DerefMut for crate::Libraries::HM::HMLib::VR::HapticPresetSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Libraries+HM+HMLib+VR+HapticPresetSO")]
impl crate::Libraries::HM::HMLib::VR::HapticPresetSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasOculusHapticsClip(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasOculusHapticsClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasPS5HapticsClip(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPS5HapticsClip", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Libraries+HM+HMLib+VR+HapticPresetSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::Libraries::HM::HMLib::VR::HapticPresetSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
