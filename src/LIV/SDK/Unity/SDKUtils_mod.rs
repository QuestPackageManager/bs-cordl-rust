#[cfg(feature = "cordl_class_LIV+SDK+Unity+SDKUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_LIV+SDK+Unity+SDKUtils")]
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl crate::LIV::SDK::Unity::SDKUtils {
    pub fn ApplyUserSpaceTransform(
        render: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ApplyUserSpaceTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyUserSpaceTransform", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (render))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CleanCameraBehaviours", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (camera, excludeBehaviours))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsFlag(flags: u64, flag: u64) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64), bool, 2usize>("ContainsFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsFlag", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (flags, flag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBridgeOutputFrame(
        render: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CreateBridgeOutputFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateBridgeOutputFrame", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (render))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateClipPlane", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (mesh, resX, resY, useQuads, skirtLength))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateTexture", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (renderTexture, width, height, depth, format))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        bool,
                        1usize,
                    >("DestroyObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyObject", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (reference))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyTexture(
        _renderTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_renderTexture))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisableStandardAssets", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cameraInstance, behaviours, wasBehaviourEnabled))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        bool,
                        1usize,
                    >("DisposeObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisposeObject", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (reference))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FeatureEnabled(
        features: crate::LIV::SDK::Unity::FEATURES,
        feature: crate::LIV::SDK::Unity::FEATURES,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::LIV::SDK::Unity::FEATURES,
                            crate::LIV::SDK::Unity::FEATURES,
                        ),
                        bool,
                        2usize,
                    >("FeatureEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FeatureEnabled", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (features, feature))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ForceForwardRendering", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cameraInstance, clipPlaneMesh, forceForwardRenderingMaterial),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraPositionAndRotation(
        pose: crate::LIV::SDK::Unity::SDKPose,
        originLocalToWorldMatrix: crate::UnityEngine::Matrix4x4,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::LIV::SDK::Unity::SDKPose,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Quaternion,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetCameraPositionAndRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCameraPositionAndRotation", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (pose, originLocalToWorldMatrix, position, rotation),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetColorSpace(
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                        crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
                        1usize,
                    >("GetColorSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetColorSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE = unsafe {
            cordl_method_info.invoke_unchecked((), (renderTexture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::TEXTURE_DEVICE,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::LIV::SDK::Unity::TEXTURE_DEVICE,
                        0usize,
                    >("GetDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDevice", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_DEVICE = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetReadWriteFromColorSpace(
        colorSpace: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureReadWrite> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE),
                        crate::UnityEngine::RenderTextureReadWrite,
                        1usize,
                    >("GetReadWriteFromColorSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetReadWriteFromColorSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureReadWrite = unsafe {
            cordl_method_info.invoke_unchecked((), (colorSpace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderingPipeline(
        renderingPath: crate::UnityEngine::RenderingPath,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::RENDERING_PIPELINE> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::RenderingPath),
                        crate::LIV::SDK::Unity::RENDERING_PIPELINE,
                        1usize,
                    >("GetRenderingPipeline")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderingPipeline", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::LIV::SDK::Unity::RENDERING_PIPELINE = unsafe {
            cordl_method_info.invoke_unchecked((), (renderingPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedSpace(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKTrackedSpace> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                        crate::LIV::SDK::Unity::SDKTrackedSpace,
                        1usize,
                    >("GetTrackedSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTrackedSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::LIV::SDK::Unity::SDKTrackedSpace = unsafe {
            cordl_method_info.invoke_unchecked((), (transform))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RestoreStandardAssets", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (behaviours, wasBehaviourEnabled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateQuaternionByMatrix(
        matrix: crate::UnityEngine::Matrix4x4,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Quaternion),
                        crate::UnityEngine::Quaternion,
                        2usize,
                    >("RotateQuaternionByMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RotateQuaternionByMatrix", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (matrix, rotation))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetCamera", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        camera,
                        cameraTransform,
                        inputFrame,
                        originLocalToWorldMatrix,
                        layerMask,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFlag(
        flags: u64,
        flag: u64,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64, bool), u64, 3usize>("SetFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "SetFlag",
                            3usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked((), (flags, flag, enabled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_GetDefaultColorSpace() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
                        0usize,
                    >("get_GetDefaultColorSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_GetDefaultColorSpace", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LIV+SDK+Unity+SDKUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
