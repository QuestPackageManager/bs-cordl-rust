#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraEvent {
    AfterDepthNormalsTexture = 3i32,
    AfterDepthTexture = 1i32,
    AfterEverything = 20i32,
    AfterFinalPass = 9i32,
    AfterForwardAlpha = 17i32,
    AfterForwardOpaque = 11i32,
    AfterGBuffer = 5i32,
    AfterHaloAndLensFlares = 24i32,
    AfterImageEffects = 19i32,
    AfterImageEffectsOpaque = 13i32,
    AfterLighting = 7i32,
    AfterReflections = 22i32,
    AfterSkybox = 15i32,
    BeforeDepthNormalsTexture = 2i32,
    BeforeDepthTexture = 0i32,
    BeforeFinalPass = 8i32,
    BeforeForwardAlpha = 16i32,
    BeforeForwardOpaque = 10i32,
    BeforeGBuffer = 4i32,
    BeforeHaloAndLensFlares = 23i32,
    BeforeImageEffects = 18i32,
    BeforeImageEffectsOpaque = 12i32,
    BeforeLighting = 6i32,
    BeforeReflections = 21i32,
    BeforeSkybox = 14i32,
}
#[cfg(feature = "UnityEngine+Rendering+CameraEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CameraEvent =>
    "UnityEngine.Rendering"."CameraEvent"
);
