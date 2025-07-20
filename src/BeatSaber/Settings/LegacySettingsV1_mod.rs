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
    pub language: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    pub customServerHostName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub enableMemoryTracker: bool,
    pub enableFPSCounter: bool,
    pub enableFPSRecorder: bool,
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Settings::LegacySettingsV1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "LegacySettingsV1";
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
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
impl std::ops::Deref for crate::BeatSaber::Settings::LegacySettingsV1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::LegacySettingsV1 {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+LegacySettingsV1")]
impl crate::BeatSaber::Settings::LegacySettingsV1 {
    pub fn ApplyTo(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::BeatSaber::Settings::Settings,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ApplyTo", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (settings))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
