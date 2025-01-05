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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRMixedRealityCaptureSettings
    => ""."OVRMixedRealityCaptureSettings"
);
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedRealityCaptureSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMixedRealityCaptureSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CameraDevice = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_capturingCameraDevice",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeyColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeyColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySimilarity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeySimilarity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySmoothRange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeySmoothRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySpillRange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeySpillRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_compositionMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_CompositionMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CompositionMethod = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_compositionMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_depthQuality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_DepthQuality> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_DepthQuality = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_depthQuality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicCullingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_dynamicCullingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicLightingDepthVariationClampingValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_dynamicLightingDepthVariationClampingValue",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicLightingSmoothFactor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_dynamicLightingSmoothFactor",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_enableMixedReality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_enableMixedReality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_externalCompositionBackdropColorQuest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorQuest",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_externalCompositionBackdropColorRift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorRift",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_extraHiddenLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_extraHiddenLayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_extraVisibleLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_extraVisibleLayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_flipCameraFrameHorizontally(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameHorizontally",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_flipCameraFrameVertically(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameVertically",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_handPoseStateLatency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_handPoseStateLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_instantiateMixedRealityCameraGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        > = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_instantiateMixedRealityCameraGameObject",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_mrcActivationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_MrcActivationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_MrcActivationMode = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_mrcActivationMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_sandwichCompositionBufferedFrames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionBufferedFrames",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_sandwichCompositionRenderLatency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionRenderLatency",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_useDynamicLighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_useDynamicLighting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenApplyDepthCulling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenApplyDepthCulling",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenBottomY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenBottomY",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenDepthTolerance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenDepthTolerance",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenTopY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenTopY",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenType",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_capturingCameraDevice(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_capturingCameraDevice",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeyColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.set_chromaKeyColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySimilarity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_chromaKeySimilarity",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySmoothRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_chromaKeySmoothRange",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySpillRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_chromaKeySpillRange",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_compositionMethod(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CompositionMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_compositionMethod",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_depthQuality(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_DepthQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.set_depthQuality", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicCullingMask(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_dynamicCullingMask",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicLightingDepthVariationClampingValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_dynamicLightingDepthVariationClampingValue",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicLightingSmoothFactor(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_dynamicLightingSmoothFactor",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_enableMixedReality(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_enableMixedReality",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_externalCompositionBackdropColorQuest(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorQuest",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_externalCompositionBackdropColorRift(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorRift",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_extraHiddenLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_extraHiddenLayers",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_extraVisibleLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_extraVisibleLayers",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_flipCameraFrameHorizontally(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameHorizontally",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_flipCameraFrameVertically(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameVertically",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_handPoseStateLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_handPoseStateLatency",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_instantiateMixedRealityCameraGameObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_instantiateMixedRealityCameraGameObject",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_mrcActivationMode(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_MrcActivationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_mrcActivationMode",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_sandwichCompositionBufferedFrames(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionBufferedFrames",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_sandwichCompositionRenderLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionRenderLatency",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_useDynamicLighting(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_useDynamicLighting",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenApplyDepthCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenApplyDepthCulling",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenBottomY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenBottomY",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenDepthTolerance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenDepthTolerance",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenTopY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenTopY",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenType(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenType",
                (value),
            )?;
        Ok(__cordl_ret.into())
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
