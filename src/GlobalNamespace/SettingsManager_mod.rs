#[cfg(feature = "SettingsManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub settings: crate::BeatSaber::Settings::Settings,
}
#[cfg(feature = "SettingsManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SettingsManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SettingsManager";
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
#[cfg(feature = "SettingsManager")]
impl std::ops::Deref for crate::GlobalNamespace::SettingsManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::SettingsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsManager")]
impl crate::GlobalNamespace::SettingsManager {
    pub fn AdjustPlatformSettings(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        platform: crate::GlobalNamespace::HardwareCategory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustPlatformSettings", (settings, platform))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstanceWithCurrentPlatformPreset() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstanceWithCurrentPlatformPreset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUninitialized() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUninitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentPlatformPreset() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::Settings::Settings,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentPlatformPreset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlatformPreset(
        platform: crate::GlobalNamespace::HardwareCategory,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::Settings::Settings,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlatformPreset", (platform))?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "SettingsManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SettingsManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
