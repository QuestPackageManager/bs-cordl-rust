#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrustumPlaneCuller {
    pub planePackets: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4,
    >,
    pub splitInfos: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::FrustumPlaneCuller {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "FrustumPlaneCuller";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::FrustumPlaneCuller {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::FrustumPlaneCuller {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::FrustumPlaneCuller {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::FrustumPlaneCuller {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::FrustumPlaneCuller {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+FrustumPlaneCuller")]
impl crate::UnityEngine::Rendering::FrustumPlaneCuller {
    #[cfg(feature = "UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
    pub type PlanePacket4 = crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4;
    #[cfg(feature = "UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
    pub type SplitInfo = crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo;
    pub fn ComputeSplitVisibilityMask(
        planePackets: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4,
        >,
        splitInfos: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo,
        >,
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::AABB>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::AABB,
                            >,
                        ),
                        u32,
                        3usize,
                    >("ComputeSplitVisibilityMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeSplitVisibilityMask", 3usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked((), (planePackets, splitInfos, bounds))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        cc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::BatchCullingContext,
        >,
        receiverPlanes: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Plane,
        >,
        receiverSphereCuller: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ReceiverSphereCuller,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::FrustumPlaneCuller,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::BatchCullingContext,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Plane,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ReceiverSphereCuller,
                            >,
                            crate::Unity::Collections::Allocator,
                        ),
                        crate::UnityEngine::Rendering::FrustumPlaneCuller,
                        4usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::FrustumPlaneCuller = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cc, receiverPlanes, receiverSphereCuller, allocator),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        job: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (job))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrustumPlaneCuller_PlanePacket4 {
    pub nx: crate::Unity::Mathematics::float4,
    pub ny: crate::Unity::Mathematics::float4,
    pub nz: crate::Unity::Mathematics::float4,
    pub d: crate::Unity::Mathematics::float4,
    pub nxAbs: crate::Unity::Mathematics::float4,
    pub nyAbs: crate::Unity::Mathematics::float4,
    pub nzAbs: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "FrustumPlaneCuller/PlanePacket4";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4 {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4 {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4 {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+FrustumPlaneCuller+PlanePacket4")]
impl crate::UnityEngine::Rendering::FrustumPlaneCuller_PlanePacket4 {
    pub fn _ctor(
        &mut self,
        planes: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
        offset: i32,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Plane,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (planes, offset, limit))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrustumPlaneCuller_SplitInfo {
    pub packetCount: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "FrustumPlaneCuller/SplitInfo";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+FrustumPlaneCuller+SplitInfo")]
impl crate::UnityEngine::Rendering::FrustumPlaneCuller_SplitInfo {}
