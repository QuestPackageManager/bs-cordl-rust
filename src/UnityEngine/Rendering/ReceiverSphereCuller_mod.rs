#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct ReceiverSphereCuller {
    pub splitInfos: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo,
    >,
    pub worldToLightSpaceRotation: crate::Unity::Mathematics::float3x3,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::ReceiverSphereCuller {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ReceiverSphereCuller";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ReceiverSphereCuller
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ReceiverSphereCuller
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ReceiverSphereCuller
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::ReceiverSphereCuller {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::ReceiverSphereCuller
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ReceiverSphereCuller")]
impl crate::UnityEngine::Rendering::ReceiverSphereCuller {
    #[cfg(feature = "UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
    pub type SplitInfo = crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo;
    pub fn ComputeSplitVisibilityMask(
        lightFacingFrustumPlanes: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Plane,
        >,
        splitInfos: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo,
        >,
        worldToLightSpaceRotation: crate::Unity::Mathematics::float3x3,
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo,
                        >,
                        crate::Unity::Mathematics::float3x3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
                    ), u32, 4usize>("ComputeSplitVisibilityMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputeSplitVisibilityMask",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    lightFacingFrustumPlanes,
                    splitInfos,
                    worldToLightSpaceRotation,
                    bounds,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        cc: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::BatchCullingContext>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ReceiverSphereCuller> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::BatchCullingContext,
                        >,
                        crate::Unity::Collections::Allocator,
                    ), crate::UnityEngine::Rendering::ReceiverSphereCuller, 2usize>(
                        "Create"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Create",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ReceiverSphereCuller =
            unsafe { cordl_method_info.invoke_unchecked((), (cc, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEmptyForTesting(
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ReceiverSphereCuller> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::Allocator),
                        crate::UnityEngine::Rendering::ReceiverSphereCuller,
                        1usize,
                    >("CreateEmptyForTesting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEmptyForTesting", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ReceiverSphereCuller =
            unsafe { cordl_method_info.invoke_unchecked((), (allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        job: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Jobs::JobHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (job))? };
        Ok(__cordl_ret.into())
    }
    pub fn DistanceUntilCylinderFullyCrossesPlane(
        cylinderCenter: crate::Unity::Mathematics::float3,
        cylinderDirection: crate::Unity::Mathematics::float3,
        cylinderRadius: f32,
        plane: crate::UnityEngine::Plane,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float3,
                        crate::Unity::Mathematics::float3,
                        f32,
                        crate::UnityEngine::Plane,
                    ), f32, 4usize>("DistanceUntilCylinderFullyCrossesPlane")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DistanceUntilCylinderFullyCrossesPlane",
                            4usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (cylinderCenter, cylinderDirection, cylinderRadius, plane),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UseReceiverPlanes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("UseReceiverPlanes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseReceiverPlanes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct ReceiverSphereCuller_SplitInfo {
    pub receiverSphereLightSpace: crate::Unity::Mathematics::float4,
    pub cascadeBlendCullingFactor: f32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ReceiverSphereCuller/SplitInfo";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ReceiverSphereCuller+SplitInfo")]
impl crate::UnityEngine::Rendering::ReceiverSphereCuller_SplitInfo {}
