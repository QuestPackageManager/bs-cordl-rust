#[cfg(feature = "cordl_class_Unity+Mathematics+Geometry+Math")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct Math {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Mathematics+Geometry+Math")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::Geometry::Math {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Mathematics.Geometry";
    const CLASS_NAME: &'static str = "Math";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
impl std::ops::Deref for crate::Unity::Mathematics::Geometry::Math {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
impl std::ops::DerefMut for crate::Unity::Mathematics::Geometry::Math {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
impl crate::Unity::Mathematics::Geometry::Math {
    pub fn Transform_RigidTransform0(
        transform: crate::Unity::Mathematics::RigidTransform,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::MinMaxAABB> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::RigidTransform,
                        crate::Unity::Mathematics::Geometry::MinMaxAABB,
                    ), crate::Unity::Mathematics::Geometry::MinMaxAABB, 2usize>(
                        "Transform"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Transform",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::Geometry::MinMaxAABB =
            unsafe { cordl_method_info.invoke_unchecked((), (transform, aabb))? };
        Ok(__cordl_ret.into())
    }
    pub fn Transform_float3x3_2(
        transform: crate::Unity::Mathematics::float3x3,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::MinMaxAABB> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float3x3,
                        crate::Unity::Mathematics::Geometry::MinMaxAABB,
                    ), crate::Unity::Mathematics::Geometry::MinMaxAABB, 2usize>(
                        "Transform"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Transform",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::Geometry::MinMaxAABB =
            unsafe { cordl_method_info.invoke_unchecked((), (transform, aabb))? };
        Ok(__cordl_ret.into())
    }
    pub fn Transform_float4x4_1(
        transform: crate::Unity::Mathematics::float4x4,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::MinMaxAABB> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Mathematics::float4x4,
                        crate::Unity::Mathematics::Geometry::MinMaxAABB,
                    ), crate::Unity::Mathematics::Geometry::MinMaxAABB, 2usize>(
                        "Transform"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Transform",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::Geometry::MinMaxAABB =
            unsafe { cordl_method_info.invoke_unchecked((), (transform, aabb))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Mathematics+Geometry+Math")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Mathematics::Geometry::Math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
