#[cfg(feature = "cordl_class_UnityEngine+Rendering+LensFlareCommonSRP")]
#[repr(C)]
#[derive(Debug)]
pub struct LensFlareCommonSRP {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LensFlareCommonSRP")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::LensFlareCommonSRP {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LensFlareCommonSRP";
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
#[cfg(feature = "UnityEngine+Rendering+LensFlareCommonSRP")]
impl std::ops::Deref for crate::UnityEngine::Rendering::LensFlareCommonSRP {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LensFlareCommonSRP")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::LensFlareCommonSRP {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LensFlareCommonSRP")]
impl crate::UnityEngine::Rendering::LensFlareCommonSRP {
    #[cfg(feature = "UnityEngine+Rendering+LensFlareCommonSRP+LensFlareCompInfo")]
    pub type LensFlareCompInfo = crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo;
    pub fn AddData(
        &mut self,
        newData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareComponentSRP,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::LensFlareComponentSRP,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "AddData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (newData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalcCropExtents(
        actualWidth: f32,
        actualHeight: f32,
        fieldOfView: f32,
        d: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32, f32),
                        crate::UnityEngine::Vector2,
                        4usize,
                    >("CalcCropExtents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalcCropExtents", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (actualWidth, actualHeight, fieldOfView, d))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalcViewExtents(
        actualWidth: f32,
        actualHeight: f32,
        fieldOfView: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, f32),
                        crate::UnityEngine::Vector2,
                        3usize,
                    >("CalcViewExtents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalcViewExtents", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (actualWidth, actualHeight, fieldOfView))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOcclusion_CommandBuffer3(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        17usize,
                    >("ComputeOcclusion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeOcclusion", 17usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOcclusion_CommandBuffer_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_2(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        _FlareOcclusionTex: i32,
        _FlareCloudOpacity: i32,
        _FlareOcclusionIndex: i32,
        _FlareTex: i32,
        _FlareColorValue: i32,
        _FlareSunOcclusionTex: i32,
        _FlareData0: i32,
        _FlareData1: i32,
        _FlareData2: i32,
        _FlareData3: i32,
        _FlareData4: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        28usize,
                    >("ComputeOcclusion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeOcclusion", 28usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                        _FlareOcclusionTex,
                        _FlareCloudOpacity,
                        _FlareOcclusionIndex,
                        _FlareTex,
                        _FlareColorValue,
                        _FlareSunOcclusionTex,
                        _FlareData0,
                        _FlareData1,
                        _FlareData2,
                        _FlareData3,
                        _FlareData4,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOcclusion_UnsafeCommandBuffer1(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::UnsafeCommandBuffer,
        >,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::UnsafeCommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        17usize,
                    >("ComputeOcclusion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeOcclusion", 17usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeOcclusion_UnsafeCommandBuffer_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_0(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::UnsafeCommandBuffer,
        >,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        _FlareOcclusionTex: i32,
        _FlareCloudOpacity: i32,
        _FlareOcclusionIndex: i32,
        _FlareTex: i32,
        _FlareColorValue: i32,
        _FlareSunOcclusionTex: i32,
        _FlareData0: i32,
        _FlareData1: i32,
        _FlareData2: i32,
        _FlareData3: i32,
        _FlareData4: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::UnsafeCommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        28usize,
                    >("ComputeOcclusion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeOcclusion", 28usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                        _FlareOcclusionTex,
                        _FlareCloudOpacity,
                        _FlareOcclusionIndex,
                        _FlareTex,
                        _FlareColorValue,
                        _FlareSunOcclusionTex,
                        _FlareData0,
                        _FlareData1,
                        _FlareData2,
                        _FlareData3,
                        _FlareData4,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareDataDrivenCommon_CommandBuffer__cordl_bool3(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        viewport: crate::UnityEngine::Rect,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        GetLensFlareLightAttenuation: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                crate::UnityEngine::Vector3,
                f32,
            >,
        >,
        debugView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_4<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                                    crate::UnityEngine::Vector3,
                                    f32,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        21usize,
                    >("DoLensFlareDataDrivenCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoLensFlareDataDrivenCommon", 21usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        viewport,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                        colorBuffer,
                        GetLensFlareLightAttenuation,
                        debugView,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareDataDrivenCommon_CommandBuffer_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32__cordl_bool2(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        viewport: crate::UnityEngine::Rect,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        GetLensFlareLightAttenuation: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                crate::UnityEngine::Vector3,
                f32,
            >,
        >,
        _FlareOcclusionRemapTex: i32,
        _FlareOcclusionTex: i32,
        _FlareOcclusionIndex: i32,
        _FlareCloudOpacity: i32,
        _FlareSunOcclusionTex: i32,
        _FlareTex: i32,
        _FlareColorValue: i32,
        _FlareData0: i32,
        _FlareData1: i32,
        _FlareData2: i32,
        _FlareData3: i32,
        _FlareData4: i32,
        debugView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_4<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                                    crate::UnityEngine::Vector3,
                                    f32,
                                >,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        33usize,
                    >("DoLensFlareDataDrivenCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoLensFlareDataDrivenCommon", 33usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        viewport,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                        colorBuffer,
                        GetLensFlareLightAttenuation,
                        _FlareOcclusionRemapTex,
                        _FlareOcclusionTex,
                        _FlareOcclusionIndex,
                        _FlareCloudOpacity,
                        _FlareSunOcclusionTex,
                        _FlareTex,
                        _FlareColorValue,
                        _FlareData0,
                        _FlareData1,
                        _FlareData2,
                        _FlareData3,
                        _FlareData4,
                        debugView,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareDataDrivenCommon_UnsafeCommandBuffer__cordl_bool1(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        viewport: crate::UnityEngine::Rect,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::UnsafeCommandBuffer,
        >,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        GetLensFlareLightAttenuation: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                crate::UnityEngine::Vector3,
                f32,
            >,
        >,
        debugView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::UnsafeCommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_4<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                                    crate::UnityEngine::Vector3,
                                    f32,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        21usize,
                    >("DoLensFlareDataDrivenCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoLensFlareDataDrivenCommon", 21usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        viewport,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                        colorBuffer,
                        GetLensFlareLightAttenuation,
                        debugView,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareDataDrivenCommon_UnsafeCommandBuffer_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32__cordl_bool0(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        viewport: crate::UnityEngine::Rect,
        xr: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::XRPass,
        >,
        xrIndex: i32,
        actualWidth: f32,
        actualHeight: f32,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
        isCameraRelative: bool,
        cameraPositionWS: crate::UnityEngine::Vector3,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::UnsafeCommandBuffer,
        >,
        taaEnabled: bool,
        hasCloudLayer: bool,
        cloudOpacityTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        colorBuffer: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        GetLensFlareLightAttenuation: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                crate::UnityEngine::Vector3,
                f32,
            >,
        >,
        _FlareOcclusionRemapTex: i32,
        _FlareOcclusionTex: i32,
        _FlareOcclusionIndex: i32,
        _FlareCloudOpacity: i32,
        _FlareSunOcclusionTex: i32,
        _FlareTex: i32,
        _FlareColorValue: i32,
        _FlareData0: i32,
        _FlareData1: i32,
        _FlareData2: i32,
        _FlareData3: i32,
        _FlareData4: i32,
        debugView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Experimental::Rendering::XRPass,
                            >,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::UnsafeCommandBuffer,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_4<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                                    crate::UnityEngine::Vector3,
                                    f32,
                                >,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        33usize,
                    >("DoLensFlareDataDrivenCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoLensFlareDataDrivenCommon", 33usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        viewport,
                        xr,
                        xrIndex,
                        actualWidth,
                        actualHeight,
                        usePanini,
                        paniniDistance,
                        paniniCropToFit,
                        isCameraRelative,
                        cameraPositionWS,
                        viewProjMatrix,
                        cmd,
                        taaEnabled,
                        hasCloudLayer,
                        cloudOpacityTexture,
                        sunOcclusionTexture,
                        colorBuffer,
                        GetLensFlareLightAttenuation,
                        _FlareOcclusionRemapTex,
                        _FlareOcclusionTex,
                        _FlareOcclusionIndex,
                        _FlareCloudOpacity,
                        _FlareSunOcclusionTex,
                        _FlareTex,
                        _FlareColorValue,
                        _FlareData0,
                        _FlareData1,
                        _FlareData2,
                        _FlareData3,
                        _FlareData4,
                        debugView,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareScreenSpaceCommon_CommandBuffer__cordl_bool2(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        actualWidth: f32,
        actualHeight: f32,
        tintColor: crate::UnityEngine::Color,
        originalBloomTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        bloomMipTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        spectralLut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        streakTextureTmp: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        streakTextureTmp2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        parameters1: crate::UnityEngine::Vector4,
        parameters2: crate::UnityEngine::Vector4,
        parameters3: crate::UnityEngine::Vector4,
        parameters4: crate::UnityEngine::Vector4,
        parameters5: crate::UnityEngine::Vector4,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        result: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        debugView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            f32,
                            f32,
                            crate::UnityEngine::Color,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        18usize,
                    >("DoLensFlareScreenSpaceCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoLensFlareScreenSpaceCommon", 18usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        actualWidth,
                        actualHeight,
                        tintColor,
                        originalBloomTexture,
                        bloomMipTexture,
                        spectralLut,
                        streakTextureTmp,
                        streakTextureTmp2,
                        parameters1,
                        parameters2,
                        parameters3,
                        parameters4,
                        parameters5,
                        cmd,
                        result,
                        debugView,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareScreenSpaceCommon_CommandBuffer_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32__cordl_bool1(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        actualWidth: f32,
        actualHeight: f32,
        tintColor: crate::UnityEngine::Color,
        originalBloomTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        bloomMipTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        spectralLut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        streakTextureTmp: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        streakTextureTmp2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        parameters1: crate::UnityEngine::Vector4,
        parameters2: crate::UnityEngine::Vector4,
        parameters3: crate::UnityEngine::Vector4,
        parameters4: crate::UnityEngine::Vector4,
        parameters5: crate::UnityEngine::Vector4,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        result: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        _LensFlareScreenSpaceBloomMipTexture: i32,
        _LensFlareScreenSpaceResultTexture: i32,
        _LensFlareScreenSpaceSpectralLut: i32,
        _LensFlareScreenSpaceStreakTex: i32,
        _LensFlareScreenSpaceMipLevel: i32,
        _LensFlareScreenSpaceTintColor: i32,
        _LensFlareScreenSpaceParams1: i32,
        _LensFlareScreenSpaceParams2: i32,
        _LensFlareScreenSpaceParams3: i32,
        _LensFlareScreenSpaceParams4: i32,
        _LensFlareScreenSpaceParams5: i32,
        debugView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            f32,
                            f32,
                            crate::UnityEngine::Color,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        29usize,
                    >("DoLensFlareScreenSpaceCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoLensFlareScreenSpaceCommon", 29usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        actualWidth,
                        actualHeight,
                        tintColor,
                        originalBloomTexture,
                        bloomMipTexture,
                        spectralLut,
                        streakTextureTmp,
                        streakTextureTmp2,
                        parameters1,
                        parameters2,
                        parameters3,
                        parameters4,
                        parameters5,
                        cmd,
                        result,
                        _LensFlareScreenSpaceBloomMipTexture,
                        _LensFlareScreenSpaceResultTexture,
                        _LensFlareScreenSpaceSpectralLut,
                        _LensFlareScreenSpaceStreakTex,
                        _LensFlareScreenSpaceMipLevel,
                        _LensFlareScreenSpaceTintColor,
                        _LensFlareScreenSpaceParams1,
                        _LensFlareScreenSpaceParams2,
                        _LensFlareScreenSpaceParams3,
                        _LensFlareScreenSpaceParams4,
                        _LensFlareScreenSpaceParams5,
                        debugView,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareScreenSpaceCommon_UnsafeCommandBuffer__cordl_bool0(
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        actualWidth: f32,
        actualHeight: f32,
        tintColor: crate::UnityEngine::Color,
        originalBloomTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        bloomMipTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        spectralLut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        streakTextureTmp: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        streakTextureTmp2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        parameters1: crate::UnityEngine::Vector4,
        parameters2: crate::UnityEngine::Vector4,
        parameters3: crate::UnityEngine::Vector4,
        parameters4: crate::UnityEngine::Vector4,
        parameters5: crate::UnityEngine::Vector4,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::UnsafeCommandBuffer,
        >,
        result: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        debugView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            f32,
                            f32,
                            crate::UnityEngine::Color,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            crate::UnityEngine::Vector4,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::UnsafeCommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        18usize,
                    >("DoLensFlareScreenSpaceCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoLensFlareScreenSpaceCommon", 18usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        lensFlareShader,
                        cam,
                        actualWidth,
                        actualHeight,
                        tintColor,
                        originalBloomTexture,
                        bloomMipTexture,
                        spectralLut,
                        streakTextureTmp,
                        streakTextureTmp2,
                        parameters1,
                        parameters2,
                        parameters3,
                        parameters4,
                        parameters5,
                        cmd,
                        result,
                        debugView,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoPaniniProjection(
        screenPos: crate::UnityEngine::Vector2,
        actualWidth: f32,
        actualHeight: f32,
        fieldOfView: f32,
        paniniProjectionCropToFit: f32,
        paniniProjectionDistance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2, f32, f32, f32, f32, f32),
                        crate::UnityEngine::Vector2,
                        6usize,
                    >("DoPaniniProjection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoPaniniProjection", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        screenPos,
                        actualWidth,
                        actualHeight,
                        fieldOfView,
                        paniniProjectionCropToFit,
                        paniniProjectionDistance,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ForceSingleElement(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareDataElementSRP,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::LensFlareDataElementSRP,
                        >),
                        bool,
                        1usize,
                    >("ForceSingleElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ForceSingleElement", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFlareData0(
        screenPos: crate::UnityEngine::Vector2,
        translationScale: crate::UnityEngine::Vector2,
        rayOff0: crate::UnityEngine::Vector2,
        vLocalScreenRatio: crate::UnityEngine::Vector2,
        angleDeg: f32,
        position: f32,
        angularOffset: f32,
        positionOffset: crate::UnityEngine::Vector2,
        autoRotate: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            f32,
                            f32,
                            f32,
                            crate::UnityEngine::Vector2,
                            bool,
                        ),
                        crate::UnityEngine::Vector4,
                        9usize,
                    >("GetFlareData0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFlareData0", 9usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        screenPos,
                        translationScale,
                        rayOff0,
                        vLocalScreenRatio,
                        angleDeg,
                        position,
                        angularOffset,
                        positionOffset,
                        autoRotate,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLensFlareRayOffset(
        screenPos: crate::UnityEngine::Vector2,
        position: f32,
        globalCos0: f32,
        globalSin0: f32,
        vAspectRatio: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            f32,
                            f32,
                            f32,
                            crate::UnityEngine::Vector2,
                        ),
                        crate::UnityEngine::Vector2,
                        5usize,
                    >("GetLensFlareRayOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLensFlareRayOffset", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (screenPos, position, globalCos0, globalSin0, vAspectRatio),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextAvailableIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetNextAvailableIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNextAvailableIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOcclusionRTFormat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        0usize,
                    >("GetOcclusionRTFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOcclusionRTFormat", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCloudLayerOpacityNeeded(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        bool,
                        1usize,
                    >("IsCloudLayerOpacityNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsCloudLayerOpacityNeeded", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsEmpty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsLensFlareSRPHidden(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        comp: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareComponentSRP,
        >,
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::LensFlareDataSRP>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::LensFlareComponentSRP,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::LensFlareDataSRP,
                            >,
                        ),
                        bool,
                        3usize,
                    >("IsLensFlareSRPHidden")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsLensFlareSRPHidden", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (cam, comp, data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsOcclusionRTCompatible() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsOcclusionRTCompatible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsOcclusionRTCompatible", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Panini_Generic_Inv(
        projPos: crate::UnityEngine::Vector2,
        d: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2, f32),
                        crate::UnityEngine::Vector2,
                        2usize,
                    >("Panini_Generic_Inv")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Panini_Generic_Inv", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked((), (projPos, d))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessLensFlareSRPElements(
        elements: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::LensFlareDataElementSRP,
                    >,
                >,
            >,
        >,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        globalColorModulation: crate::UnityEngine::Color,
        light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        compIntensity: f32,
        scale: f32,
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        screenPos: crate::UnityEngine::Vector2,
        compAllowOffScreen: bool,
        vScreenRatio: crate::UnityEngine::Vector2,
        flareData1: crate::UnityEngine::Vector4,
        preview: bool,
        depth: i32,
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
                                        quest_hook::libil2cpp::Gc<
                                            crate::UnityEngine::Rendering::LensFlareDataElementSRP,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Color,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            f32,
                            f32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::Vector2,
                            bool,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector4,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        13usize,
                    >("ProcessLensFlareSRPElements")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessLensFlareSRPElements", 13usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        elements,
                        cmd,
                        globalColorModulation,
                        light,
                        compIntensity,
                        scale,
                        lensFlareShader,
                        screenPos,
                        compAllowOffScreen,
                        vScreenRatio,
                        flareData1,
                        preview,
                        depth,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessLensFlareSRPElementsSingle(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareDataElementSRP,
        >,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        globalColorModulation: crate::UnityEngine::Color,
        light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        compIntensity: f32,
        scale: f32,
        lensFlareShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        screenPos: crate::UnityEngine::Vector2,
        compAllowOffScreen: bool,
        vScreenRatio: crate::UnityEngine::Vector2,
        flareData1: crate::UnityEngine::Vector4,
        preview: bool,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::LensFlareDataElementSRP,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Color,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                            f32,
                            f32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::Vector2,
                            bool,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector4,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        13usize,
                    >("ProcessLensFlareSRPElementsSingle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessLensFlareSRPElementsSingle", 13usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        element,
                        cmd,
                        globalColorModulation,
                        light,
                        compIntensity,
                        scale,
                        lensFlareShader,
                        screenPos,
                        compAllowOffScreen,
                        vScreenRatio,
                        flareData1,
                        preview,
                        depth,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareComponentSRP,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::LensFlareComponentSRP,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOcclusionPermutation(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        useFogOpacityOcclusion: bool,
        _FlareSunOcclusionTex: i32,
        sunOcclusionTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            bool,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetOcclusionPermutation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOcclusionPermutation", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cmd,
                        useFogOpacityOcclusion,
                        _FlareSunOcclusionTex,
                        sunOcclusionTexture,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuateForwardLight(
        forward: crate::UnityEngine::Vector3,
        wo: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        f32,
                        2usize,
                    >("ShapeAttenuateForwardLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuateForwardLight", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, wo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationAreaDiscLight(
        forward: crate::UnityEngine::Vector3,
        wo: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        f32,
                        2usize,
                    >("ShapeAttenuationAreaDiscLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationAreaDiscLight", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, wo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationAreaRectangleLight(
        forward: crate::UnityEngine::Vector3,
        wo: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        f32,
                        2usize,
                    >("ShapeAttenuationAreaRectangleLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationAreaRectangleLight", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, wo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationAreaTubeLight(
        lightPositionWS: crate::UnityEngine::Vector3,
        lightSide: crate::UnityEngine::Vector3,
        lightWidth: f32,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        f32,
                        4usize,
                    >("ShapeAttenuationAreaTubeLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationAreaTubeLight", 4usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (lightPositionWS, lightSide, lightWidth, cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationDirLight(
        forward: crate::UnityEngine::Vector3,
        wo: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        f32,
                        2usize,
                    >("ShapeAttenuationDirLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationDirLight", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, wo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationPointLight() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("ShapeAttenuationPointLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationPointLight", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationSpotBoxLight(
        forward: crate::UnityEngine::Vector3,
        wo: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        f32,
                        2usize,
                    >("ShapeAttenuationSpotBoxLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationSpotBoxLight", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, wo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationSpotConeLight(
        forward: crate::UnityEngine::Vector3,
        wo: crate::UnityEngine::Vector3,
        spotAngle: f32,
        innerSpotPercent01: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            f32,
                        ),
                        f32,
                        4usize,
                    >("ShapeAttenuationSpotConeLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationSpotConeLight", 4usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (forward, wo, spotAngle, innerSpotPercent01))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShapeAttenuationSpotPyramidLight(
        forward: crate::UnityEngine::Vector3,
        wo: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        f32,
                        2usize,
                    >("ShapeAttenuationSpotPyramidLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShapeAttenuationSpotPyramidLight", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (forward, wo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewport(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        isLocalLight: bool,
        isCameraRelative: bool,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        positionWS: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            bool,
                            bool,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Vector3,
                        ),
                        crate::UnityEngine::Vector3,
                        5usize,
                    >("WorldToViewport")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WorldToViewport", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (camera, isLocalLight, isCameraRelative, viewProjMatrix, positionWS),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportDistance(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        positionWS: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Vector3,
                        ),
                        crate::UnityEngine::Vector3,
                        2usize,
                    >("WorldToViewportDistance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WorldToViewportDistance", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (cam, positionWS))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToViewportLocal(
        isCameraRelative: bool,
        viewProjMatrix: crate::UnityEngine::Matrix4x4,
        cameraPosWS: crate::UnityEngine::Vector3,
        positionWS: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            bool,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                        ),
                        crate::UnityEngine::Vector3,
                        4usize,
                    >("WorldToViewportLocal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WorldToViewportLocal", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (isCameraRelative, viewProjMatrix, cameraPosWS, positionWS),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ProcessLensFlareSRPElementsSingle_g__ComputeLocalSize_74_0(
        rayOff: crate::UnityEngine::Vector2,
        rayOff0: crate::UnityEngine::Vector2,
        curSize: crate::UnityEngine::Vector2,
        distortionCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        crate::UnityEngine::Vector2,
                        5usize,
                    >("<ProcessLensFlareSRPElementsSingle>g__ComputeLocalSize|74_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<ProcessLensFlareSRPElementsSingle>g__ComputeLocalSize|74_0",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        rayOff,
                        rayOff0,
                        curSize,
                        distortionCurve,
                        _cordl_fixed_empty_name_whitespace,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ProcessLensFlareSRPElementsSingle_g__RandomRange_74_1(
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32),
                        f32,
                        2usize,
                    >("<ProcessLensFlareSRPElementsSingle>g__RandomRange|74_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<ProcessLensFlareSRPElementsSingle>g__RandomRange|74_1",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ShapeAttenuationAreaTubeLight_g__DiffLineIntegral_57_2(
        p1: crate::UnityEngine::Vector3,
        p2: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        f32,
                        2usize,
                    >("<ShapeAttenuationAreaTubeLight>g__DiffLineIntegral|57_2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<ShapeAttenuationAreaTubeLight>g__DiffLineIntegral|57_2",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (p1, p2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ShapeAttenuationAreaTubeLight_g__Fpo_57_0(
        d: f32,
        l: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32),
                        f32,
                        2usize,
                    >("<ShapeAttenuationAreaTubeLight>g__Fpo|57_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<ShapeAttenuationAreaTubeLight>g__Fpo|57_0", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (d, l))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ShapeAttenuationAreaTubeLight_g__Fwt_57_1(
        d: f32,
        l: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32),
                        f32,
                        2usize,
                    >("<ShapeAttenuationAreaTubeLight>g__Fwt|57_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<ShapeAttenuationAreaTubeLight>g__Fwt|57_1", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (d, l))?
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
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_Data")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Data", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::LensFlareCommonSRP>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::LensFlareCommonSRP,
                        >,
                        0usize,
                    >("get_Instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Instance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareCommonSRP,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+LensFlareCommonSRP")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::LensFlareCommonSRP {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LensFlareCommonSRP+LensFlareCompInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LensFlareCommonSRP_LensFlareCompInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub index: i32,
    pub comp: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::LensFlareComponentSRP,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LensFlareCommonSRP+LensFlareCompInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "LensFlareCommonSRP/LensFlareCompInfo";
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
#[cfg(feature = "UnityEngine+Rendering+LensFlareCommonSRP+LensFlareCompInfo")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LensFlareCommonSRP+LensFlareCompInfo")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LensFlareCommonSRP+LensFlareCompInfo")]
impl crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo {
    pub fn New(
        idx: i32,
        cmp: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareComponentSRP,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (idx, cmp))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        idx: i32,
        cmp: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::LensFlareComponentSRP,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::LensFlareComponentSRP,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (idx, cmp))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+LensFlareCommonSRP+LensFlareCompInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::LensFlareCommonSRP_LensFlareCompInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
