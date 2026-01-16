#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct AdditionalLightsShadowAtlasLayout {
    pub m_SortedShadowResolutionRequests: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
    >,
    pub m_VisibleLightIndexToSortedShadowResolutionRequestsFirstSliceIndex: crate::Unity::Collections::NativeArray_1<
        i32,
    >,
    pub m_TotalShadowSlicesCount: i32,
    pub m_TotalShadowResolutionRequestCount: i32,
    pub m_TooManyShadowMaps: bool,
    pub m_ShadowSlicesScaleFactor: i32,
    pub m_AtlasSize: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "AdditionalLightsShadowAtlasLayout";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout
{
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout
{
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout")]
impl crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
    )]
    pub type ShadowResolutionRequest = crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest;
    pub fn ClearStaticCaches() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ClearStaticCaches",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearStaticCaches",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCompareShadowResolutionRequesPredicate() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                i32,
            >,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_3<
                                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                                i32,
                            >,
                        >,
                        0usize,
                    >("CreateCompareShadowResolutionRequesPredicate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateCompareShadowResolutionRequesPredicate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                i32,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EstimateScaleFactorNeededToFitAllShadowsInAtlas(
        shadowResolutionRequests: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
            >,
        >,
        endIndex: i32,
        atlasSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<
                                    crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                                >,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("EstimateScaleFactorNeededToFitAllShadowsInAtlas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EstimateScaleFactorNeededToFitAllShadowsInAtlas", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (shadowResolutionRequests, endIndex, atlasSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAtlasSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetAtlasSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAtlasSize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowSlicesScaleFactor(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetShadowSlicesScaleFactor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetShadowSlicesScaleFactor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSliceShadowResolutionRequest(
        &mut self,
        originalVisibleLightIndex: i32,
        sliceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32),
                        crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                        2usize,
                    >("GetSliceShadowResolutionRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSliceShadowResolutionRequest", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (originalVisibleLightIndex, sliceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSortedShadowResolutionRequest(
        &mut self,
        sortedShadowResolutionRequestIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest,
                        1usize,
                    >("GetSortedShadowResolutionRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSortedShadowResolutionRequest", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (sortedShadowResolutionRequestIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalShadowResolutionRequestCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetTotalShadowResolutionRequestCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTotalShadowResolutionRequestCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalShadowSlicesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetTotalShadowSlicesCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTotalShadowSlicesCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HasSpaceForLight(
        &mut self,
        originalVisibleLightIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("HasSpaceForLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasSpaceForLight",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (originalVisibleLightIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasTooManyShadowMaps(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("HasTooManyShadowMaps")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasTooManyShadowMaps",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalLightData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lightData, shadowData, cameraData))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
    pub visibleLightIndex: u16,
    pub perLightShadowSliceIndex: u16,
    pub requestedResolution: u16,
    pub offsetX: u16,
    pub offsetY: u16,
    pub allocatedResolution: u16,
    pub m_ShadowProperties: crate::UnityEngine::Rendering::Universal::ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "AdditionalLightsShadowAtlasLayout/ShadowResolutionRequest";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest"
)]
impl crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout_ShadowResolutionRequest {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest+SettingsOptions"
    )]
    pub type SettingsOptions = crate::UnityEngine::Rendering::Universal::ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions;
    pub fn get_pointLightShadow(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_pointLightShadow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_pointLightShadow", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_softShadow(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_softShadow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_softShadow", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_pointLightShadow(
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
                    >("set_pointLightShadow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_pointLightShadow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_softShadow(
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
                    >("set_softShadow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_softShadow", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest+SettingsOptions"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions {
    #[default]
    All = 65535u16,
    None = 0u16,
    PointLightShadow = 2u16,
    SoftShadow = 1u16,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest+SettingsOptions"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "AdditionalLightsShadowAtlasLayout/ShadowResolutionRequest/SettingsOptions";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest+SettingsOptions"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest+SettingsOptions"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest+SettingsOptions"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+AdditionalLightsShadowAtlasLayout+ShadowResolutionRequest+SettingsOptions"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::ShadowResolutionRequest_AdditionalLightsShadowAtlasLayout_SettingsOptions {
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
