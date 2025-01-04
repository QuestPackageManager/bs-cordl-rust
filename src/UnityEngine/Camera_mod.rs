#[cfg(feature = "UnityEngine+Camera")]
#[repr(C)]
#[derive(Debug)]
pub struct Camera {
    __cordl_parent: crate::UnityEngine::Behaviour,
}
#[cfg(feature = "UnityEngine+Camera")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera => "UnityEngine"."Camera"
);
#[cfg(feature = "UnityEngine+Camera")]
impl std::ops::Deref for crate::UnityEngine::Camera {
    type Target = crate::UnityEngine::Behaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Camera")]
impl std::ops::DerefMut for crate::UnityEngine::Camera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Camera")]
impl crate::UnityEngine::Camera {
    pub const kMaxAperture: f32 = 32f32;
    pub const kMaxBladeCount: i32 = 11i32;
    pub const kMinAperture: f32 = 0.7f32;
    pub const kMinBladeCount: i32 = 3i32;
    #[cfg(feature = "UnityEngine+Camera+CameraCallback")]
    pub type CameraCallback = crate::UnityEngine::Camera_CameraCallback;
    #[cfg(feature = "UnityEngine+Camera+GateFitMode")]
    pub type GateFitMode = crate::UnityEngine::Camera_GateFitMode;
    #[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
    pub type GateFitParameters = crate::UnityEngine::Camera_GateFitParameters;
    #[cfg(feature = "UnityEngine+Camera+MonoOrStereoscopicEye")]
    pub type MonoOrStereoscopicEye = crate::UnityEngine::Camera_MonoOrStereoscopicEye;
    #[cfg(feature = "UnityEngine+Camera+ProjectionMatrixMode")]
    pub type ProjectionMatrixMode = crate::UnityEngine::Camera_ProjectionMatrixMode;
    #[cfg(feature = "UnityEngine+Camera+RenderRequest")]
    pub type RenderRequest = crate::UnityEngine::Camera_RenderRequest;
    #[cfg(feature = "UnityEngine+Camera+RenderRequestMode")]
    pub type RenderRequestMode = crate::UnityEngine::Camera_RenderRequestMode;
    #[cfg(feature = "UnityEngine+Camera+RenderRequestOutputSpace")]
    pub type RenderRequestOutputSpace = crate::UnityEngine::Camera_RenderRequestOutputSpace;
    #[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
    pub type SceneViewFilterMode = crate::UnityEngine::Camera_SceneViewFilterMode;
    #[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
    pub type StereoscopicEye = crate::UnityEngine::Camera_StereoscopicEye;
    pub fn AddCommandBuffer(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCommandBuffer", (evt, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCommandBufferAsync(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        queueType: crate::UnityEngine::Rendering::ComputeQueueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCommandBufferAsync", (evt, buffer, queueType))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCommandBufferAsyncImpl(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        queueType: crate::UnityEngine::Rendering::ComputeQueueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCommandBufferAsyncImpl", (evt, buffer, queueType))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCommandBufferImpl(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCommandBufferImpl", (evt, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFrustumCorners(
        &mut self,
        viewport: crate::UnityEngine::Rect,
        z: f32,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        outCorners: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateFrustumCorners", (viewport, z, eye, outCorners))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFrustumCornersInternal(
        &mut self,
        viewport: crate::UnityEngine::Rect,
        z: f32,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        outCorners: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateFrustumCornersInternal", (viewport, z, eye, outCorners))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFrustumCornersInternal_Injected(
        &mut self,
        viewport: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        z: f32,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        outCorners: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CalculateFrustumCornersInternal_Injected",
                (viewport, z, eye, outCorners),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateObliqueMatrix(
        &mut self,
        clipPlane: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("CalculateObliqueMatrix", (clipPlane))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateObliqueMatrix_Injected(
        &mut self,
        clipPlane: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateObliqueMatrix_Injected", (clipPlane, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateProjectionMatrixFromPhysicalProperties(
        output: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        focalLength: f32,
        sensorSize: crate::UnityEngine::Vector2,
        lensShift: crate::UnityEngine::Vector2,
        nearClip: f32,
        farClip: f32,
        gateFitParameters: crate::UnityEngine::Camera_GateFitParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CalculateProjectionMatrixFromPhysicalProperties",
                (
                    output,
                    focalLength,
                    sensorSize,
                    lensShift,
                    nearClip,
                    farClip,
                    gateFitParameters,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateProjectionMatrixFromPhysicalPropertiesInternal(
        output: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        focalLength: f32,
        sensorSize: crate::UnityEngine::Vector2,
        lensShift: crate::UnityEngine::Vector2,
        nearClip: f32,
        farClip: f32,
        gateAspect: f32,
        gateFitMode: crate::UnityEngine::Camera_GateFitMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CalculateProjectionMatrixFromPhysicalPropertiesInternal",
                (
                    output,
                    focalLength,
                    sensorSize,
                    lensShift,
                    nearClip,
                    farClip,
                    gateAspect,
                    gateFitMode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateProjectionMatrixFromPhysicalPropertiesInternal_Injected(
        output: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        focalLength: f32,
        sensorSize: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        lensShift: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        nearClip: f32,
        farClip: f32,
        gateAspect: f32,
        gateFitMode: crate::UnityEngine::Camera_GateFitMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CalculateProjectionMatrixFromPhysicalPropertiesInternal_Injected",
                (
                    output,
                    focalLength,
                    sensorSize,
                    lensShift,
                    nearClip,
                    farClip,
                    gateAspect,
                    gateFitMode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFrom", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyStereoDeviceProjectionMatrixToNonJittered(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyStereoDeviceProjectionMatrixToNonJittered", (eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn FieldOfViewToFocalLength(
        fieldOfView: f32,
        sensorSize: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FieldOfViewToFocalLength", (fieldOfView, sensorSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn FireOnPostRender(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FireOnPostRender", (cam))?;
        Ok(__cordl_ret.into())
    }
    pub fn FireOnPreCull(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FireOnPreCull", (cam))?;
        Ok(__cordl_ret.into())
    }
    pub fn FireOnPreRender(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FireOnPreRender", (cam))?;
        Ok(__cordl_ret.into())
    }
    pub fn FocalLengthToFieldOfView(
        focalLength: f32,
        sensorSize: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FocalLengthToFieldOfView", (focalLength, sensorSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllCameras(
        cameras: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllCameras", (cameras))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllCamerasCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllCamerasCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllCamerasImpl(
        cam: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllCamerasImpl", (cam))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraBufferWarnings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetCameraBufferWarnings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCommandBuffers(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Rendering::CommandBuffer,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Rendering::CommandBuffer,
            >,
        > = __cordl_object.invoke("GetCommandBuffers", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCullingParameters_Internal(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        stereoAware: bool,
        cullingParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
        managedCullingParametersSize: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCullingParameters_Internal",
                (camera, stereoAware, cullingParameters, managedCullingParametersSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFilterMode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetFilterMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFrustumPlaneSizeAt(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetFrustumPlaneSizeAt", (distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFrustumPlaneSizeAt_Injected(
        &mut self,
        distance: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetFrustumPlaneSizeAt_Injected", (distance, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGateFittedFieldOfView(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetGateFittedFieldOfView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGateFittedLensShift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetGateFittedLensShift", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGateFittedLensShift_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGateFittedLensShift_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerCullDistances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = __cordl_object.invoke("GetLayerCullDistances", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalSpaceAim(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetLocalSpaceAim", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalSpaceAim_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetLocalSpaceAim_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoNonJitteredProjectionMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("GetStereoNonJitteredProjectionMatrix", (eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoNonJitteredProjectionMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetStereoNonJitteredProjectionMatrix_Injected", (eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoProjectionMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("GetStereoProjectionMatrix", (eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoProjectionMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetStereoProjectionMatrix_Injected", (eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoViewMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("GetStereoViewMatrix", (eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoViewMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetStereoViewMatrix_Injected", (eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn HorizontalToVerticalFieldOfView(
        horizontalFieldOfView: f32,
        aspectRatio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "HorizontalToVerticalFieldOfView",
                (horizontalFieldOfView, aspectRatio),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnlyUsedForTesting1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnlyUsedForTesting1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnlyUsedForTesting2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnlyUsedForTesting2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAllCommandBuffers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAllCommandBuffers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCommandBuffer(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCommandBuffer", (evt, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCommandBufferImpl(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCommandBufferImpl", (evt, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCommandBuffers(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCommandBuffers", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderDontRestore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderDontRestore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemapEyeImpl(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        faceMask: i32,
        stereoEye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RenderToCubemapEyeImpl", (cubemap, faceMask, stereoEye))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemapImpl(
        &mut self,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        faceMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RenderToCubemapImpl", (tex, faceMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_Cubemap1(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RenderToCubemap", (cubemap))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_Cubemap_i32_0(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
        faceMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RenderToCubemap", (cubemap, faceMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_RenderTexture3(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RenderToCubemap", (cubemap))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_RenderTexture_i32_2(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        faceMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RenderToCubemap", (cubemap, faceMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_RenderTexture_i32_Camera_MonoOrStereoscopicEye4(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        faceMask: i32,
        stereoEye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RenderToCubemap", (cubemap, faceMask, stereoEye))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderWithShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        replacementTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderWithShader", (shader, replacementTag))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetAspect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAspect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetCullingMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetCullingMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetProjectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetProjectionMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetReplacementShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetReplacementShader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetStereoProjectionMatrices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetStereoProjectionMatrices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetStereoViewMatrices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetStereoViewMatrices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetTransparencySortSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTransparencySortSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetWorldToCameraMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetWorldToCameraMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Injected(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScreenPointToRay_Injected", (pos, eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Vector2_Camera_MonoOrStereoscopicEye0(
        &mut self,
        pos: crate::UnityEngine::Vector2,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Ray = __cordl_object
            .invoke("ScreenPointToRay", (pos, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Vector3_2(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Ray = __cordl_object
            .invoke("ScreenPointToRay", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Vector3_Camera_MonoOrStereoscopicEye1(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Ray = __cordl_object
            .invoke("ScreenPointToRay", (pos, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToViewportPoint(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ScreenToViewportPoint", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToViewportPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScreenToViewportPoint_Injected", (position, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToWorldPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ScreenToWorldPoint", (position, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToWorldPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScreenToWorldPoint_Injected", (position, eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToWorldPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ScreenToWorldPoint", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayerCullDistances(
        &mut self,
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayerCullDistances", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetReplacementShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        replacementTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReplacementShader", (shader, replacementTag))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoProjectionMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStereoProjectionMatrix", (eye, matrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoProjectionMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStereoProjectionMatrix_Injected", (eye, matrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoViewMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStereoViewMatrix", (eye, matrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoViewMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStereoViewMatrix_Injected", (eye, matrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersImpl(
        &mut self,
        color: crate::UnityEngine::RenderBuffer,
        depth: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetBuffersImpl", (color, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersImpl_Injected(
        &mut self,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
        depth: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetBuffersImpl_Injected", (color, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersMRTImpl(
        &mut self,
        color: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RenderBuffer>,
        >,
        depth: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetBuffersMRTImpl", (color, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersMRTImpl_Injected(
        &mut self,
        color: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RenderBuffer>,
        >,
        depth: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetBuffersMRTImpl_Injected", (color, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffers_Il2CppArray1(
        &mut self,
        colorBuffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RenderBuffer>,
        >,
        depthBuffer: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetBuffers", (colorBuffer, depthBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffers_RenderBuffer0(
        &mut self,
        colorBuffer: crate::UnityEngine::RenderBuffer,
        depthBuffer: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetBuffers", (colorBuffer, depthBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupCurrent(
        cur: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetupCurrent", (cur))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubmitBuiltInObjectIDRenderRequest(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        > = __cordl_object
            .invoke(
                "SubmitBuiltInObjectIDRenderRequest",
                (target, mipLevel, cubemapFace, depthSlice),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SubmitRenderRequest<RequestData>(
        &mut self,
        renderRequest: RequestData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        RequestData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubmitRenderRequest", (renderRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubmitRenderRequests(
        &mut self,
        renderRequests: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Camera_RenderRequest,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubmitRenderRequests", (renderRequests))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubmitRenderRequestsInternal(
        &mut self,
        requests: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubmitRenderRequestsInternal", (requests))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCullingParameters_ByRefMut0(
        &mut self,
        cullingParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetCullingParameters", (cullingParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCullingParameters__cordl_bool_ByRefMut1(
        &mut self,
        stereoAware: bool,
        cullingParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetCullingParameters", (stereoAware, cullingParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerticalToHorizontalFieldOfView(
        verticalFieldOfView: f32,
        aspectRatio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "VerticalToHorizontalFieldOfView",
                (verticalFieldOfView, aspectRatio),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Injected(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ViewportPointToRay_Injected", (pos, eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Vector2_Camera_MonoOrStereoscopicEye0(
        &mut self,
        pos: crate::UnityEngine::Vector2,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Ray = __cordl_object
            .invoke("ViewportPointToRay", (pos, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Vector3_2(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Ray = __cordl_object
            .invoke("ViewportPointToRay", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Vector3_Camera_MonoOrStereoscopicEye1(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Ray = __cordl_object
            .invoke("ViewportPointToRay", (pos, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToScreenPoint(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ViewportToScreenPoint", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToScreenPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ViewportToScreenPoint_Injected", (position, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToWorldPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ViewportToWorldPoint", (position, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToWorldPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ViewportToWorldPoint_Injected", (position, eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToWorldPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ViewportToWorldPoint", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("WorldToScreenPoint", (position, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WorldToScreenPoint_Injected", (position, eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("WorldToScreenPoint", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("WorldToViewportPoint", (position, eye))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WorldToViewportPoint_Injected", (position, eye, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("WorldToViewportPoint", (position))?;
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
    pub fn get_PreviewCullingLayer() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PreviewCullingLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = __cordl_object
            .invoke("get_activeTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actualRenderingPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderingPath> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderingPath = __cordl_object
            .invoke("get_actualRenderingPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allCameras() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Camera>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Camera>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_allCameras", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allCamerasCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_allCamerasCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allowDynamicResolution(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_allowDynamicResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allowHDR(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_allowHDR", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allowMSAA(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_allowMSAA", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_anamorphism(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_anamorphism", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_aperture(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_aperture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_areVRStereoViewMatricesWithinSingleCullTolerance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_areVRStereoViewMatricesWithinSingleCullTolerance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_aspect(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_aspect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_backgroundColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_backgroundColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_barrelClipping(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_barrelClipping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bladeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bladeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraToWorldMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_cameraToWorldMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraToWorldMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_cameraToWorldMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::CameraType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::CameraType = __cordl_object
            .invoke("get_cameraType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clearFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::CameraClearFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::CameraClearFlags = __cordl_object
            .invoke("get_clearFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clearStencilAfterLightingPass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_clearStencilAfterLightingPass", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_commandBufferCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_commandBufferCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cullingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_cullingMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_cullingMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_current() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_curvature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_curvature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_curvature_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_curvature_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_depth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_depth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_depthTextureMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::DepthTextureMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::DepthTextureMode = __cordl_object
            .invoke("get_depthTextureMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eventMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_eventMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_farClipPlane(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_farClipPlane", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fieldOfView(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fieldOfView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focalLength(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_focalLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_focusDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_forceIntoRenderTexture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_forceIntoRenderTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gateFit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_GateFitMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Camera_GateFitMode = __cordl_object
            .invoke("get_gateFit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_iso(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_iso", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layerCullDistances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = __cordl_object.invoke("get_layerCullDistances", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layerCullSpherical(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_layerCullSpherical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lensShift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_lensShift", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lensShift_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_lensShift_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_main() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_main", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nearClipPlane(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_nearClipPlane", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nonJitteredProjectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_nonJitteredProjectionMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nonJitteredProjectionMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_nonJitteredProjectionMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_opaqueSortMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::OpaqueSortMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::OpaqueSortMode = __cordl_object
            .invoke("get_opaqueSortMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_orthographic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_orthographic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_orthographicSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_orthographicSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideSceneCullingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("get_overrideSceneCullingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_pixelHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_pixelRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelRect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_pixelRect_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_pixelWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previousViewProjectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_previousViewProjectionMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previousViewProjectionMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_previousViewProjectionMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_projectionMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionMatrixMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_ProjectionMatrixMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Camera_ProjectionMatrixMode = __cordl_object
            .invoke("get_projectionMatrixMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_projectionMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_rect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_rect_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderingPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderingPath> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderingPath = __cordl_object
            .invoke("get_renderingPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scaledPixelHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_scaledPixelHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scaledPixelWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_scaledPixelWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = __cordl_object
            .invoke("get_scene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneCullingMask(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_sceneCullingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneViewFilterMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_SceneViewFilterMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Camera_SceneViewFilterMode = __cordl_object
            .invoke("get_sceneViewFilterMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scene_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_scene_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sensorSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_sensorSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sensorSize_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_sensorSize_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shutterSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shutterSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_skyboxMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_skyboxMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoActiveEye(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Camera_MonoOrStereoscopicEye = __cordl_object
            .invoke("get_stereoActiveEye", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoConvergence(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_stereoConvergence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_stereoEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoSeparation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_stereoSeparation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoTargetEye(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::StereoTargetEyeMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::StereoTargetEyeMask = __cordl_object
            .invoke("get_stereoTargetEye", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetDisplay(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_targetDisplay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = __cordl_object
            .invoke("get_targetTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transparencySortAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_transparencySortAxis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transparencySortAxis_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_transparencySortAxis_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transparencySortMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TransparencySortMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TransparencySortMode = __cordl_object
            .invoke("get_transparencySortMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useJitteredProjectionMatrixForTransparentRendering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useJitteredProjectionMatrixForTransparentRendering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useOcclusionCulling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useOcclusionCulling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usePhysicalProperties(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_usePhysicalProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_velocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_velocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_velocity_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_velocity_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldToCameraMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_worldToCameraMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldToCameraMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_worldToCameraMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_allowDynamicResolution(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_allowDynamicResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_allowHDR(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_allowHDR", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_allowMSAA(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_allowMSAA", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_anamorphism(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_anamorphism", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_aperture(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_aperture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_aspect(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_aspect", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_backgroundColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundColor_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_backgroundColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_barrelClipping(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_barrelClipping", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bladeCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bladeCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cameraType(
        &mut self,
        value: crate::UnityEngine::CameraType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cameraType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clearFlags(
        &mut self,
        value: crate::UnityEngine::CameraClearFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearFlags", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clearStencilAfterLightingPass(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearStencilAfterLightingPass", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cullingMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cullingMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cullingMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cullingMatrix", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cullingMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cullingMatrix_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_curvature(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_curvature", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_curvature_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_curvature_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_depth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_depth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_depthTextureMode(
        &mut self,
        value: crate::UnityEngine::DepthTextureMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_depthTextureMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eventMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_farClipPlane(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_farClipPlane", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fieldOfView(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fieldOfView", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_focalLength(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_focalLength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_focusDistance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_focusDistance", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_forceIntoRenderTexture(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_forceIntoRenderTexture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gateFit(
        &mut self,
        value: crate::UnityEngine::Camera_GateFitMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gateFit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_iso(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_iso", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_layerCullDistances(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layerCullDistances", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_layerCullSpherical(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layerCullSpherical", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lensShift(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lensShift", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lensShift_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lensShift_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nearClipPlane(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_nearClipPlane", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nonJitteredProjectionMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_nonJitteredProjectionMatrix", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nonJitteredProjectionMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_nonJitteredProjectionMatrix_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_opaqueSortMode(
        &mut self,
        value: crate::UnityEngine::Rendering::OpaqueSortMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_opaqueSortMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_orthographic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_orthographic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_orthographicSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_orthographicSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideSceneCullingMask(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideSceneCullingMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pixelRect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pixelRect", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pixelRect_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pixelRect_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_projectionMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_projectionMatrix", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_projectionMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_projectionMatrix_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rect", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rect_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rect_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderingPath(
        &mut self,
        value: crate::UnityEngine::RenderingPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_renderingPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scene(
        &mut self,
        value: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scene", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scene_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scene_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sensorSize(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sensorSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sensorSize_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sensorSize_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_shutterSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shutterSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stereoConvergence(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stereoConvergence", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stereoSeparation(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stereoSeparation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stereoTargetEye(
        &mut self,
        value: crate::UnityEngine::StereoTargetEyeMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stereoTargetEye", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetDisplay(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetDisplay", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetTexture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetTexture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transparencySortAxis(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transparencySortAxis", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transparencySortAxis_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transparencySortAxis_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transparencySortMode(
        &mut self,
        value: crate::UnityEngine::TransparencySortMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transparencySortMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useJitteredProjectionMatrixForTransparentRendering(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useJitteredProjectionMatrixForTransparentRendering", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useOcclusionCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useOcclusionCulling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_usePhysicalProperties(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_usePhysicalProperties", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_worldToCameraMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_worldToCameraMatrix", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_worldToCameraMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_worldToCameraMatrix_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Camera")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Camera {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Camera+CameraCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Camera_CameraCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Camera+CameraCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_CameraCallback =>
    "UnityEngine"."Camera/CameraCallback"
);
#[cfg(feature = "UnityEngine+Camera+CameraCallback")]
impl std::ops::Deref for crate::UnityEngine::Camera_CameraCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Camera+CameraCallback")]
impl std::ops::DerefMut for crate::UnityEngine::Camera_CameraCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Camera+CameraCallback")]
impl crate::UnityEngine::Camera_CameraCallback {
    pub fn Invoke(
        &mut self,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (cam))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Camera+CameraCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Camera_CameraCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_GateFitMode {
    #[default]
    Fill = 3i32,
    Horizontal = 2i32,
    None = 0i32,
    Overscan = 4i32,
    Vertical = 1i32,
}
#[cfg(feature = "UnityEngine+Camera+GateFitMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_GateFitMode => "UnityEngine"
    ."Camera/GateFitMode"
);
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Camera_GateFitParameters {
    pub _mode_k__BackingField: crate::UnityEngine::Camera_GateFitMode,
    pub _aspect_k__BackingField: f32,
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_GateFitParameters =>
    "UnityEngine"."Camera/GateFitParameters"
);
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Camera_GateFitParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
impl crate::UnityEngine::Camera_GateFitParameters {
    pub fn get_aspect(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_aspect",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_GateFitMode> {
        let __cordl_ret: crate::UnityEngine::Camera_GateFitMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Camera+MonoOrStereoscopicEye")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_MonoOrStereoscopicEye {
    #[default]
    Left = 0i32,
    Mono = 2i32,
    Right = 1i32,
}
#[cfg(feature = "UnityEngine+Camera+MonoOrStereoscopicEye")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_MonoOrStereoscopicEye =>
    "UnityEngine"."Camera/MonoOrStereoscopicEye"
);
#[cfg(feature = "UnityEngine+Camera+ProjectionMatrixMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_ProjectionMatrixMode {
    #[default]
    Explicit = 0i32,
    Implicit = 1i32,
    PhysicalPropertiesBased = 2i32,
}
#[cfg(feature = "UnityEngine+Camera+ProjectionMatrixMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_ProjectionMatrixMode =>
    "UnityEngine"."Camera/ProjectionMatrixMode"
);
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Camera_RenderRequest {
    pub m_CameraRenderMode: crate::UnityEngine::Camera_RenderRequestMode,
    pub m_ResultRT: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub m_OutputSpace: crate::UnityEngine::Camera_RenderRequestOutputSpace,
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_RenderRequest =>
    "UnityEngine"."Camera/RenderRequest"
);
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Camera_RenderRequest {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
impl crate::UnityEngine::Camera_RenderRequest {}
#[cfg(feature = "UnityEngine+Camera+RenderRequestMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_RenderRequestMode {
    #[default]
    BaseColor = 6i32,
    Depth = 2i32,
    DiffuseColor = 13i32,
    Emission = 9i32,
    EntityId = 5i32,
    Metallic = 8i32,
    None = 0i32,
    Normal = 10i32,
    ObjectId = 1i32,
    Occlusion = 12i32,
    Smoothness = 11i32,
    SpecularColor = 7i32,
    VertexNormal = 3i32,
    WorldPosition = 4i32,
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_RenderRequestMode =>
    "UnityEngine"."Camera/RenderRequestMode"
);
#[cfg(feature = "UnityEngine+Camera+RenderRequestOutputSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_RenderRequestOutputSpace {
    #[default]
    ScreenSpace = -1i32,
    UV0 = 0i32,
    UV1 = 1i32,
    UV2 = 2i32,
    UV3 = 3i32,
    UV4 = 4i32,
    UV5 = 5i32,
    UV6 = 6i32,
    UV7 = 7i32,
    UV8 = 8i32,
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestOutputSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_RenderRequestOutputSpace =>
    "UnityEngine"."Camera/RenderRequestOutputSpace"
);
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_SceneViewFilterMode {
    #[default]
    Off = 0i32,
    ShowFiltered = 1i32,
}
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_SceneViewFilterMode =>
    "UnityEngine"."Camera/SceneViewFilterMode"
);
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_StereoscopicEye {
    #[default]
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Camera_StereoscopicEye =>
    "UnityEngine"."Camera/StereoscopicEye"
);
