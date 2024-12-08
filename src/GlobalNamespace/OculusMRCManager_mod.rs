#[cfg(feature = "OculusMRCManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusMRCManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _vrPlatformHelper: *mut IVRPlatformHelper,
    pub _instantiateMixedRealityBackgroundCameraGameObject: *mut crate::System::Func_2<
        *mut crate::UnityEngine::GameObject,
        *mut crate::UnityEngine::GameObject,
    >,
    pub _instantiateMixedRealityForegroundCameraGameObject: *mut crate::System::Func_2<
        *mut crate::UnityEngine::GameObject,
        *mut crate::UnityEngine::GameObject,
    >,
    pub _enableMixedReality_k__BackingField: bool,
    pub _extraHiddenLayers_k__BackingField: crate::UnityEngine::LayerMask,
    pub _extraVisibleLayers_k__BackingField: crate::UnityEngine::LayerMask,
    pub _dynamicCullingMask_k__BackingField: bool,
    pub _compositionMethod_k__BackingField: crate::GlobalNamespace::OVRManager_CompositionMethod,
    pub _externalCompositionBackdropColorRift_k__BackingField: crate::UnityEngine::Color,
    pub _externalCompositionBackdropColorQuest_k__BackingField: crate::UnityEngine::Color,
    pub _capturingCameraDevice_k__BackingField: crate::GlobalNamespace::OVRManager_CameraDevice,
    pub _flipCameraFrameHorizontally_k__BackingField: bool,
    pub _flipCameraFrameVertically_k__BackingField: bool,
    pub _handPoseStateLatency_k__BackingField: f32,
    pub _sandwichCompositionRenderLatency_k__BackingField: f32,
    pub _sandwichCompositionBufferedFrames_k__BackingField: i32,
    pub _chromaKeyColor_k__BackingField: crate::UnityEngine::Color,
    pub _chromaKeySimilarity_k__BackingField: f32,
    pub _chromaKeySmoothRange_k__BackingField: f32,
    pub _chromaKeySpillRange_k__BackingField: f32,
    pub _useDynamicLighting_k__BackingField: bool,
    pub _depthQuality_k__BackingField: crate::GlobalNamespace::OVRManager_DepthQuality,
    pub _dynamicLightingSmoothFactor_k__BackingField: f32,
    pub _dynamicLightingDepthVariationClampingValue_k__BackingField: f32,
    pub _virtualGreenScreenType_k__BackingField: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    pub _virtualGreenScreenTopY_k__BackingField: f32,
    pub _virtualGreenScreenBottomY_k__BackingField: f32,
    pub _virtualGreenScreenApplyDepthCulling_k__BackingField: bool,
    pub _virtualGreenScreenDepthTolerance_k__BackingField: f32,
    pub _mrcActivationMode_k__BackingField: crate::GlobalNamespace::OVRManager_MrcActivationMode,
    pub _instantiateMixedRealityCameraGameObject: *mut crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
}
#[cfg(feature = "OculusMRCManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusMRCManager => ""."OculusMRCManager"
);
#[cfg(feature = "OculusMRCManager")]
impl std::ops::Deref for OculusMRCManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusMRCManager")]
impl std::ops::DerefMut for OculusMRCManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusMRCManager")]
impl OculusMRCManager {
    pub fn set_flipCameraFrameVertically(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flipCameraFrameVertically", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_dynamicCullingMask(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_dynamicCullingMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_extraHiddenLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("get_extraHiddenLayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_handPoseStateLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_handPoseStateLatency", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_handPoseStateLatency(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_handPoseStateLatency", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_dynamicLightingSmoothFactor(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dynamicLightingSmoothFactor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_dynamicLightingDepthVariationClampingValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_dynamicLightingDepthVariationClampingValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableMixedReality(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableMixedReality", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_virtualGreenScreenType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType = __cordl_object
            .invoke("get_virtualGreenScreenType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_chromaKeySimilarity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_chromaKeySimilarity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_compositionMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_CompositionMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CompositionMethod = __cordl_object
            .invoke("get_compositionMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        instantiateMixedRealityBackgroundCameraGameObject: *mut crate::System::Func_2<
            *mut crate::UnityEngine::GameObject,
            *mut crate::UnityEngine::GameObject,
        >,
        instantiateMixedRealityForegroundCameraGameObject: *mut crate::System::Func_2<
            *mut crate::UnityEngine::GameObject,
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    instantiateMixedRealityBackgroundCameraGameObject,
                    instantiateMixedRealityForegroundCameraGameObject,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_externalCompositionBackdropColorQuest(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_externalCompositionBackdropColorQuest", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_chromaKeySpillRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_chromaKeySpillRange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dynamicCullingMask(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dynamicCullingMask", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_externalCompositionBackdropColorRift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_externalCompositionBackdropColorRift", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_virtualGreenScreenTopY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualGreenScreenTopY", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useDynamicLighting(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useDynamicLighting", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_mrcActivationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_MrcActivationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_MrcActivationMode = __cordl_object
            .invoke("get_mrcActivationMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_dynamicLightingDepthVariationClampingValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dynamicLightingDepthVariationClampingValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_instantiateMixedRealityCameraGameObject(
        &mut self,
        value: *mut crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_instantiateMixedRealityCameraGameObject", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_extraVisibleLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("get_extraVisibleLayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_flipCameraFrameHorizontally(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flipCameraFrameHorizontally", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_virtualGreenScreenTopY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_virtualGreenScreenTopY", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_chromaKeySmoothRange(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_chromaKeySmoothRange", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useDynamicLighting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useDynamicLighting", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_virtualGreenScreenDepthTolerance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_virtualGreenScreenDepthTolerance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sandwichCompositionRenderLatency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_sandwichCompositionRenderLatency", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_extraHiddenLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_extraHiddenLayers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_compositionMethod(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CompositionMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_compositionMethod", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sandwichCompositionRenderLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sandwichCompositionRenderLatency", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_flipCameraFrameVertically(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_flipCameraFrameVertically", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_sandwichCompositionBufferedFrames(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sandwichCompositionBufferedFrames", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_capturingCameraDevice(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_capturingCameraDevice", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_sandwichCompositionBufferedFrames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_sandwichCompositionBufferedFrames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_depthQuality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_DepthQuality> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_DepthQuality = __cordl_object
            .invoke("get_depthQuality", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_instantiateMixedRealityCameraGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate = __cordl_object
            .invoke("get_instantiateMixedRealityCameraGameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_chromaKeyColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_chromaKeyColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_chromaKeySimilarity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_chromaKeySimilarity", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_enableMixedReality(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableMixedReality", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_capturingCameraDevice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_CameraDevice> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CameraDevice = __cordl_object
            .invoke("get_capturingCameraDevice", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_mrcActivationMode(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_MrcActivationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mrcActivationMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_flipCameraFrameHorizontally(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_flipCameraFrameHorizontally", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_depthQuality(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_DepthQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_depthQuality", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_virtualGreenScreenType(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualGreenScreenType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_externalCompositionBackdropColorRift(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_externalCompositionBackdropColorRift", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_virtualGreenScreenBottomY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualGreenScreenBottomY", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_virtualGreenScreenApplyDepthCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualGreenScreenApplyDepthCulling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_extraVisibleLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_extraVisibleLayers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_externalCompositionBackdropColorQuest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_externalCompositionBackdropColorQuest", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_chromaKeySpillRange(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_chromaKeySpillRange", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_chromaKeySmoothRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_chromaKeySmoothRange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_virtualGreenScreenDepthTolerance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualGreenScreenDepthTolerance", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_virtualGreenScreenApplyDepthCulling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_virtualGreenScreenApplyDepthCulling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dynamicLightingSmoothFactor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_dynamicLightingSmoothFactor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_chromaKeyColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_chromaKeyColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_virtualGreenScreenBottomY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_virtualGreenScreenBottomY", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateMixedRealityCameraGameObject(
        &mut self,
        mainCameraGameObject: *mut crate::UnityEngine::GameObject,
        cameraType: crate::GlobalNamespace::OVRManager_MrcCameraType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke(
                "InstantiateMixedRealityCameraGameObject",
                (mainCameraGameObject, cameraType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusMRCManager")]
impl quest_hook::libil2cpp::ObjectType for OculusMRCManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
