#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QualitySettings {
    pub renderViewportScale: f32,
    pub vrResolutionScale: f32,
    pub menuVRResolutionScaleMultiplier: f32,
    pub antiAliasingLevel: i32,
    pub targetFramerate: i32,
    pub vSyncCount: i32,
    pub maxQueuedFrames: i32,
    pub mainEffect: crate::BeatSaber::Settings::QualitySettings_MainEffectOption,
    pub bloom: crate::BeatSaber::Settings::QualitySettings_BloomQuality,
    pub mirror: crate::BeatSaber::Settings::QualitySettings_MirrorQuality,
    pub obstacles: crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
    pub screenDisplacementEffects: bool,
    pub smokeGraphics: bool,
    pub depthTexture: bool,
    pub burnMarkTrails: bool,
    pub maxShockwaveParticles: i32,
    pub maxNumberOfCutSoundEffects: i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::QualitySettings =>
    "BeatSaber.Settings"."QualitySettings"
);
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::QualitySettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings")]
impl crate::BeatSaber::Settings::QualitySettings {
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
    pub type BloomQuality = crate::BeatSaber::Settings::QualitySettings_BloomQuality;
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
    pub type MainEffectOption = crate::BeatSaber::Settings::QualitySettings_MainEffectOption;
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
    pub type MirrorQuality = crate::BeatSaber::Settings::QualitySettings_MirrorQuality;
    #[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
    pub type ObstacleQuality = crate::BeatSaber::Settings::QualitySettings_ObstacleQuality;
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualitySettings_BloomQuality {
    Game = 0i32,
    LightBaking = 1i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+BloomQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::QualitySettings_BloomQuality => "BeatSaber.Settings"
    ."QualitySettings/BloomQuality"
);
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualitySettings_MainEffectOption {
    Game = 1i32,
    LightBaking = 2i32,
    Off = 0i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MainEffectOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::QualitySettings_MainEffectOption => "BeatSaber.Settings"
    ."QualitySettings/MainEffectOption"
);
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualitySettings_MirrorQuality {
    High = 3i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+MirrorQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::QualitySettings_MirrorQuality => "BeatSaber.Settings"
    ."QualitySettings/MirrorQuality"
);
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualitySettings_ObstacleQuality {
    High = 2i32,
    Low = 0i32,
    Medium = 1i32,
}
#[cfg(feature = "BeatSaber+Settings+QualitySettings+ObstacleQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::QualitySettings_ObstacleQuality => "BeatSaber.Settings"
    ."QualitySettings/ObstacleQuality"
);
