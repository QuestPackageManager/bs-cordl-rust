#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingResults")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CullingResults {
    pub ptr: crate::System::IntPtr,
    pub m_AllocationInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingResults")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CullingResults {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CullingResults";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingResults")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::CullingResults {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingResults")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::CullingResults {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingResults")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::CullingResults {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingResults")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::CullingResults {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CullingResults")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CullingResults {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CullingResults")]
impl crate::UnityEngine::Rendering::CullingResults {
    pub fn ComputeDirectionalShadowMatricesAndCullingPrimitives_Injected(
        cullingResultsPtr: crate::System::IntPtr,
        activeLightIndex: i32,
        splitIndex: i32,
        splitCount: i32,
        splitRatio: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        shadowResolution: i32,
        shadowNearPlaneOffset: f32,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        shadowSplitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            i32,
                            f32,
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
                        10usize,
                    >("ComputeDirectionalShadowMatricesAndCullingPrimitives_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeDirectionalShadowMatricesAndCullingPrimitives_Injected",
                            10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullingResultsPtr,
                        activeLightIndex,
                        splitIndex,
                        splitCount,
                        splitRatio,
                        shadowResolution,
                        shadowNearPlaneOffset,
                        viewMatrix,
                        projMatrix,
                        shadowSplitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeDirectionalShadowMatricesAndCullingPrimitives_IntPtr_i32_Vector3_i32_f32_ByRefMut0(
        cullingResultsPtr: crate::System::IntPtr,
        activeLightIndex: i32,
        splitIndex: i32,
        splitCount: i32,
        splitRatio: crate::UnityEngine::Vector3,
        shadowResolution: i32,
        shadowNearPlaneOffset: f32,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        shadowSplitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Vector3,
                            i32,
                            f32,
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
                        10usize,
                    >("ComputeDirectionalShadowMatricesAndCullingPrimitives")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeDirectionalShadowMatricesAndCullingPrimitives",
                            10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullingResultsPtr,
                        activeLightIndex,
                        splitIndex,
                        splitCount,
                        splitRatio,
                        shadowResolution,
                        shadowNearPlaneOffset,
                        viewMatrix,
                        projMatrix,
                        shadowSplitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeDirectionalShadowMatricesAndCullingPrimitives_i32_Vector3_i32_f32_ByRefMut1(
        &mut self,
        activeLightIndex: i32,
        splitIndex: i32,
        splitCount: i32,
        splitRatio: crate::UnityEngine::Vector3,
        shadowResolution: i32,
        shadowNearPlaneOffset: f32,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        shadowSplitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Vector3,
                            i32,
                            f32,
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
                    >("ComputeDirectionalShadowMatricesAndCullingPrimitives")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeDirectionalShadowMatricesAndCullingPrimitives",
                            9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        activeLightIndex,
                        splitIndex,
                        splitCount,
                        splitRatio,
                        shadowResolution,
                        shadowNearPlaneOffset,
                        viewMatrix,
                        projMatrix,
                        shadowSplitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputePointShadowMatricesAndCullingPrimitives_IntPtr_i32_CubemapFace_f32_ByRefMut0(
        cullingResultsPtr: crate::System::IntPtr,
        activeLightIndex: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        fovBias: f32,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        shadowSplitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
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
                                crate::UnityEngine::Rendering::ShadowSplitData,
                            >,
                        ),
                        bool,
                        7usize,
                    >("ComputePointShadowMatricesAndCullingPrimitives")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputePointShadowMatricesAndCullingPrimitives", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullingResultsPtr,
                        activeLightIndex,
                        cubemapFace,
                        fovBias,
                        viewMatrix,
                        projMatrix,
                        shadowSplitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputePointShadowMatricesAndCullingPrimitives_i32_CubemapFace_f32_ByRefMut1(
        &mut self,
        activeLightIndex: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        fovBias: f32,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        shadowSplitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
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
                                crate::UnityEngine::Rendering::ShadowSplitData,
                            >,
                        ),
                        bool,
                        6usize,
                    >("ComputePointShadowMatricesAndCullingPrimitives")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputePointShadowMatricesAndCullingPrimitives", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        activeLightIndex,
                        cubemapFace,
                        fovBias,
                        viewMatrix,
                        projMatrix,
                        shadowSplitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeSpotShadowMatricesAndCullingPrimitives_IntPtr_i32_ByRefMut0(
        cullingResultsPtr: crate::System::IntPtr,
        activeLightIndex: i32,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        shadowSplitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            i32,
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
                        5usize,
                    >("ComputeSpotShadowMatricesAndCullingPrimitives")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeSpotShadowMatricesAndCullingPrimitives", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        cullingResultsPtr,
                        activeLightIndex,
                        viewMatrix,
                        projMatrix,
                        shadowSplitData,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeSpotShadowMatricesAndCullingPrimitives_i32_ByRefMut1(
        &mut self,
        activeLightIndex: i32,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        shadowSplitData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowSplitData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
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
                        4usize,
                    >("ComputeSpotShadowMatricesAndCullingPrimitives")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeSpotShadowMatricesAndCullingPrimitives", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (activeLightIndex, viewMatrix, projMatrix, shadowSplitData),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_CullingResults0(
        &mut self,
        other: crate::UnityEngine::Rendering::CullingResults,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::CullingResults),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLightAndReflectionProbeIndices_ComputeBuffer1(
        &mut self,
        computeBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FillLightAndReflectionProbeIndices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FillLightAndReflectionProbeIndices", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLightAndReflectionProbeIndices_Injected(
        cullingResultsPtr: crate::System::IntPtr,
        computeBuffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("FillLightAndReflectionProbeIndices_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FillLightAndReflectionProbeIndices_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cullingResultsPtr, computeBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLightAndReflectionProbeIndices_IntPtr_ComputeBuffer0(
        cullingResultsPtr: crate::System::IntPtr,
        computeBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("FillLightAndReflectionProbeIndices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FillLightAndReflectionProbeIndices", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cullingResultsPtr, computeBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLightIndexMap(
        cullingResultsPtr: crate::System::IntPtr,
        indexMapPtr: crate::System::IntPtr,
        indexMapSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("FillLightIndexMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FillLightIndexMap", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cullingResultsPtr, indexMapPtr, indexMapSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLightIndexCount(
        cullingResultsPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        i32,
                        1usize,
                    >("GetLightIndexCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLightIndexCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (cullingResultsPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLightIndexMap(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::Allocator),
                        crate::Unity::Collections::NativeArray_1<i32>,
                        1usize,
                    >("GetLightIndexMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLightIndexMap", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<i32> = unsafe {
            cordl_method_info.invoke_unchecked(self, (allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLightIndexMapSize(
        cullingResultsPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        i32,
                        1usize,
                    >("GetLightIndexMapSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLightIndexMapSize", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (cullingResultsPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeArray<T>(
        &mut self,
        dataPointer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::NativeArray_1<T>,
                        2usize,
                    >("GetNativeArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNativeArray", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = unsafe {
            cordl_method_info.invoke_unchecked(self, (dataPointer, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetReflectionProbeIndexCount(
        cullingResultsPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        i32,
                        1usize,
                    >("GetReflectionProbeIndexCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetReflectionProbeIndexCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (cullingResultsPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowCasterBounds_IntPtr_i32_ByRefMut0(
        cullingResultsPtr: crate::System::IntPtr,
        lightIndex: i32,
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
                        ),
                        bool,
                        3usize,
                    >("GetShadowCasterBounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetShadowCasterBounds", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cullingResultsPtr, lightIndex, bounds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowCasterBounds_i32_ByRefMut1(
        &mut self,
        lightIndex: i32,
        outBounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
                        ),
                        bool,
                        2usize,
                    >("GetShadowCasterBounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetShadowCasterBounds", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (lightIndex, outBounds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLightIndexMap_IntPtr_IntPtr_i32_0(
        cullingResultsPtr: crate::System::IntPtr,
        indexMapPtr: crate::System::IntPtr,
        indexMapSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetLightIndexMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetLightIndexMap", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cullingResultsPtr, indexMapPtr, indexMapSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLightIndexMap_NativeArray_1_1(
        &mut self,
        lightIndexMap: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetLightIndexMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetLightIndexMap", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lightIndexMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_lightAndReflectionProbeIndexCount(
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
                    >("get_lightAndReflectionProbeIndexCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_lightAndReflectionProbeIndexCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_visibleLights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::VisibleLight,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::VisibleLight,
                        >,
                        0usize,
                    >("get_visibleLights")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_visibleLights", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::VisibleLight,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_visibleReflectionProbes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::VisibleReflectionProbe,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::VisibleReflectionProbe,
                        >,
                        0usize,
                    >("get_visibleReflectionProbes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_visibleReflectionProbes", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::VisibleReflectionProbe,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::Rendering::CullingResults,
        right: crate::UnityEngine::Rendering::CullingResults,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::CullingResults,
                            crate::UnityEngine::Rendering::CullingResults,
                        ),
                        bool,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (left, right))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CullingResults")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::CullingResults>>
for crate::UnityEngine::Rendering::CullingResults {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::CullingResults> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CullingResults")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::CullingResults>>
for crate::UnityEngine::Rendering::CullingResults {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::CullingResults,
    > {
        todo!()
    }
}
