#[cfg(feature = "OVRMixedRealityCaptureSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMixedRealityCaptureSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub enableMixedReality: bool,
    pub extraHiddenLayers: crate::UnityEngine::LayerMask,
    pub extraVisibleLayers: crate::UnityEngine::LayerMask,
    pub dynamicCullingMask: bool,
    pub compositionMethod: crate::GlobalNamespace::OVRManager_CompositionMethod,
    pub externalCompositionBackdropColorRift: crate::UnityEngine::Color,
    pub externalCompositionBackdropColorQuest: crate::UnityEngine::Color,
    pub capturingCameraDevice: crate::GlobalNamespace::OVRManager_CameraDevice,
    pub flipCameraFrameHorizontally: bool,
    pub flipCameraFrameVertically: bool,
    pub handPoseStateLatency: f32,
    pub sandwichCompositionRenderLatency: f32,
    pub sandwichCompositionBufferedFrames: i32,
    pub chromaKeyColor: crate::UnityEngine::Color,
    pub chromaKeySimilarity: f32,
    pub chromaKeySmoothRange: f32,
    pub chromaKeySpillRange: f32,
    pub useDynamicLighting: bool,
    pub depthQuality: crate::GlobalNamespace::OVRManager_DepthQuality,
    pub dynamicLightingSmoothFactor: f32,
    pub dynamicLightingDepthVariationClampingValue: f32,
    pub virtualGreenScreenType: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    pub virtualGreenScreenTopY: f32,
    pub virtualGreenScreenBottomY: f32,
    pub virtualGreenScreenApplyDepthCulling: bool,
    pub virtualGreenScreenDepthTolerance: f32,
    pub mrcActivationMode: crate::GlobalNamespace::OVRManager_MrcActivationMode,
    pub _OVRMixedRealityCaptureConfiguration_instantiateMixedRealityCameraGameObject_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
    >,
}
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRMixedRealityCaptureSettings";
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
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_capturingCameraDevice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_CameraDevice> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRManager_CameraDevice,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_capturingCameraDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_capturingCameraDevice",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CameraDevice = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeyColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Color,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_chromaKeyColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_chromaKeyColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySimilarity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_chromaKeySimilarity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_chromaKeySimilarity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySmoothRange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_chromaKeySmoothRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_chromaKeySmoothRange",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySpillRange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_chromaKeySpillRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_chromaKeySpillRange",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_compositionMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_CompositionMethod,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRManager_CompositionMethod,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_compositionMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_compositionMethod",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CompositionMethod = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_depthQuality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_DepthQuality> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRManager_DepthQuality,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_depthQuality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_depthQuality",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRManager_DepthQuality = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicCullingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_dynamicCullingMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_dynamicCullingMask",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicLightingDepthVariationClampingValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_dynamicLightingDepthVariationClampingValue",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_dynamicLightingDepthVariationClampingValue",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicLightingSmoothFactor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_dynamicLightingSmoothFactor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_dynamicLightingSmoothFactor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_enableMixedReality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_enableMixedReality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_enableMixedReality",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_externalCompositionBackdropColorQuest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Color,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorQuest",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorQuest",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_externalCompositionBackdropColorRift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Color,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorRift",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorRift",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_extraHiddenLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::LayerMask,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_extraHiddenLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_extraHiddenLayers",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::LayerMask = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_extraVisibleLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::LayerMask,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_extraVisibleLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_extraVisibleLayers",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::LayerMask = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_flipCameraFrameHorizontally(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameHorizontally",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameHorizontally",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_flipCameraFrameVertically(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameVertically",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameVertically",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_handPoseStateLatency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_handPoseStateLatency")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_handPoseStateLatency",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_instantiateMixedRealityCameraGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
                        >,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_instantiateMixedRealityCameraGameObject",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_instantiateMixedRealityCameraGameObject",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_mrcActivationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_MrcActivationMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRManager_MrcActivationMode,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_mrcActivationMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_mrcActivationMode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRManager_MrcActivationMode = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_sandwichCompositionBufferedFrames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        i32,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionBufferedFrames",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionBufferedFrames",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_sandwichCompositionRenderLatency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionRenderLatency",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionRenderLatency",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_useDynamicLighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_useDynamicLighting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_useDynamicLighting",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenApplyDepthCulling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenApplyDepthCulling",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenApplyDepthCulling",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenBottomY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenBottomY",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenBottomY",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenDepthTolerance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenDepthTolerance",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenDepthTolerance",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenTopY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenTopY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenTopY",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
                        0usize,
                    >("OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_capturingCameraDevice(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRManager_CameraDevice),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_capturingCameraDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_capturingCameraDevice",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeyColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_chromaKeyColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_chromaKeyColor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySimilarity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_chromaKeySimilarity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_chromaKeySimilarity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySmoothRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_chromaKeySmoothRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_chromaKeySmoothRange",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySpillRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_chromaKeySpillRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_chromaKeySpillRange",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_compositionMethod(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CompositionMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRManager_CompositionMethod),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_compositionMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_compositionMethod",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_depthQuality(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_DepthQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRManager_DepthQuality),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_depthQuality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_depthQuality",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicCullingMask(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_dynamicCullingMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_dynamicCullingMask",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicLightingDepthVariationClampingValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_dynamicLightingDepthVariationClampingValue",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_dynamicLightingDepthVariationClampingValue",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicLightingSmoothFactor(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_dynamicLightingSmoothFactor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_dynamicLightingSmoothFactor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_enableMixedReality(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_enableMixedReality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_enableMixedReality",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_externalCompositionBackdropColorQuest(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorQuest",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorQuest",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_externalCompositionBackdropColorRift(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorRift",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorRift",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_extraHiddenLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::LayerMask),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_extraHiddenLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_extraHiddenLayers",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_extraVisibleLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::LayerMask),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_extraVisibleLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_extraVisibleLayers",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_flipCameraFrameHorizontally(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameHorizontally",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameHorizontally",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_flipCameraFrameVertically(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameVertically",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameVertically",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_handPoseStateLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_handPoseStateLatency")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_handPoseStateLatency",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_instantiateMixedRealityCameraGameObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_instantiateMixedRealityCameraGameObject",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_instantiateMixedRealityCameraGameObject",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_mrcActivationMode(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_MrcActivationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRManager_MrcActivationMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_mrcActivationMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_mrcActivationMode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_sandwichCompositionBufferedFrames(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionBufferedFrames",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionBufferedFrames",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_sandwichCompositionRenderLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionRenderLatency",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionRenderLatency",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_useDynamicLighting(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_useDynamicLighting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_useDynamicLighting",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenApplyDepthCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenApplyDepthCulling",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenApplyDepthCulling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenBottomY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenBottomY",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenBottomY",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenDepthTolerance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenDepthTolerance",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenDepthTolerance",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenTopY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenTopY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenTopY",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenType(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRManager_VirtualGreenScreenType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenType",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl AsRef<crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration>
for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl AsMut<crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration>
for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration {
        unsafe { std::mem::transmute(self) }
    }
}
