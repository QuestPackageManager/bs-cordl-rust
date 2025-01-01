#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
#[repr(C)]
#[derive(Debug)]
pub struct LegacySettingsV1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub roomCenterX: f32,
    pub roomCenterY: f32,
    pub roomCenterZ: f32,
    pub roomRotation: f32,
    pub controllerPositionX: f32,
    pub controllerPositionY: f32,
    pub controllerPositionZ: f32,
    pub controllerRotationX: f32,
    pub controllerRotationY: f32,
    pub controllerRotationZ: f32,
    pub controllersRumbleEnabled: bool,
    pub smoothCameraEnabled: i32,
    pub smoothCameraFieldOfView: f32,
    pub smoothCameraPositionSmooth: f32,
    pub smoothCameraRotationSmooth: f32,
    pub smoothCameraThirdPersonEnabled: i32,
    pub smoothCameraThirdPersonPositionX: f32,
    pub smoothCameraThirdPersonPositionY: f32,
    pub smoothCameraThirdPersonPositionZ: f32,
    pub smoothCameraThirdPersonEulerAnglesX: f32,
    pub smoothCameraThirdPersonEulerAnglesY: f32,
    pub smoothCameraThirdPersonEulerAnglesZ: f32,
    pub volume: f32,
    pub ambientVolumeScale: f32,
    pub overrideAudioLatency: bool,
    pub audioLatency: f32,
    pub language: *mut quest_hook::libil2cpp::Il2CppString,
    pub pauseButtonPressDurationLevel: i32,
    pub vrResolutionScale: f32,
    pub menuVRResolutionScaleMultiplier: f32,
    pub antiAliasingLevel: i32,
    pub targetFramerate: f32,
    pub mainEffectGraphicsSettings: i32,
    pub bloomGraphicsSettings: i32,
    pub mirrorGraphicsSettings: i32,
    pub screenDisplacementEffectsEnabled: bool,
    pub smokeGraphicsSettings: i32,
    pub burnMarkTrailsEnabled: bool,
    pub maxShockwaveParticles: i32,
    pub maxNumberOfCutSoundEffects: i32,
    pub windowMode: i32,
    pub windowResolutionWidth: i32,
    pub windowResolutionHeight: i32,
    pub useCustomServerEnvironment: bool,
    pub forceGameLiftServerEnvironment: bool,
    pub customServerHostName: *mut quest_hook::libil2cpp::Il2CppString,
    pub enableMemoryTracker: bool,
    pub enableFPSCounter: bool,
    pub enableFPSRecorder: bool,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::LegacySettingsV1 =>
    "BeatSaber.Settings"."LegacySettingsV1"
);
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::LegacySettingsV1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
impl crate::BeatSaber::Settings::LegacySettingsV1 {
    pub fn ApplyTo(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyTo", (settings))?;
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Settings::LegacySettingsV1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
