#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::LIV::SDK::Unity::SDKUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LIV.SDK.Unity";
    const CLASS_NAME: &'static str = "SDKUtils";
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
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl std::ops::Deref for crate::LIV::SDK::Unity::SDKUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl crate::LIV::SDK::Unity::SDKUtils {
    pub fn ApplyUserSpaceTransform(
        render: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ApplyUserSpaceTransform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ApplyUserSpaceTransform", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (render))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CleanCameraBehaviours(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        excludeBehaviours: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CleanCameraBehaviours")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CleanCameraBehaviours", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (camera, excludeBehaviours))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsFlag(flags: u64, flag: u64) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), bool, 2usize>("ContainsFlag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ContainsFlag", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (flags, flag)) };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBridgeOutputFrame(
        render: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CreateBridgeOutputFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateBridgeOutputFrame", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (render))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateClipPlane(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        resX: i32,
        resY: i32,
        useQuads: bool,
        skirtLength: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    i32,
                    i32,
                    bool,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("CreateClipPlane")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateClipPlane", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mesh, resX, resY, useQuads, skirtLength))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTexture(
        renderTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    >,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::RenderTextureFormat,
                ),
                bool,
                5usize,
            >("CreateTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateTexture", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (renderTexture, width, height, depth, format))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyObject<T>(
        reference: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>),
                bool,
                1usize,
            >("DestroyObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DestroyObject", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (reference)) };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyTexture(
        _renderTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DestroyTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DestroyTexture", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_renderTexture))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableStandardAssets(
        cameraInstance: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        behaviours: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                >,
            >,
        >,
        wasBehaviourEnabled: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<bool>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DisableStandardAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DisableStandardAssets", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked((), (cameraInstance, behaviours, wasBehaviourEnabled))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisposeObject<T>(
        reference: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<T>),
                bool,
                1usize,
            >("DisposeObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DisposeObject", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (reference)) };
        Ok(__cordl_ret.into())
    }
    pub fn FeatureEnabled(
        features: crate::LIV::SDK::Unity::FEATURES,
        feature: crate::LIV::SDK::Unity::FEATURES,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::LIV::SDK::Unity::FEATURES, crate::LIV::SDK::Unity::FEATURES),
                bool,
                2usize,
            >("FeatureEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FeatureEnabled", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (features, feature))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ForceForwardRendering(
        cameraInstance: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        clipPlaneMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        forceForwardRenderingMaterial: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Material,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ForceForwardRendering")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ForceForwardRendering", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (cameraInstance, clipPlaneMesh, forceForwardRenderingMaterial),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraPositionAndRotation(
        pose: crate::LIV::SDK::Unity::SDKPose,
        originLocalToWorldMatrix: crate::UnityEngine::Matrix4x4,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::LIV::SDK::Unity::SDKPose,
                    crate::UnityEngine::Matrix4x4,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("GetCameraPositionAndRotation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCameraPositionAndRotation", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (pose, originLocalToWorldMatrix, position, rotation),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetColorSpace(
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
                1usize,
            >("GetColorSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetColorSpace", 1usize
                )
            });
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE = unsafe {
            method.invoke_unchecked((), (renderTexture))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::TEXTURE_DEVICE,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::LIV::SDK::Unity::TEXTURE_DEVICE,
                0usize,
            >("GetDevice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDevice", 0usize
                )
            });
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_DEVICE = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetReadWriteFromColorSpace(
        colorSpace: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureReadWrite> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE),
                crate::UnityEngine::RenderTextureReadWrite,
                1usize,
            >("GetReadWriteFromColorSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetReadWriteFromColorSpace", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureReadWrite = unsafe {
            method.invoke_unchecked((), (colorSpace))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderingPipeline(
        renderingPath: crate::UnityEngine::RenderingPath,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::RENDERING_PIPELINE> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::RenderingPath),
                crate::LIV::SDK::Unity::RENDERING_PIPELINE,
                1usize,
            >("GetRenderingPipeline")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRenderingPipeline", 1usize
                )
            });
        let __cordl_ret: crate::LIV::SDK::Unity::RENDERING_PIPELINE = unsafe {
            method.invoke_unchecked((), (renderingPath))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedSpace(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKTrackedSpace> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                crate::LIV::SDK::Unity::SDKTrackedSpace,
                1usize,
            >("GetTrackedSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTrackedSpace", 1usize
                )
            });
        let __cordl_ret: crate::LIV::SDK::Unity::SDKTrackedSpace = unsafe {
            method.invoke_unchecked((), (transform))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestoreStandardAssets(
        behaviours: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                >,
            >,
        >,
        wasBehaviourEnabled: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<bool>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RestoreStandardAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RestoreStandardAssets", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (behaviours, wasBehaviourEnabled))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateQuaternionByMatrix(
        matrix: crate::UnityEngine::Matrix4x4,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Quaternion),
                crate::UnityEngine::Quaternion,
                2usize,
            >("RotateQuaternionByMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RotateQuaternionByMatrix", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            method.invoke_unchecked((), (matrix, rotation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCamera(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cameraTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        inputFrame: crate::LIV::SDK::Unity::SDKInputFrame,
        originLocalToWorldMatrix: crate::UnityEngine::Matrix4x4,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    crate::LIV::SDK::Unity::SDKInputFrame,
                    crate::UnityEngine::Matrix4x4,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetCamera")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetCamera", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        camera,
                        cameraTransform,
                        inputFrame,
                        originLocalToWorldMatrix,
                        layerMask,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFlag(
        flags: u64,
        flag: u64,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64, bool), u64, 3usize>("SetFlag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetFlag", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (flags, flag, enabled))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetDefaultColorSpace() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
                0usize,
            >("get_GetDefaultColorSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_GetDefaultColorSpace", 0usize
                )
            });
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
