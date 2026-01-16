#[cfg(feature = "cordl_class_Unity+Mathematics+svd")]
#[repr(C)]
#[derive(Debug)]
pub struct svd {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Mathematics+svd")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::svd {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "svd";
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
#[cfg(feature = "Unity+Mathematics+svd")]
impl std::ops::Deref for crate::Unity::Mathematics::svd {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+svd")]
impl std::ops::DerefMut for crate::Unity::Mathematics::svd {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+svd")]
impl crate::Unity::Mathematics::svd {
    pub const k_EpsilonDeterminant: f32 = 0.000001f32;
    pub const k_EpsilonNormal: f32 = 0.000000000000000000000000000001f32;
    pub const k_EpsilonNormalSqrt: f32 = 0.000000000000001f32;
    pub const k_EpsilonRCP: f32 = 0.000000001f32;
    pub fn approxGivensQuat(
        pq: crate::Unity::Mathematics::float3,
        mask: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float4,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("approxGivensQuat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "approxGivensQuat", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (pq, mask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn condNegSwap(
        c: bool,
        x: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
        y: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("condNegSwap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "condNegSwap", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (c, x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn condNegSwapQuat(
        c: bool,
        q: crate::Unity::Mathematics::quaternion,
        mask: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            bool,
                            crate::Unity::Mathematics::quaternion,
                            crate::Unity::Mathematics::float4,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        3usize,
                    >("condNegSwapQuat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "condNegSwapQuat", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (c, q, mask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn condSwap(
        c: bool,
        x: quest_hook::libil2cpp::ByRefMut<f32>,
        y: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("condSwap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "condSwap", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (c, x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn givensQRFactorization(
        b: crate::Unity::Mathematics::float3x3,
        r: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3x3>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3x3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3x3,
                            >,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("givensQRFactorization")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "givensQRFactorization", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (b, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn jacobiIteration(
        s: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3x3>,
        iterations: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3x3,
                            >,
                            i32,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("jacobiIteration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "jacobiIteration", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (s, iterations))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn qrGivensQuat(
        pq: crate::Unity::Mathematics::float2,
        mask: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float2,
                            crate::Unity::Mathematics::float4,
                        ),
                        crate::Unity::Mathematics::quaternion,
                        2usize,
                    >("qrGivensQuat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "qrGivensQuat", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (pq, mask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn rcpsafe(
        x: crate::Unity::Mathematics::float3,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3, f32),
                        crate::Unity::Mathematics::float3,
                        2usize,
                    >("rcpsafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "rcpsafe",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 = unsafe {
            cordl_method_info.invoke_unchecked((), (x, epsilon))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn singularValuesDecomposition(
        a: crate::Unity::Mathematics::float3x3,
        u: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::quaternion>,
        v: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::quaternion>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3x3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::quaternion,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::quaternion,
                            >,
                        ),
                        crate::Unity::Mathematics::float3,
                        3usize,
                    >("singularValuesDecomposition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "singularValuesDecomposition", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 = unsafe {
            cordl_method_info.invoke_unchecked((), (a, u, v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn sortSingularValues(
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3x3>,
        v: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3x3,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::quaternion,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("sortSingularValues")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "sortSingularValues", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (b, v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn svdInverse(
        a: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3x3),
                        crate::Unity::Mathematics::float3x3,
                        1usize,
                    >("svdInverse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "svdInverse", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = unsafe {
            cordl_method_info.invoke_unchecked((), (a))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn svdRotation(
        a: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Mathematics::float3x3),
                        crate::Unity::Mathematics::quaternion,
                        1usize,
                    >("svdRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "svdRotation", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (a))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Mathematics+svd")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Mathematics::svd {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
