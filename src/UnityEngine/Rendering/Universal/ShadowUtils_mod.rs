#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ShadowUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ShadowUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ShadowUtils";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ShadowUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ShadowUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowUtils")]
impl crate::UnityEngine::Rendering::Universal::ShadowUtils {
    pub const kMinimumPunctualLightHardShadowResolution: i32 = 8i32;
    pub const kMinimumPunctualLightSoftShadowResolution: i32 = 16i32;
    pub fn AllocShadowRT(
        width: i32,
        height: i32,
        bits: i32,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        6usize,
                    >("AllocShadowRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocShadowRT", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RTHandle,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (width, height, bits, anisoLevel, mipMapBias, name),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplySliceTransform(
        shadowSliceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
        atlasWidth: i32,
        atlasHeight: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowSliceData,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ApplySliceTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplySliceTransform", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (shadowSliceData, atlasWidth, atlasHeight))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractDirectionalLightMatrix_ByRefMut1(
        cullResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowData,
        >,
        shadowLightIndex: i32,
        cascadeIndex: i32,
        shadowmapWidth: i32,
        shadowmapHeight: i32,
        shadowResolution: i32,
        shadowNearPlane: f32,
        cascadeSplitDistance: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector4,
        >,
        shadowSliceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowData,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowSliceData,
                            >,
                        ),
                        bool,
                        10usize,
                    >("ExtractDirectionalLightMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractDirectionalLightMatrix", 10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullResults,
                        shadowData,
                        shadowLightIndex,
                        cascadeIndex,
                        shadowmapWidth,
                        shadowmapHeight,
                        shadowResolution,
                        shadowNearPlane,
                        cascadeSplitDistance,
                        shadowSliceData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractDirectionalLightMatrix_ByRefMut_ByRefMut_ByRefMut0(
        cullResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowData,
        >,
        shadowLightIndex: i32,
        cascadeIndex: i32,
        shadowmapWidth: i32,
        shadowmapHeight: i32,
        shadowResolution: i32,
        shadowNearPlane: f32,
        cascadeSplitDistance: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector4,
        >,
        shadowSliceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowData,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowSliceData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        ),
                        bool,
                        12usize,
                    >("ExtractDirectionalLightMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractDirectionalLightMatrix", 12usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullResults,
                        shadowData,
                        shadowLightIndex,
                        cascadeIndex,
                        shadowmapWidth,
                        shadowmapHeight,
                        shadowResolution,
                        shadowNearPlane,
                        cascadeSplitDistance,
                        shadowSliceData,
                        viewMatrix,
                        projMatrix,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractDirectionalLightMatrix_UniversalShadowData2(
        cullResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
        shadowLightIndex: i32,
        cascadeIndex: i32,
        shadowmapWidth: i32,
        shadowmapHeight: i32,
        shadowResolution: i32,
        shadowNearPlane: f32,
        cascadeSplitDistance: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector4,
        >,
        shadowSliceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowSliceData,
                            >,
                        ),
                        bool,
                        10usize,
                    >("ExtractDirectionalLightMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractDirectionalLightMatrix", 10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullResults,
                        shadowData,
                        shadowLightIndex,
                        cascadeIndex,
                        shadowmapWidth,
                        shadowmapHeight,
                        shadowResolution,
                        shadowNearPlane,
                        cascadeSplitDistance,
                        shadowSliceData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPointLightMatrix_ByRefMut0(
        cullResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowData,
        >,
        shadowLightIndex: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        fovBias: f32,
        shadowMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        splitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowData,
                            >,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ShadowSplitData,
                            >,
                        ),
                        bool,
                        9usize,
                    >("ExtractPointLightMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractPointLightMatrix", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullResults,
                        shadowData,
                        shadowLightIndex,
                        cubemapFace,
                        fovBias,
                        shadowMatrix,
                        viewMatrix,
                        projMatrix,
                        splitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPointLightMatrix_UniversalShadowData1(
        cullResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
        shadowLightIndex: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        fovBias: f32,
        shadowMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        splitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ShadowSplitData,
                            >,
                        ),
                        bool,
                        9usize,
                    >("ExtractPointLightMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractPointLightMatrix", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullResults,
                        shadowData,
                        shadowLightIndex,
                        cubemapFace,
                        fovBias,
                        shadowMatrix,
                        viewMatrix,
                        projMatrix,
                        splitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractSpotLightMatrix_ByRefMut0(
        cullResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowData,
        >,
        shadowLightIndex: i32,
        shadowMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        splitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowData,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ShadowSplitData,
                            >,
                        ),
                        bool,
                        7usize,
                    >("ExtractSpotLightMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractSpotLightMatrix", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullResults,
                        shadowData,
                        shadowLightIndex,
                        shadowMatrix,
                        viewMatrix,
                        projMatrix,
                        splitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractSpotLightMatrix_UniversalShadowData1(
        cullResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
        shadowLightIndex: i32,
        shadowMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        splitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ShadowSplitData,
                            >,
                        ),
                        bool,
                        7usize,
                    >("ExtractSpotLightMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractSpotLightMatrix", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullResults,
                        shadowData,
                        shadowLightIndex,
                        shadowMatrix,
                        viewMatrix,
                        projMatrix,
                        splitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FastApproximately_Vector4_Vector4_1(
        a: crate::UnityEngine::Vector4,
        b: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector4, crate::UnityEngine::Vector4),
                        bool,
                        2usize,
                    >("FastApproximately")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FastApproximately", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FastApproximately_f32_f32_0(
        a: f32,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), bool, 2usize>("FastApproximately")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FastApproximately", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxTileResolutionInAtlas(
        atlasWidth: i32,
        atlasHeight: i32,
        tileCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32),
                        i32,
                        3usize,
                    >("GetMaxTileResolutionInAtlas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMaxTileResolutionInAtlas", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (atlasWidth, atlasHeight, tileCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPunctualLightShadowSlicesCount(
        lightType: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::LightType>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::LightType>),
                        i32,
                        1usize,
                    >("GetPunctualLightShadowSlicesCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPunctualLightShadowSlicesCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (lightType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetScaleAndBiasForLinearDistanceFade(
        fadeDistance: f32,
        border: f32,
        scale: quest_hook::libil2cpp::ByRefMut<f32>,
        bias: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            f32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetScaleAndBiasForLinearDistanceFade")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetScaleAndBiasForLinearDistanceFade", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (fadeDistance, border, scale, bias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowBias_ByRefMut_Matrix4x4_f32_0(
        shadowLight: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::VisibleLight,
        >,
        shadowLightIndex: i32,
        shadowData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowData,
        >,
        lightProjectionMatrix: crate::UnityEngine::Matrix4x4,
        shadowResolution: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::VisibleLight,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowData,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            f32,
                        ),
                        crate::UnityEngine::Vector4,
                        5usize,
                    >("GetShadowBias")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetShadowBias", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        shadowLight,
                        shadowLightIndex,
                        shadowData,
                        lightProjectionMatrix,
                        shadowResolution,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowBias_List_1__cordl_bool_Matrix4x4_f32_2(
        shadowLight: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::VisibleLight,
        >,
        shadowLightIndex: i32,
        bias: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        supportsSoftShadows: bool,
        lightProjectionMatrix: crate::UnityEngine::Matrix4x4,
        shadowResolution: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::VisibleLight,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::Vector4,
                                >,
                            >,
                            bool,
                            crate::UnityEngine::Matrix4x4,
                            f32,
                        ),
                        crate::UnityEngine::Vector4,
                        6usize,
                    >("GetShadowBias")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetShadowBias", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        shadowLight,
                        shadowLightIndex,
                        bias,
                        supportsSoftShadows,
                        lightProjectionMatrix,
                        shadowResolution,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowBias_UniversalShadowData_Matrix4x4_f32_1(
        shadowLight: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::VisibleLight,
        >,
        shadowLightIndex: i32,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
        lightProjectionMatrix: crate::UnityEngine::Matrix4x4,
        shadowResolution: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::VisibleLight,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            f32,
                        ),
                        crate::UnityEngine::Vector4,
                        5usize,
                    >("GetShadowBias")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetShadowBias", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        shadowLight,
                        shadowLightIndex,
                        shadowData,
                        lightProjectionMatrix,
                        shadowResolution,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowTransform(
        proj: crate::UnityEngine::Matrix4x4,
        view: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4),
                        crate::UnityEngine::Matrix4x4,
                        2usize,
                    >("GetShadowTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetShadowTransform", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            cordl_method_info.invoke_unchecked((), (proj, view))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryShadowTexture(
        width: i32,
        height: i32,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        3usize,
                    >("GetTemporaryShadowTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTemporaryShadowTexture", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = unsafe {
            cordl_method_info.invoke_unchecked((), (width, height, bits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryShadowTextureDescriptor(
        width: i32,
        height: i32,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureDescriptor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32),
                        crate::UnityEngine::RenderTextureDescriptor,
                        3usize,
                    >("GetTemporaryShadowTextureDescriptor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTemporaryShadowTextureDescriptor", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureDescriptor = unsafe {
            cordl_method_info.invoke_unchecked((), (width, height, bits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidShadowCastingLight_LightType_LightShadows_f32_1(
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        i: i32,
        lightType: crate::UnityEngine::LightType,
        lightShadows: crate::UnityEngine::LightShadows,
        shadowStrength: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalLightData,
                            >,
                            i32,
                            crate::UnityEngine::LightType,
                            crate::UnityEngine::LightShadows,
                            f32,
                        ),
                        bool,
                        5usize,
                    >("IsValidShadowCastingLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValidShadowCastingLight", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (lightData, i, lightType, lightShadows, shadowStrength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidShadowCastingLight_UniversalLightData_i32_0(
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalLightData,
                            >,
                            i32,
                        ),
                        bool,
                        2usize,
                    >("IsValidShadowCastingLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValidShadowCastingLight", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (lightData, i))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MinimalPunctualLightShadowResolution(
        softShadow: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool),
                        i32,
                        1usize,
                    >("MinimalPunctualLightShadowResolution")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MinimalPunctualLightShadowResolution", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (softShadow))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderShadowSlice_CommandBuffer_ByRefMut2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        shadowSliceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowDrawingSettings,
        >,
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ScriptableRenderContext,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowSliceData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ShadowDrawingSettings,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("RenderShadowSlice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderShadowSlice", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, context, shadowSliceData, settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderShadowSlice_CommandBuffer_ByRefMut_Matrix4x4_Matrix4x4_0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        shadowSliceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowDrawingSettings,
        >,
        proj: crate::UnityEngine::Matrix4x4,
        view: crate::UnityEngine::Matrix4x4,
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ScriptableRenderContext,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowSliceData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ShadowDrawingSettings,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("RenderShadowSlice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderShadowSlice", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, context, shadowSliceData, settings, proj, view),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderShadowSlice_RasterCommandBuffer_Matrix4x4_Matrix4x4_1(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        shadowSliceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
        shadowRendererList: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RendererList,
        >,
        proj: crate::UnityEngine::Matrix4x4,
        view: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ShadowSliceData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RendererList,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("RenderShadowSlice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderShadowSlice", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, shadowSliceData, shadowRendererList, proj, view),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCameraPosition(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        worldSpaceCameraPos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetCameraPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetCameraPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, worldSpaceCameraPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLightDirection(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        lightDirection: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetLightDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetLightDirection", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, lightDirection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLightPosition(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        lightPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetLightPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetLightPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, lightPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerLightSoftShadowKeyword(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        hasSoftShadows: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetPerLightSoftShadowKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetPerLightSoftShadowKeyword", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, hasSoftShadows))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetShadowBias(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        shadowBias: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetShadowBias")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetShadowBias", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, shadowBias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSoftShadowQualityShaderKeywords(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetSoftShadowQualityShaderKeywords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetSoftShadowQualityShaderKeywords", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, shadowData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWorldToCameraAndCameraToWorldMatrices(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        viewMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetWorldToCameraAndCameraToWorldMatrices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetWorldToCameraAndCameraToWorldMatrices", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, viewMatrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupShadowCasterConstantBuffer_CommandBuffer0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        shadowLight: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::VisibleLight,
        >,
        shadowBias: crate::UnityEngine::Vector4,
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::VisibleLight,
                            >,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetupShadowCasterConstantBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupShadowCasterConstantBuffer", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, shadowLight, shadowBias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupShadowCasterConstantBuffer_RasterCommandBuffer1(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        shadowLight: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::VisibleLight,
        >,
        shadowBias: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::VisibleLight,
                            >,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetupShadowCasterConstantBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupShadowCasterConstantBuffer", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, shadowLight, shadowBias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShadowRTNeedsReAlloc(
        handle: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        width: i32,
        height: i32,
        bits: i32,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        7usize,
                    >("ShadowRTNeedsReAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShadowRTNeedsReAlloc", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (handle, width, height, bits, anisoLevel, mipMapBias, name),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShadowRTReAllocateIfNeeded(
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        width: i32,
        height: i32,
        bits: i32,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::RTHandle,
                                >,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        7usize,
                    >("ShadowRTReAllocateIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShadowRTReAllocateIfNeeded", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (handle, width, height, bits, anisoLevel, mipMapBias, name),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SoftShadowQualityToShaderProperty(
        light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        softShadowsEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>, bool),
                        f32,
                        2usize,
                    >("SoftShadowQualityToShaderProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SoftShadowQualityToShaderProperty", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (light, softShadowsEnabled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsPerLightSoftShadowQuality() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        bool,
                        0usize,
                    >("SupportsPerLightSoftShadowQuality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SupportsPerLightSoftShadowQuality", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::ShadowUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
