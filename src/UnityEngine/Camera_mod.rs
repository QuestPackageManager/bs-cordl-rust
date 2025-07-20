#[cfg(feature = "UnityEngine+Camera")]
#[repr(C)]
#[derive(Debug)]
pub struct Camera {
    __cordl_parent: crate::UnityEngine::Behaviour,
}
#[cfg(feature = "UnityEngine+Camera")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Camera {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::CameraEvent,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddCommandBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "AddCommandBuffer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCommandBufferAsync(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        queueType: crate::UnityEngine::Rendering::ComputeQueueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::CameraEvent,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                    crate::UnityEngine::Rendering::ComputeQueueType,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddCommandBufferAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "AddCommandBufferAsync", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt, buffer, queueType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCommandBufferAsyncImpl(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        queueType: crate::UnityEngine::Rendering::ComputeQueueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::CameraEvent,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                    crate::UnityEngine::Rendering::ComputeQueueType,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddCommandBufferAsyncImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "AddCommandBufferAsyncImpl", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt, buffer, queueType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCommandBufferImpl(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::CameraEvent,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddCommandBufferImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "AddCommandBufferImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt, buffer))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rect,
                    f32,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("CalculateFrustumCorners")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CalculateFrustumCorners", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewport, z, eye, outCorners))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFrustumCornersInternal(
        &mut self,
        viewport: crate::UnityEngine::Rect,
        z: f32,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        outCorners: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rect,
                    f32,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("CalculateFrustumCornersInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CalculateFrustumCornersInternal", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewport, z, eye, outCorners))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFrustumCornersInternal_Injected(
        &mut self,
        viewport: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        z: f32,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        outCorners: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                    f32,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("CalculateFrustumCornersInternal_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CalculateFrustumCornersInternal_Injected", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewport, z, eye, outCorners))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateObliqueMatrix(
        &mut self,
        clipPlane: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector4),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("CalculateObliqueMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CalculateObliqueMatrix", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, (clipPlane))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateObliqueMatrix_Injected(
        &mut self,
        clipPlane: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CalculateObliqueMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CalculateObliqueMatrix_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (clipPlane, ret))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    f32,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    f32,
                    f32,
                    crate::UnityEngine::Camera_GateFitParameters,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("CalculateProjectionMatrixFromPhysicalProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CalculateProjectionMatrixFromPhysicalProperties", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        output,
                        focalLength,
                        sensorSize,
                        lensShift,
                        nearClip,
                        farClip,
                        gateFitParameters,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    f32,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    f32,
                    f32,
                    f32,
                    crate::UnityEngine::Camera_GateFitMode,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("CalculateProjectionMatrixFromPhysicalPropertiesInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CalculateProjectionMatrixFromPhysicalPropertiesInternal",
                    8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    f32,
                    f32,
                    f32,
                    crate::UnityEngine::Camera_GateFitMode,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("CalculateProjectionMatrixFromPhysicalPropertiesInternal_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(),
                    "CalculateProjectionMatrixFromPhysicalPropertiesInternal_Injected",
                    8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CopyFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CopyFrom", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyStereoDeviceProjectionMatrixToNonJittered(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Camera_StereoscopicEye),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CopyStereoDeviceProjectionMatrixToNonJittered")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "CopyStereoDeviceProjectionMatrixToNonJittered", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FieldOfViewToFocalLength(
        fieldOfView: f32,
        sensorSize: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), f32, 2usize>("FieldOfViewToFocalLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "FieldOfViewToFocalLength", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (fieldOfView, sensorSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FireOnPostRender(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FireOnPostRender")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "FireOnPostRender", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FireOnPreCull(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FireOnPreCull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "FireOnPreCull", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FireOnPreRender(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FireOnPreRender")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "FireOnPreRender", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FocalLengthToFieldOfView(
        focalLength: f32,
        sensorSize: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), f32, 2usize>("FocalLengthToFieldOfView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "FocalLengthToFieldOfView", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (focalLength, sensorSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllCameras(
        cameras: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    >,
                >),
                i32,
                1usize,
            >("GetAllCameras")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetAllCameras", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (cameras))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllCamerasCount() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("GetAllCamerasCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetAllCamerasCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllCamerasImpl(
        cam: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        >,
                    >,
                >),
                i32,
                1usize,
            >("GetAllCamerasImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetAllCamerasImpl", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (cam))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraBufferWarnings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                0usize,
            >("GetCameraBufferWarnings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetCameraBufferWarnings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCommandBuffers(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rendering::CameraEvent),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >,
                    >,
                >,
                1usize,
            >("GetCommandBuffers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetCommandBuffers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
            >,
        > = unsafe { method.invoke_unchecked(self, (evt))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::ScriptableCullingParameters,
                    >,
                    i32,
                ),
                bool,
                4usize,
            >("GetCullingParameters_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetCullingParameters_Internal", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        camera,
                        stereoAware,
                        cullingParameters,
                        managedCullingParametersSize,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFilterMode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetFilterMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetFilterMode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFrustumPlaneSizeAt(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                crate::UnityEngine::Vector2,
                1usize,
            >("GetFrustumPlaneSizeAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetFrustumPlaneSizeAt", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFrustumPlaneSizeAt_Injected(
        &mut self,
        distance: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetFrustumPlaneSizeAt_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetFrustumPlaneSizeAt_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (distance, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGateFittedFieldOfView(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("GetGateFittedFieldOfView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetGateFittedFieldOfView", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGateFittedLensShift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("GetGateFittedLensShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetGateFittedLensShift", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGateFittedLensShift_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetGateFittedLensShift_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetGateFittedLensShift_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerCullDistances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                0usize,
            >("GetLayerCullDistances")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetLayerCullDistances", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalSpaceAim(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("GetLocalSpaceAim")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetLocalSpaceAim", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalSpaceAim_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetLocalSpaceAim_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetLocalSpaceAim_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoNonJitteredProjectionMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Camera_StereoscopicEye),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("GetStereoNonJitteredProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetStereoNonJitteredProjectionMatrix", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, (eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoNonJitteredProjectionMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Camera_StereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetStereoNonJitteredProjectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetStereoNonJitteredProjectionMatrix_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoProjectionMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Camera_StereoscopicEye),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("GetStereoProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetStereoProjectionMatrix", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, (eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoProjectionMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Camera_StereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetStereoProjectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetStereoProjectionMatrix_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoViewMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Camera_StereoscopicEye),
                crate::UnityEngine::Matrix4x4,
                1usize,
            >("GetStereoViewMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetStereoViewMatrix", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, (eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStereoViewMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Camera_StereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetStereoViewMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "GetStereoViewMatrix_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HorizontalToVerticalFieldOfView(
        horizontalFieldOfView: f32,
        aspectRatio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32),
                f32,
                2usize,
            >("HorizontalToVerticalFieldOfView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "HorizontalToVerticalFieldOfView", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (horizontalFieldOfView, aspectRatio))?
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
    pub fn OnlyUsedForTesting1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("OnlyUsedForTesting1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "OnlyUsedForTesting1", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnlyUsedForTesting2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("OnlyUsedForTesting2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "OnlyUsedForTesting2", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAllCommandBuffers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RemoveAllCommandBuffers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RemoveAllCommandBuffers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCommandBuffer(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::CameraEvent,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RemoveCommandBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RemoveCommandBuffer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCommandBufferImpl(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::CameraEvent,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::CommandBuffer,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RemoveCommandBufferImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RemoveCommandBufferImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCommandBuffers(
        &mut self,
        evt: crate::UnityEngine::Rendering::CameraEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rendering::CameraEvent),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveCommandBuffers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RemoveCommandBuffers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Render")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "Render", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDontRestore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RenderDontRestore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderDontRestore", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemapEyeImpl(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        faceMask: i32,
        stereoEye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    i32,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                bool,
                3usize,
            >("RenderToCubemapEyeImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderToCubemapEyeImpl", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (cubemap, faceMask, stereoEye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemapImpl(
        &mut self,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        faceMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>, i32),
                bool,
                2usize,
            >("RenderToCubemapImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderToCubemapImpl", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (tex, faceMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_Cubemap1(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>),
                bool,
                1usize,
            >("RenderToCubemap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderToCubemap", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (cubemap))? };
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_Cubemap_i32_0(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
        faceMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>, i32),
                bool,
                2usize,
            >("RenderToCubemap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderToCubemap", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (cubemap, faceMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_RenderTexture3(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                bool,
                1usize,
            >("RenderToCubemap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderToCubemap", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (cubemap))? };
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_RenderTexture_i32_2(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        faceMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>, i32),
                bool,
                2usize,
            >("RenderToCubemap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderToCubemap", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (cubemap, faceMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderToCubemap_RenderTexture_i32_Camera_MonoOrStereoscopicEye4(
        &mut self,
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        faceMask: i32,
        stereoEye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    i32,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                bool,
                3usize,
            >("RenderToCubemap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderToCubemap", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (cubemap, faceMask, stereoEye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderWithShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        replacementTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RenderWithShader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "RenderWithShader", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (shader, replacementTag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAspect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetAspect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetAspect", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetCullingMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetCullingMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetCullingMatrix", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetProjectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetProjectionMatrix", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetReplacementShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetReplacementShader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetReplacementShader", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetStereoProjectionMatrices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetStereoProjectionMatrices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetStereoProjectionMatrices", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetStereoViewMatrices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetStereoViewMatrices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetStereoViewMatrices", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetTransparencySortSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetTransparencySortSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetTransparencySortSettings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetWorldToCameraMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetWorldToCameraMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ResetWorldToCameraMatrix", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Injected(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ScreenPointToRay_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenPointToRay_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pos, eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Vector2_Camera_MonoOrStereoscopicEye0(
        &mut self,
        pos: crate::UnityEngine::Vector2,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Ray,
                2usize,
            >("ScreenPointToRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenPointToRay", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Ray = unsafe {
            method.invoke_unchecked(self, (pos, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Vector3_2(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Ray,
                1usize,
            >("ScreenPointToRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenPointToRay", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Ray = unsafe {
            method.invoke_unchecked(self, (pos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay_Vector3_Camera_MonoOrStereoscopicEye1(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Ray,
                2usize,
            >("ScreenPointToRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenPointToRay", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Ray = unsafe {
            method.invoke_unchecked(self, (pos, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToViewportPoint(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("ScreenToViewportPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenToViewportPoint", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToViewportPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ScreenToViewportPoint_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenToViewportPoint_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToWorldPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Vector3,
                2usize,
            >("ScreenToWorldPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenToWorldPoint", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToWorldPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ScreenToWorldPoint_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenToWorldPoint_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToWorldPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("ScreenToWorldPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ScreenToWorldPoint", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLayerCullDistances(
        &mut self,
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetLayerCullDistances")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetLayerCullDistances", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (d))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetReplacementShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        replacementTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetReplacementShader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetReplacementShader", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (shader, replacementTag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoProjectionMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Camera_StereoscopicEye,
                    crate::UnityEngine::Matrix4x4,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetStereoProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetStereoProjectionMatrix", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye, matrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoProjectionMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Camera_StereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetStereoProjectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetStereoProjectionMatrix_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye, matrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoViewMatrix(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Camera_StereoscopicEye,
                    crate::UnityEngine::Matrix4x4,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetStereoViewMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetStereoViewMatrix", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye, matrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetStereoViewMatrix_Injected(
        &mut self,
        eye: crate::UnityEngine::Camera_StereoscopicEye,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Camera_StereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetStereoViewMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetStereoViewMatrix_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eye, matrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersImpl(
        &mut self,
        color: crate::UnityEngine::RenderBuffer,
        depth: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::RenderBuffer, crate::UnityEngine::RenderBuffer),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTargetBuffersImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetTargetBuffersImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (color, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersImpl_Injected(
        &mut self,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
        depth: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTargetBuffersImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetTargetBuffersImpl_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (color, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersMRTImpl(
        &mut self,
        color: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RenderBuffer>,
        >,
        depth: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::RenderBuffer,
                        >,
                    >,
                    crate::UnityEngine::RenderBuffer,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTargetBuffersMRTImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetTargetBuffersMRTImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (color, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffersMRTImpl_Injected(
        &mut self,
        color: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RenderBuffer>,
        >,
        depth: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::RenderBuffer,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTargetBuffersMRTImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetTargetBuffersMRTImpl_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (color, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffers_Il2CppArray1(
        &mut self,
        colorBuffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RenderBuffer>,
        >,
        depthBuffer: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::RenderBuffer,
                        >,
                    >,
                    crate::UnityEngine::RenderBuffer,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTargetBuffers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetTargetBuffers", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colorBuffer, depthBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTargetBuffers_RenderBuffer0(
        &mut self,
        colorBuffer: crate::UnityEngine::RenderBuffer,
        depthBuffer: crate::UnityEngine::RenderBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::RenderBuffer, crate::UnityEngine::RenderBuffer),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTargetBuffers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetTargetBuffers", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colorBuffer, depthBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCurrent(
        cur: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetupCurrent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SetupCurrent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cur))?
        };
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
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    i32,
                    crate::UnityEngine::CubemapFace,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >,
                4usize,
            >("SubmitBuiltInObjectIDRenderRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SubmitBuiltInObjectIDRenderRequest", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = unsafe {
            method.invoke_unchecked(self, (target, mipLevel, cubemapFace, depthSlice))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (RequestData),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SubmitRenderRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SubmitRenderRequest", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (renderRequest))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::Camera_RenderRequest,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SubmitRenderRequests")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SubmitRenderRequests", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (renderRequests))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitRenderRequestsInternal(
        &mut self,
        requests: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SubmitRenderRequestsInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "SubmitRenderRequestsInternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (requests))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCullingParameters_ByRefMut0(
        &mut self,
        cullingParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Rendering::ScriptableCullingParameters,
                >),
                bool,
                1usize,
            >("TryGetCullingParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "TryGetCullingParameters", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (cullingParameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCullingParameters__cordl_bool_ByRefMut1(
        &mut self,
        stereoAware: bool,
        cullingParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::ScriptableCullingParameters,
                    >,
                ),
                bool,
                2usize,
            >("TryGetCullingParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "TryGetCullingParameters", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (stereoAware, cullingParameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerticalToHorizontalFieldOfView(
        verticalFieldOfView: f32,
        aspectRatio: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32),
                f32,
                2usize,
            >("VerticalToHorizontalFieldOfView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "VerticalToHorizontalFieldOfView", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (verticalFieldOfView, aspectRatio))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Injected(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ViewportPointToRay_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportPointToRay_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pos, eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Vector2_Camera_MonoOrStereoscopicEye0(
        &mut self,
        pos: crate::UnityEngine::Vector2,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Ray,
                2usize,
            >("ViewportPointToRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportPointToRay", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Ray = unsafe {
            method.invoke_unchecked(self, (pos, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Vector3_2(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Ray,
                1usize,
            >("ViewportPointToRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportPointToRay", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Ray = unsafe {
            method.invoke_unchecked(self, (pos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportPointToRay_Vector3_Camera_MonoOrStereoscopicEye1(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Ray,
                2usize,
            >("ViewportPointToRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportPointToRay", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Ray = unsafe {
            method.invoke_unchecked(self, (pos, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToScreenPoint(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("ViewportToScreenPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportToScreenPoint", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToScreenPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ViewportToScreenPoint_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportToScreenPoint_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToWorldPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Vector3,
                2usize,
            >("ViewportToWorldPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportToWorldPoint", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToWorldPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ViewportToWorldPoint_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportToWorldPoint_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewportToWorldPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("ViewportToWorldPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "ViewportToWorldPoint", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Vector3,
                2usize,
            >("WorldToScreenPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "WorldToScreenPoint", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WorldToScreenPoint_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "WorldToScreenPoint_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("WorldToScreenPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "WorldToScreenPoint", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportPoint_Camera_MonoOrStereoscopicEye0(
        &mut self,
        position: crate::UnityEngine::Vector3,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                ),
                crate::UnityEngine::Vector3,
                2usize,
            >("WorldToViewportPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "WorldToViewportPoint", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportPoint_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        eye: crate::UnityEngine::Camera_MonoOrStereoscopicEye,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WorldToViewportPoint_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "WorldToViewportPoint_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position, eye, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportPoint_Vector3_1(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("WorldToViewportPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "WorldToViewportPoint", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (position))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_PreviewCullingLayer() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_PreviewCullingLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_PreviewCullingLayer", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_activeTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                0usize,
            >("get_activeTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_activeTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_actualRenderingPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderingPath> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::RenderingPath,
                0usize,
            >("get_actualRenderingPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_actualRenderingPath", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::RenderingPath = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_allCameras() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    >,
                >,
                0usize,
            >("get_allCameras")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_allCameras", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_allCamerasCount() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_allCamerasCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_allCamerasCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_allowDynamicResolution(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_allowDynamicResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_allowDynamicResolution", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_allowHDR(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_allowHDR")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_allowHDR", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_allowMSAA(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_allowMSAA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_allowMSAA", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_anamorphism(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_anamorphism")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_anamorphism", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_aperture(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_aperture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_aperture", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_areVRStereoViewMatricesWithinSingleCullTolerance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                bool,
                0usize,
            >("get_areVRStereoViewMatricesWithinSingleCullTolerance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_areVRStereoViewMatricesWithinSingleCullTolerance",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_aspect(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_aspect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_aspect", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Color, 0usize>("get_backgroundColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_backgroundColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_backgroundColor_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_backgroundColor_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_barrelClipping(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_barrelClipping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_barrelClipping", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_bladeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_bladeCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_bladeCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraToWorldMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Matrix4x4,
                0usize,
            >("get_cameraToWorldMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_cameraToWorldMatrix", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraToWorldMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_cameraToWorldMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_cameraToWorldMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::CameraType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::CameraType, 0usize>("get_cameraType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_cameraType", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::CameraType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_clearFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::CameraClearFlags> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::CameraClearFlags,
                0usize,
            >("get_clearFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_clearFlags", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::CameraClearFlags = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_clearStencilAfterLightingPass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_clearStencilAfterLightingPass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_clearStencilAfterLightingPass", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_commandBufferCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_commandBufferCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_commandBufferCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_cullingMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_cullingMask", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Matrix4x4,
                0usize,
            >("get_cullingMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_cullingMatrix", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_cullingMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_cullingMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_current() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                0usize,
            >("get_current")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_current", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_curvature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_curvature")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_curvature", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_curvature_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_curvature_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_curvature_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_depth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_depth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_depth", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_depthTextureMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::DepthTextureMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::DepthTextureMode,
                0usize,
            >("get_depthTextureMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_depthTextureMode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::DepthTextureMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_eventMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_eventMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_eventMask", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_farClipPlane(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_farClipPlane")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_farClipPlane", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_fieldOfView(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_fieldOfView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_fieldOfView", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_focalLength(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_focalLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_focalLength", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_focusDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_focusDistance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_focusDistance", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_forceIntoRenderTexture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_forceIntoRenderTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_forceIntoRenderTexture", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_gateFit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_GateFitMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Camera_GateFitMode,
                0usize,
            >("get_gateFit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_gateFit", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Camera_GateFitMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_iso(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_iso")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_iso", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_layerCullDistances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                0usize,
            >("get_layerCullDistances")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_layerCullDistances", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_layerCullSpherical(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_layerCullSpherical")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_layerCullSpherical", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lensShift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_lensShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_lensShift", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_lensShift_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_lensShift_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_lensShift_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_main() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                0usize,
            >("get_main")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_main", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_nearClipPlane(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_nearClipPlane")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_nearClipPlane", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_nonJitteredProjectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Matrix4x4,
                0usize,
            >("get_nonJitteredProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_nonJitteredProjectionMatrix", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_nonJitteredProjectionMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_nonJitteredProjectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_nonJitteredProjectionMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_opaqueSortMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::OpaqueSortMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Rendering::OpaqueSortMode,
                0usize,
            >("get_opaqueSortMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_opaqueSortMode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::OpaqueSortMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_orthographic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_orthographic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_orthographic", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_orthographicSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_orthographicSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_orthographicSize", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideSceneCullingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u64, 0usize>("get_overrideSceneCullingMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_overrideSceneCullingMask", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_pixelHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_pixelHeight", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Rect, 0usize>("get_pixelRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_pixelRect", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelRect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_pixelRect_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_pixelRect_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_pixelWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_pixelWidth", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_previousViewProjectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Matrix4x4,
                0usize,
            >("get_previousViewProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_previousViewProjectionMatrix", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_previousViewProjectionMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_previousViewProjectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_previousViewProjectionMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Matrix4x4,
                0usize,
            >("get_projectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_projectionMatrix", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionMatrixMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_ProjectionMatrixMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Camera_ProjectionMatrixMode,
                0usize,
            >("get_projectionMatrixMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_projectionMatrixMode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Camera_ProjectionMatrixMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_projectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_projectionMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_rect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Rect, 0usize>("get_rect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_rect", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_rect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_rect_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_rect_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_renderingPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderingPath> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::RenderingPath,
                0usize,
            >("get_renderingPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_renderingPath", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::RenderingPath = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_scaledPixelHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_scaledPixelHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_scaledPixelHeight", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_scaledPixelWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_scaledPixelWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_scaledPixelWidth", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_scene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::SceneManagement::Scene,
                0usize,
            >("get_scene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_scene", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneCullingMask(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u64, 0usize>("get_sceneCullingMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_sceneCullingMask", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneViewFilterMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_SceneViewFilterMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Camera_SceneViewFilterMode,
                0usize,
            >("get_sceneViewFilterMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_sceneViewFilterMode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Camera_SceneViewFilterMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_scene_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::SceneManagement::Scene,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_scene_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_scene_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_sensorSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector2, 0usize>("get_sensorSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_sensorSize", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_sensorSize_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_sensorSize_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_sensorSize_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_shutterSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_shutterSpeed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_shutterSpeed", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_skyboxMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                0usize,
            >("get_skyboxMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_skyboxMaterial", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoActiveEye(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Camera_MonoOrStereoscopicEye,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Camera_MonoOrStereoscopicEye,
                0usize,
            >("get_stereoActiveEye")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_stereoActiveEye", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Camera_MonoOrStereoscopicEye = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoConvergence(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_stereoConvergence")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_stereoConvergence", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_stereoEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_stereoEnabled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoSeparation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_stereoSeparation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_stereoSeparation", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoTargetEye(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::StereoTargetEyeMask> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::StereoTargetEyeMask,
                0usize,
            >("get_stereoTargetEye")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_stereoTargetEye", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::StereoTargetEyeMask = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_targetDisplay(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_targetDisplay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_targetDisplay", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_targetTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                0usize,
            >("get_targetTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_targetTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_transparencySortAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector3,
                0usize,
            >("get_transparencySortAxis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_transparencySortAxis", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_transparencySortAxis_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_transparencySortAxis_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_transparencySortAxis_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_transparencySortMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TransparencySortMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::TransparencySortMode,
                0usize,
            >("get_transparencySortMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_transparencySortMode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::TransparencySortMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_useJitteredProjectionMatrixForTransparentRendering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                bool,
                0usize,
            >("get_useJitteredProjectionMatrixForTransparentRendering")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_useJitteredProjectionMatrixForTransparentRendering",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_useOcclusionCulling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_useOcclusionCulling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_useOcclusionCulling", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_usePhysicalProperties(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_usePhysicalProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_usePhysicalProperties", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_velocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("get_velocity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_velocity", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_velocity_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_velocity_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_velocity_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_worldToCameraMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Matrix4x4,
                0usize,
            >("get_worldToCameraMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_worldToCameraMatrix", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_worldToCameraMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_worldToCameraMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "get_worldToCameraMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_allowDynamicResolution(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_allowDynamicResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_allowDynamicResolution", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_allowHDR(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_allowHDR")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_allowHDR", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_allowMSAA(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_allowMSAA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_allowMSAA", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_anamorphism(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_anamorphism")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_anamorphism", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_aperture(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_aperture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_aperture", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_aspect(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_aspect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_aspect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_backgroundColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_backgroundColor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundColor_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_backgroundColor_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_backgroundColor_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_barrelClipping(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_barrelClipping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_barrelClipping", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_bladeCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_bladeCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_bladeCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_cameraType(
        &mut self,
        value: crate::UnityEngine::CameraType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::CameraType),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_cameraType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_cameraType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_clearFlags(
        &mut self,
        value: crate::UnityEngine::CameraClearFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::CameraClearFlags),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_clearFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_clearFlags", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_clearStencilAfterLightingPass(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_clearStencilAfterLightingPass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_clearStencilAfterLightingPass", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_cullingMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_cullingMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_cullingMask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_cullingMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Matrix4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_cullingMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_cullingMatrix", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_cullingMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_cullingMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_cullingMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_curvature(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_curvature")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_curvature", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_curvature_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_curvature_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_curvature_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_depth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_depth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_depth", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_depthTextureMode(
        &mut self,
        value: crate::UnityEngine::DepthTextureMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::DepthTextureMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_depthTextureMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_depthTextureMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_eventMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_eventMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_eventMask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_farClipPlane(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_farClipPlane")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_farClipPlane", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_fieldOfView(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_fieldOfView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_fieldOfView", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_focalLength(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_focalLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_focalLength", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_focusDistance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_focusDistance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_focusDistance", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_forceIntoRenderTexture(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_forceIntoRenderTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_forceIntoRenderTexture", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_gateFit(
        &mut self,
        value: crate::UnityEngine::Camera_GateFitMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Camera_GateFitMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_gateFit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_gateFit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_iso(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_iso")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_iso", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_layerCullDistances(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_layerCullDistances")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_layerCullDistances", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_layerCullSpherical(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_layerCullSpherical")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_layerCullSpherical", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_lensShift(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_lensShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_lensShift", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_lensShift_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_lensShift_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_lensShift_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_nearClipPlane(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_nearClipPlane")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_nearClipPlane", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_nonJitteredProjectionMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Matrix4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_nonJitteredProjectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_nonJitteredProjectionMatrix", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_nonJitteredProjectionMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_nonJitteredProjectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_nonJitteredProjectionMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_opaqueSortMode(
        &mut self,
        value: crate::UnityEngine::Rendering::OpaqueSortMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rendering::OpaqueSortMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_opaqueSortMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_opaqueSortMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_orthographic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_orthographic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_orthographic", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_orthographicSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_orthographicSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_orthographicSize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideSceneCullingMask(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_overrideSceneCullingMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_overrideSceneCullingMask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_pixelRect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_pixelRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_pixelRect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_pixelRect_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_pixelRect_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_pixelRect_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_projectionMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Matrix4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_projectionMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_projectionMatrix", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_projectionMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_projectionMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_projectionMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_rect(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_rect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_rect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_rect_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_rect_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_rect_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_renderingPath(
        &mut self,
        value: crate::UnityEngine::RenderingPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::RenderingPath),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_renderingPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_renderingPath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_scene(
        &mut self,
        value: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::SceneManagement::Scene),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_scene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_scene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_scene_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::SceneManagement::Scene,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_scene_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_scene_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_sensorSize(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_sensorSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_sensorSize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_sensorSize_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_sensorSize_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_sensorSize_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_shutterSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_shutterSpeed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_shutterSpeed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_stereoConvergence(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_stereoConvergence")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_stereoConvergence", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_stereoSeparation(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_stereoSeparation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_stereoSeparation", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_stereoTargetEye(
        &mut self,
        value: crate::UnityEngine::StereoTargetEyeMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::StereoTargetEyeMask),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_stereoTargetEye")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_stereoTargetEye", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_targetDisplay(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_targetDisplay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_targetDisplay", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_targetTexture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_targetTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_targetTexture", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_transparencySortAxis(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_transparencySortAxis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_transparencySortAxis", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_transparencySortAxis_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_transparencySortAxis_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_transparencySortAxis_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_transparencySortMode(
        &mut self,
        value: crate::UnityEngine::TransparencySortMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::TransparencySortMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_transparencySortMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_transparencySortMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_useJitteredProjectionMatrixForTransparentRendering(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_useJitteredProjectionMatrixForTransparentRendering")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_useJitteredProjectionMatrixForTransparentRendering",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_useOcclusionCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_useOcclusionCulling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_useOcclusionCulling", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_usePhysicalProperties(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_usePhysicalProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_usePhysicalProperties", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_worldToCameraMatrix(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Matrix4x4),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_worldToCameraMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_worldToCameraMatrix", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_worldToCameraMatrix_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_worldToCameraMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera as quest_hook::libil2cpp::Type >
                    ::class(), "set_worldToCameraMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Camera_CameraCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/CameraCallback";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera_CameraCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera_CameraCallback as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cam))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera_CameraCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera_CameraCallback as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Camera_GateFitMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/GateFitMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitMode")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Camera_GateFitMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitMode")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Camera_GateFitMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitMode")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Camera_GateFitMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitMode")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Camera_GateFitMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Camera_GateFitParameters {
    pub _mode_k__BackingField: crate::UnityEngine::Camera_GateFitMode,
    pub _aspect_k__BackingField: f32,
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Camera_GateFitParameters {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/GateFitParameters";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_GateFitParameters {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_GateFitParameters {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_GateFitParameters {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+GateFitParameters")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Camera_GateFitParameters {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera_GateFitParameters as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_aspect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera_GateFitParameters as
                    quest_hook::libil2cpp::Type > ::class(), "get_aspect", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Camera_GateFitMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Camera_GateFitParameters as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Camera_GateFitMode,
                0usize,
            >("get_mode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Camera_GateFitParameters as
                    quest_hook::libil2cpp::Type > ::class(), "get_mode", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Camera_GateFitMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Camera_MonoOrStereoscopicEye {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/MonoOrStereoscopicEye";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+MonoOrStereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_MonoOrStereoscopicEye {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+MonoOrStereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_MonoOrStereoscopicEye {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+MonoOrStereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_MonoOrStereoscopicEye {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+MonoOrStereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Camera_MonoOrStereoscopicEye {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Camera_ProjectionMatrixMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/ProjectionMatrixMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+ProjectionMatrixMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_ProjectionMatrixMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+ProjectionMatrixMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_ProjectionMatrixMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+ProjectionMatrixMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_ProjectionMatrixMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+ProjectionMatrixMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Camera_ProjectionMatrixMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Camera_RenderRequest {
    pub m_CameraRenderMode: crate::UnityEngine::Camera_RenderRequestMode,
    pub m_ResultRT: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub m_OutputSpace: crate::UnityEngine::Camera_RenderRequestOutputSpace,
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Camera_RenderRequest {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/RenderRequest";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_RenderRequest {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_RenderRequest {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_RenderRequest {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequest")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Camera_RenderRequest {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Camera_RenderRequestMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/RenderRequestMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_RenderRequestMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_RenderRequestMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_RenderRequestMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Camera_RenderRequestMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Camera_RenderRequestOutputSpace {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/RenderRequestOutputSpace";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestOutputSpace")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_RenderRequestOutputSpace {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestOutputSpace")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_RenderRequestOutputSpace {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestOutputSpace")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_RenderRequestOutputSpace {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+RenderRequestOutputSpace")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Camera_RenderRequestOutputSpace {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_SceneViewFilterMode {
    #[default]
    Off = 0i32,
    ShowFiltered = 1i32,
}
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Camera_SceneViewFilterMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/SceneViewFilterMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_SceneViewFilterMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_SceneViewFilterMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_SceneViewFilterMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+SceneViewFilterMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Camera_SceneViewFilterMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Camera_StereoscopicEye {
    #[default]
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Camera_StereoscopicEye {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Camera/StereoscopicEye";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Camera_StereoscopicEye {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Camera_StereoscopicEye {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Camera_StereoscopicEye {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Camera+StereoscopicEye")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Camera_StereoscopicEye {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
