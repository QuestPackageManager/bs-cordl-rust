#[cfg(feature = "cordl_class_UnityEngine+Physics")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Physics")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Physics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Physics";
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
#[cfg(feature = "UnityEngine+Physics")]
impl std::ops::Deref for crate::UnityEngine::Physics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics")]
impl std::ops::DerefMut for crate::UnityEngine::Physics {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics")]
impl crate::UnityEngine::Physics {
    pub const AllLayers: i32 = -1i32;
    pub const DefaultRaycastLayers: i32 = -5i32;
    pub const IgnoreRaycastLayer: i32 = 4i32;
    pub const kAllLayers: i32 = -1i32;
    pub const kDefaultRaycastLayers: i32 = -5i32;
    pub const kIgnoreRaycastLayer: i32 = 4i32;
    pub const k_MaxFloatMinusEpsilon: f32 = 340282330000000000000000000000000000000f32;
    #[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
    pub type ContactEventDelegate = crate::UnityEngine::Physics_ContactEventDelegate;
    pub fn BakeMesh_MeshColliderCookingOptions0(
        meshID: i32,
        convex: bool,
        cookingOptions: crate::UnityEngine::MeshColliderCookingOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, bool, crate::UnityEngine::MeshColliderCookingOptions),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("BakeMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BakeMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (meshID, convex, cookingOptions))? };
        Ok(__cordl_ret.into())
    }
    pub fn BakeMesh_i32__cordl_bool1(
        meshID: i32,
        convex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, bool), quest_hook::libil2cpp::Void, 2usize>(
                        "BakeMesh",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BakeMesh",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (meshID, convex))? };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastAll_Quaternion3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 4usize>("BoxCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastAll",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (center, halfExtents, direction, orientation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastAll_Quaternion_f32_2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        f32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 5usize>("BoxCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastAll",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (center, halfExtents, direction, orientation, maxDistance),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastAll_Quaternion_f32_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 6usize>("BoxCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastAll",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastAll_Quaternion_f32_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 7usize>("BoxCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastAll",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastAll_Vector3_Vector3_Vector3_4(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 3usize>("BoxCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastAll",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (center, halfExtents, direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastNonAlloc_Quaternion1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        crate::UnityEngine::Quaternion,
                    ), i32, 5usize>("BoxCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (center, halfExtents, direction, results, orientation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastNonAlloc_Quaternion_f32_2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        crate::UnityEngine::Quaternion,
                        f32,
                    ), i32, 6usize>("BoxCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastNonAlloc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    results,
                    orientation,
                    maxDistance,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastNonAlloc_Quaternion_f32_i32_3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                    ), i32, 7usize>("BoxCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastNonAlloc",
                            7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    results,
                    orientation,
                    maxDistance,
                    layerMask,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastNonAlloc_Quaternion_f32_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 8usize>("BoxCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastNonAlloc",
                            8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    results,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCastNonAlloc_Vector3_Vector3_Vector3_Il2CppArray4(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                    ), i32, 4usize>("BoxCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCastNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (center, halfExtents, direction, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut9(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    ), bool, 4usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (center, halfExtents, direction, hitInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut_Quaternion8(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        crate::UnityEngine::Quaternion,
                    ), bool, 5usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (center, halfExtents, direction, hitInfo, orientation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut_Quaternion_f32_7(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        crate::UnityEngine::Quaternion,
                        f32,
                    ), bool, 6usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    hitInfo,
                    orientation,
                    maxDistance,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut_Quaternion_f32_i32_6(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                    ), bool, 7usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    hitInfo,
                    orientation,
                    maxDistance,
                    layerMask,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut_Quaternion_f32_i32_QueryTriggerInteraction5(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 8usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    hitInfo,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Quaternion3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                    ), bool, 4usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (center, halfExtents, direction, orientation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Quaternion_f32_2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        f32,
                    ), bool, 5usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (center, halfExtents, direction, orientation, maxDistance),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Quaternion_f32_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                    ), bool, 6usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Quaternion_f32_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 7usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Vector3_Vector3_Vector3_4(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                    ), bool, 3usize>("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BoxCast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (center, halfExtents, direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastAll_Vector3_Vector3_f32_Vector3_3(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 4usize>("CapsuleCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastAll",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (point1, point2, radius, direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastAll_f32_2(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 5usize>("CapsuleCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastAll",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (point1, point2, radius, direction, maxDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastAll_f32_i32_1(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 6usize>("CapsuleCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastAll",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (point1, point2, radius, direction, maxDistance, layerMask),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastAll_f32_i32_QueryTriggerInteraction0(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 7usize>("CapsuleCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastAll",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastNonAlloc_Vector3_Vector3_f32_Vector3_Il2CppArray3(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                    ), i32, 5usize>("CapsuleCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (point1, point2, radius, direction, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastNonAlloc_f32_2(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                    ), i32, 6usize>("CapsuleCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastNonAlloc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (point1, point2, radius, direction, results, maxDistance),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastNonAlloc_f32_i32_1(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                    ), i32, 7usize>("CapsuleCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastNonAlloc",
                            7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    results,
                    maxDistance,
                    layerMask,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCastNonAlloc_f32_i32_QueryTriggerInteraction0(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 8usize>("CapsuleCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCastNonAlloc",
                            8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    results,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_ByRefMut7(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    ), bool, 5usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (point1, point2, radius, direction, hitInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_ByRefMut_f32_6(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                    ), bool, 6usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (point1, point2, radius, direction, hitInfo, maxDistance),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_ByRefMut_f32_i32_5(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                    ), bool, 7usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    hitInfo,
                    maxDistance,
                    layerMask,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_ByRefMut_f32_i32_QueryTriggerInteraction4(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 8usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    hitInfo,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_Vector3_Vector3_f32_Vector3_3(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                    ), bool, 4usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (point1, point2, radius, direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_f32_2(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), bool, 5usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (point1, point2, radius, direction, maxDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_f32_i32_1(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                    ), bool, 6usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (point1, point2, radius, direction, maxDistance, layerMask),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_f32_i32_QueryTriggerInteraction0(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 7usize>("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CapsuleCast",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layermask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 6usize>("CheckBox_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckBox_Internal",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layermask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        halfExtents: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        orientation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        layermask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 6usize>("CheckBox_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckBox_Internal_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layermask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Quaternion2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                    ), bool, 3usize>("CheckBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckBox",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (center, halfExtents, orientation))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Quaternion_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        i32,
                    ), bool, 4usize>("CheckBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckBox",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (center, halfExtents, orientation, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Quaternion_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layermask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("CheckBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckBox",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    orientation,
                    layermask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Vector3_Vector3_3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        bool,
                        2usize,
                    >("CheckBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckBox", 2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (center, halfExtents))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 6usize>("CheckCapsule_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckCapsule_Internal",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    start,
                    end,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        start: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        end: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 6usize>("CheckCapsule_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckCapsule_Internal_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    start,
                    end,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_Vector3_Vector3_f32_2(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), bool, 3usize>("CheckCapsule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckCapsule",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (start, end, radius))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_i32_1(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                    ), bool, 4usize>("CheckCapsule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckCapsule",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (start, end, radius, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_i32_QueryTriggerInteraction0(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("CheckCapsule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckCapsule",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (start, end, radius, layerMask, queryTriggerInteraction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("CheckSphere_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckSphere_Internal",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    position,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("CheckSphere_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckSphere_Internal_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    position,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_Vector3_f32_2(
        position: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Vector3, f32), bool, 2usize>(
                        "CheckSphere",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckSphere",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (position, radius))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_i32_1(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Vector3, f32, i32), bool, 3usize>(
                        "CheckSphere",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckSphere",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (position, radius, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_i32_QueryTriggerInteraction0(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 4usize>("CheckSphere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckSphere",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (position, radius, layerMask, queryTriggerInteraction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClosestPoint(
        point: crate::UnityEngine::Vector3,
        collider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                    ), crate::UnityEngine::Vector3, 4usize>("ClosestPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClosestPoint",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (point, collider, position, rotation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputePenetration(
        colliderA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        positionA: crate::UnityEngine::Vector3,
        rotationA: crate::UnityEngine::Quaternion,
        colliderB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        positionB: crate::UnityEngine::Vector3,
        rotationB: crate::UnityEngine::Quaternion,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<f32>,
                    ), bool, 8usize>("ComputePenetration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputePenetration",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    colliderA, positionA, rotationA, colliderB, positionB, rotationB, direction,
                    distance,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConnectPhysicsSDKVisualDebugger() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("ConnectPhysicsSDKVisualDebugger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConnectPhysicsSDKVisualDebugger",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectPhysicsSDKVisualDebugger(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "DisconnectPhysicsSDKVisualDebugger",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisconnectPhysicsSDKVisualDebugger",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBodyByInstanceID(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                        1usize,
                    >("GetBodyByInstanceID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBodyByInstanceID", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> =
            unsafe { cordl_method_info.invoke_unchecked((), (instanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBodyByInstanceID_Injected(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), crate::System::IntPtr, 1usize>(
                        "GetBodyByInstanceID_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetBodyByInstanceID_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked((), (instanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetColliderByInstanceID(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        1usize,
                    >("GetColliderByInstanceID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetColliderByInstanceID", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> =
            unsafe { cordl_method_info.invoke_unchecked((), (instanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetColliderByInstanceID_Injected(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), crate::System::IntPtr, 1usize>(
                        "GetColliderByInstanceID_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetColliderByInstanceID_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked((), (instanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCollisionToReport(
        header: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPairHeader>,
        pair: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPairHeader>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>, 3usize>(
                        "GetCollisionToReport",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCollisionToReport",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision> =
            unsafe { cordl_method_info.invoke_unchecked((), (header, pair, flipped))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentIntegrationId() -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), u32, 0usize>("GetCurrentIntegrationId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCurrentIntegrationId",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIgnoreCollision(
        collider1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        collider2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                    ), bool, 2usize>("GetIgnoreCollision")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetIgnoreCollision",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (collider1, collider2))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIgnoreCollision_Injected(
        collider1: crate::System::IntPtr,
        collider2: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr),
                        bool,
                        2usize,
                    >("GetIgnoreCollision_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetIgnoreCollision_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (collider1, collider2))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIgnoreLayerCollision(
        layer1: i32,
        layer2: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), bool, 2usize>("GetIgnoreLayerCollision")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetIgnoreLayerCollision",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (layer1, layer2))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIntegrationInfos_1() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<crate::UnityEngine::IntegrationInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::IntegrationInfo,
                        >,
                        0usize,
                    >("GetIntegrationInfos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetIntegrationInfos", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<crate::UnityEngine::IntegrationInfo> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIntegrationInfos_ByRefMut_ByRefMut0(
        integrations: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        integrationCount: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                        quest_hook::libil2cpp::ByRefMut<u64>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "GetIntegrationInfos"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetIntegrationInfos",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (integrations, integrationCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreCollision_Collider_Collider1(
        collider1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        collider2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IgnoreCollision")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IgnoreCollision",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (collider1, collider2))? };
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreCollision_Injected(
        collider1: crate::System::IntPtr,
        collider2: crate::System::IntPtr,
        ignore: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, bool),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("IgnoreCollision_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IgnoreCollision_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (collider1, collider2, ignore))? };
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreCollision__cordl_bool0(
        collider1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        collider2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        ignore: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("IgnoreCollision")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IgnoreCollision",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (collider1, collider2, ignore))? };
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreLayerCollision__cordl_bool0(
        layer1: i32,
        layer2: i32,
        ignore: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32, bool), quest_hook::libil2cpp::Void, 3usize>(
                        "IgnoreLayerCollision",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IgnoreLayerCollision",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (layer1, layer2, ignore))? };
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreLayerCollision_i32_i32_1(
        layer1: i32,
        layer2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "IgnoreLayerCollision",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IgnoreLayerCollision",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (layer1, layer2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BoxCastAll(
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 8usize>("Internal_BoxCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_BoxCastAll",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BoxCastAll_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        halfExtents: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        orientation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::BlittableArrayWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "Internal_BoxCastAll_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_BoxCastAll_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                    ret,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastAll(
        physicsScene: crate::UnityEngine::PhysicsScene,
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Ray,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 5usize>("Internal_RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RaycastAll",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    ray,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastAll_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::BlittableArrayWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_RaycastAll_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RaycastAll_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    ray,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                    ret,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InterpolateBodies_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::PhysicsScene),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InterpolateBodies_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InterpolateBodies_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (physicsScene))? };
        Ok(__cordl_ret.into())
    }
    pub fn InterpolateBodies_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::PhysicsScene,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InterpolateBodies_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InterpolateBodies_Internal_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (physicsScene))? };
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_ByRefMut5(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    ), bool, 3usize>("Linecast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Linecast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (start, end, hitInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_ByRefMut_i32_4(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        i32,
                    ), bool, 4usize>("Linecast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Linecast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (start, end, hitInfo, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_ByRefMut_i32_QueryTriggerInteraction3(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("Linecast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Linecast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (start, end, hitInfo, layerMask, queryTriggerInteraction),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_Vector3_Vector3_2(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        bool,
                        2usize,
                    >("Linecast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Linecast", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (start, end))? };
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_i32_1(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        i32,
                    ), bool, 3usize>("Linecast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Linecast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (start, end, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_i32_QueryTriggerInteraction0(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 4usize>("Linecast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Linecast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (start, end, layerMask, queryTriggerInteraction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnSceneContact(
        scene: crate::UnityEngine::PhysicsScene,
        buffer: crate::System::IntPtr,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::PhysicsScene, crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnSceneContact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSceneContact", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (scene, buffer, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnSceneContactModify(
        scene: crate::UnityEngine::PhysicsScene,
        buffer: crate::System::IntPtr,
        count: i32,
        isCCD: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::System::IntPtr,
                        i32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "OnSceneContactModify"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnSceneContactModify",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (scene, buffer, count, isCCD))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Quaternion2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        crate::UnityEngine::Quaternion,
                    ), i32, 4usize>("OverlapBoxNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBoxNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (center, halfExtents, results, orientation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Quaternion_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        orientation: crate::UnityEngine::Quaternion,
        mask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        crate::UnityEngine::Quaternion,
                        i32,
                    ), i32, 5usize>("OverlapBoxNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBoxNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (center, halfExtents, results, orientation, mask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Quaternion_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        orientation: crate::UnityEngine::Quaternion,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        crate::UnityEngine::Quaternion,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 6usize>("OverlapBoxNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBoxNonAlloc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    results,
                    orientation,
                    mask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Vector3_Vector3_Il2CppArray3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                    ), i32, 3usize>("OverlapBoxNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBoxNonAlloc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (center, halfExtents, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 6usize>("OverlapBox_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBox_Internal",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        halfExtents: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        orientation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 6usize>("OverlapBox_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBox_Internal_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Quaternion2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 3usize>("OverlapBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBox",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (center, halfExtents, orientation))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Quaternion_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 4usize>("OverlapBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBox",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (center, halfExtents, orientation, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Quaternion_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 5usize>("OverlapBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapBox",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    center,
                    halfExtents,
                    orientation,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Vector3_Vector3_3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        2usize,
                    >("OverlapBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapBox", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (center, halfExtents))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsuleNonAlloc_Vector3_Vector3_f32_Il2CppArray2(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                    ), i32, 4usize>("OverlapCapsuleNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsuleNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (point0, point1, radius, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsuleNonAlloc_i32_1(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        i32,
                    ), i32, 5usize>("OverlapCapsuleNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsuleNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (point0, point1, radius, results, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsuleNonAlloc_i32_QueryTriggerInteraction0(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 6usize>("OverlapCapsuleNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsuleNonAlloc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    point0,
                    point1,
                    radius,
                    results,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 6usize>("OverlapCapsule_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsule_Internal",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    point0,
                    point1,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        point0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        point1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 6usize>("OverlapCapsule_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsule_Internal_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    point0,
                    point1,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule_Vector3_Vector3_f32_2(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 3usize>("OverlapCapsule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsule",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (point0, point1, radius))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule_i32_1(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 4usize>("OverlapCapsule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsule",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (point0, point1, radius, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule_i32_QueryTriggerInteraction0(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 5usize>("OverlapCapsule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapCapsule",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (point0, point1, radius, layerMask, queryTriggerInteraction),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphereNonAlloc_Vector3_f32_Il2CppArray2(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                    ), i32, 3usize>("OverlapSphereNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapSphereNonAlloc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (position, radius, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphereNonAlloc_i32_1(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        i32,
                    ), i32, 4usize>("OverlapSphereNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapSphereNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (position, radius, results, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphereNonAlloc_i32_QueryTriggerInteraction0(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 5usize>("OverlapSphereNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapSphereNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    position,
                    radius,
                    results,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 5usize>("OverlapSphere_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapSphere_Internal",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    position,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 5usize>("OverlapSphere_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapSphere_Internal_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    position,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_Vector3_f32_2(
        position: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, f32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        2usize,
                    >("OverlapSphere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapSphere", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (position, radius))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_i32_1(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, f32, i32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                            >,
                        >,
                        3usize,
                    >("OverlapSphere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapSphere", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (position, radius, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_i32_QueryTriggerInteraction0(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        >,
                    >, 4usize>("OverlapSphere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OverlapSphere",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (position, radius, layerMask, queryTriggerInteraction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PhysXOnSceneContactModify(
        scene: crate::UnityEngine::PhysicsScene,
        buffer: crate::System::IntPtr,
        count: i32,
        isCCD: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::System::IntPtr,
                        i32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "PhysXOnSceneContactModify"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PhysXOnSceneContactModify",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (scene, buffer, count, isCCD))? };
        Ok(__cordl_ret.into())
    }
    pub fn Query_CapsuleCastAll(
        physicsScene: crate::UnityEngine::PhysicsScene,
        p0: crate::UnityEngine::Vector3,
        p1: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 8usize>("Query_CapsuleCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_CapsuleCastAll",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    p0,
                    p1,
                    radius,
                    direction,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_CapsuleCastAll_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        p0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::BlittableArrayWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "Query_CapsuleCastAll_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_CapsuleCastAll_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    p0,
                    p1,
                    radius,
                    direction,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                    ret,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_ClosestPoint(
        collider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        crate::UnityEngine::Vector3,
                    ), crate::UnityEngine::Vector3, 4usize>("Query_ClosestPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_ClosestPoint",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (collider, position, rotation, point))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_ClosestPoint_Injected(
        collider: crate::System::IntPtr,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        point: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Query_ClosestPoint_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_ClosestPoint_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (collider, position, rotation, point, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_ComputePenetration(
        colliderA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        positionA: crate::UnityEngine::Vector3,
        rotationA: crate::UnityEngine::Quaternion,
        colliderB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        positionB: crate::UnityEngine::Vector3,
        rotationB: crate::UnityEngine::Quaternion,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<f32>,
                    ), bool, 8usize>("Query_ComputePenetration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_ComputePenetration",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    colliderA, positionA, rotationA, colliderB, positionB, rotationB, direction,
                    distance,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_ComputePenetration_Injected(
        colliderA: crate::System::IntPtr,
        positionA: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotationA: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        colliderB: crate::System::IntPtr,
        positionB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotationB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<f32>,
                    ), bool, 8usize>("Query_ComputePenetration_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_ComputePenetration_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    colliderA, positionA, rotationA, colliderB, positionB, rotationB, direction,
                    distance,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_SphereCastAll(
        physicsScene: crate::UnityEngine::PhysicsScene,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 7usize>("Query_SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_SphereCastAll",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    origin,
                    radius,
                    direction,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_SphereCastAll_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::BlittableArrayWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Query_SphereCastAll_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Query_SphereCastAll_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    physicsScene,
                    origin,
                    radius,
                    direction,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                    ret,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Ray7(
        ray: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 1usize>("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastAll",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ray))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Ray_f32_6(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray, f32), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 2usize>("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastAll",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ray, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Ray_f32_i32_5(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Ray, f32, i32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit,
                            >,
                        >,
                        3usize,
                    >("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RaycastAll", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ray, maxDistance, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Ray_f32_i32_QueryTriggerInteraction4(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 4usize>("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastAll",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (ray, maxDistance, layerMask, queryTriggerInteraction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Vector3_Vector3_3(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit,
                            >,
                        >,
                        2usize,
                    >("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RaycastAll", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (origin, direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Vector3_Vector3_f32_2(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 3usize>("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastAll",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (origin, direction, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Vector3_Vector3_f32_i32_1(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 4usize>("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastAll",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, direction, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Vector3_Vector3_f32_i32_QueryTriggerInteraction0(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 5usize>("RaycastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastAll",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    origin,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Ray_Il2CppArray3(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                    ), i32, 2usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (ray, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Ray_Il2CppArray_f32_2(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                    ), i32, 3usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, results, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Ray_Il2CppArray_f32_i32_1(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                    ), i32, 4usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, results, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Ray_Il2CppArray_f32_i32_QueryTriggerInteraction0(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 5usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    ray,
                    results,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Vector3_Vector3_Il2CppArray7(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                    ), i32, 3usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (origin, direction, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Vector3_Vector3_Il2CppArray_f32_6(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                    ), i32, 4usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, direction, results, maxDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Vector3_Vector3_Il2CppArray_f32_i32_5(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                    ), i32, 5usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (origin, direction, results, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Vector3_Vector3_Il2CppArray_f32_i32_QueryTriggerInteraction4(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 6usize>("RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RaycastNonAlloc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    origin,
                    direction,
                    results,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray11(ray: crate::UnityEngine::Ray) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray), bool, 1usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ray))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut15(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    ), bool, 2usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ray, hitInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut_f32_14(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                    ), bool, 3usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, hitInfo, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut_f32_i32_13(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                    ), bool, 4usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, hitInfo, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut_f32_i32_QueryTriggerInteraction12(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    ray,
                    hitInfo,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_f32_10(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray, f32), bool, 2usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_f32_i32_9(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray, f32, i32), bool, 3usize>(
                        "Raycast",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, maxDistance, layerMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_f32_i32_QueryTriggerInteraction8(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 4usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (ray, maxDistance, layerMask, queryTriggerInteraction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_3(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                        bool,
                        2usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (origin, direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_ByRefMut7(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    ), bool, 3usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (origin, direction, hitInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_ByRefMut_f32_6(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                    ), bool, 4usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, direction, hitInfo, maxDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_ByRefMut_f32_i32_5(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                    ), bool, 5usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (origin, direction, hitInfo, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_ByRefMut_f32_i32_QueryTriggerInteraction4(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 6usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    origin,
                    direction,
                    hitInfo,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_f32_2(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), bool, 3usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (origin, direction, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_f32_i32_1(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                    ), bool, 4usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, direction, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_f32_i32_QueryTriggerInteraction0(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Raycast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    origin,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RebuildBroadphaseRegions(
        worldBounds: crate::UnityEngine::Bounds,
        subdivisions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Bounds, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RebuildBroadphaseRegions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RebuildBroadphaseRegions", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (worldBounds, subdivisions))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReportContacts(
        array: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::ContactPairHeader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::Unity::Collections::NativeArray_1_ReadOnly<
                        crate::UnityEngine::ContactPairHeader,
                    >), quest_hook::libil2cpp::Void, 1usize>("ReportContacts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReportContacts",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetInterpolationPoses_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::PhysicsScene),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetInterpolationPoses_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetInterpolationPoses_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (physicsScene))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetInterpolationPoses_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::PhysicsScene,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetInterpolationPoses_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetInterpolationPoses_Internal_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (physicsScene))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionEnter(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SendOnCollisionEnter"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendOnCollisionEnter",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (component, collision))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionEnter_Injected(
        component: crate::System::IntPtr,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SendOnCollisionEnter_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendOnCollisionEnter_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (component, collision))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionExit(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SendOnCollisionExit"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendOnCollisionExit",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (component, collision))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionExit_Injected(
        component: crate::System::IntPtr,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SendOnCollisionExit_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendOnCollisionExit_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (component, collision))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionStay(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SendOnCollisionStay"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendOnCollisionStay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (component, collision))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionStay_Injected(
        component: crate::System::IntPtr,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SendOnCollisionStay_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendOnCollisionStay_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (component, collision))? };
        Ok(__cordl_ret.into())
    }
    pub fn Simulate(step: f32) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("Simulate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Simulate",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (step))? };
        Ok(__cordl_ret.into())
    }
    pub fn Simulate_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        step: f32,
        stages: crate::UnityEngine::SimulationStage,
        options: crate::UnityEngine::SimulationOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::PhysicsScene,
                        f32,
                        crate::UnityEngine::SimulationStage,
                        crate::UnityEngine::SimulationOption,
                    ), quest_hook::libil2cpp::Void, 4usize>("Simulate_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Simulate_Internal",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (physicsScene, step, stages, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Simulate_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        step: f32,
        stages: crate::UnityEngine::SimulationStage,
        options: crate::UnityEngine::SimulationOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
                        f32,
                        crate::UnityEngine::SimulationStage,
                        crate::UnityEngine::SimulationOption,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Simulate_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Simulate_Internal_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (physicsScene, step, stages, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Ray7(
        ray: crate::UnityEngine::Ray,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray, f32), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 2usize>("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastAll",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ray, radius))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Ray_f32_6(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Ray, f32, f32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit,
                            >,
                        >,
                        3usize,
                    >("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SphereCastAll", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ray, radius, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Ray_f32_i32_5(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Ray, f32, f32, i32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit,
                            >,
                        >,
                        4usize,
                    >("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SphereCastAll", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, radius, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Ray_f32_i32_QueryTriggerInteraction4(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 5usize>("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastAll",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (ray, radius, maxDistance, layerMask, queryTriggerInteraction),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Vector3_Vector3_3(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 3usize>("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastAll",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (origin, radius, direction))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Vector3_Vector3_f32_2(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 4usize>("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastAll",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, radius, direction, maxDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Vector3_Vector3_f32_i32_1(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 5usize>("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastAll",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (origin, radius, direction, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastAll_Vector3_Vector3_f32_i32_QueryTriggerInteraction0(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                    >, 6usize>("SphereCastAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastAll",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    origin,
                    radius,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Ray_Il2CppArray7(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                    ), i32, 3usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, radius, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Ray_Il2CppArray_f32_6(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                    ), i32, 4usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, radius, results, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Ray_Il2CppArray_f32_i32_5(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                    ), i32, 5usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (ray, radius, results, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Ray_Il2CppArray_f32_i32_QueryTriggerInteraction4(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 6usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    ray,
                    radius,
                    results,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Vector3_Vector3_Il2CppArray3(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                    ), i32, 4usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, radius, direction, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Vector3_Vector3_Il2CppArray_f32_2(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                    ), i32, 5usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (origin, radius, direction, results, maxDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Vector3_Vector3_Il2CppArray_f32_i32_1(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                    ), i32, 6usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (origin, radius, direction, results, maxDistance, layerMask),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Vector3_Vector3_Il2CppArray_f32_i32_QueryTriggerInteraction0(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
                        >,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), i32, 7usize>("SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCastNonAlloc",
                            7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    origin,
                    radius,
                    direction,
                    results,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray7(
        ray: crate::UnityEngine::Ray,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray, f32), bool, 2usize>(
                        "SphereCast",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ray, radius))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_ByRefMut11(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    ), bool, 3usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, radius, hitInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_ByRefMut_f32_10(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                    ), bool, 4usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, radius, hitInfo, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_ByRefMut_f32_i32_9(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                    ), bool, 5usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (ray, radius, hitInfo, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_ByRefMut_f32_i32_QueryTriggerInteraction8(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 6usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    ray,
                    radius,
                    hitInfo,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_f32_6(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray, f32, f32), bool, 3usize>(
                        "SphereCast",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ray, radius, maxDistance))? };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_f32_i32_5(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::Ray, f32, f32, i32), bool, 4usize>(
                        "SphereCast",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, radius, maxDistance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_f32_i32_QueryTriggerInteraction4(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Ray,
                        f32,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 5usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (ray, radius, maxDistance, layerMask, queryTriggerInteraction),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Vector3_Vector3_ByRefMut3(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                    ), bool, 4usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, radius, direction, hitInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Vector3_Vector3_ByRefMut_f32_2(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                    ), bool, 5usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (origin, radius, direction, hitInfo, maxDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Vector3_Vector3_ByRefMut_f32_i32_1(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                    ), bool, 6usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (origin, radius, direction, hitInfo, maxDistance, layerMask),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Vector3_Vector3_ByRefMut_f32_i32_QueryTriggerInteraction0(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
                        f32,
                        i32,
                        crate::UnityEngine::QueryTriggerInteraction,
                    ), bool, 7usize>("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SphereCast",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    origin,
                    radius,
                    direction,
                    hitInfo,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SyncTransforms() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("SyncTransforms")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SyncTransforms",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TranslateTriangleIndexFromID(
        instanceID: i32,
        faceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, u32), u32, 2usize>("TranslateTriangleIndexFromID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TranslateTriangleIndexFromID",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (instanceID, faceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn add_ContactEvent(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Physics_ContactEventDelegate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Physics_ContactEventDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_ContactEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_ContactEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_ContactModifyEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<crate::UnityEngine::ModifiableContactPair>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            crate::UnityEngine::PhysicsScene,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::ModifiableContactPair,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_ContactModifyEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_ContactModifyEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_ContactModifyEventCCD(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<crate::UnityEngine::ModifiableContactPair>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            crate::UnityEngine::PhysicsScene,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::ModifiableContactPair,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_ContactModifyEventCCD"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_ContactModifyEventCCD",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_GenericContactModifyEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                crate::UnityEngine::PhysicsScene,
                crate::System::IntPtr,
                i32,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_4<
                            crate::UnityEngine::PhysicsScene,
                            crate::System::IntPtr,
                            i32,
                            bool,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_GenericContactModifyEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_GenericContactModifyEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_autoSimulation() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_autoSimulation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_autoSimulation",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_autoSyncTransforms() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_autoSyncTransforms")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_autoSyncTransforms",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_bounceThreshold() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_bounceThreshold")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_bounceThreshold",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_bounceTreshold() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_bounceTreshold")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_bounceTreshold",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_clothGravity() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::UnityEngine::Vector3, 0usize>(
                        "get_clothGravity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_clothGravity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_clothGravity_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("get_clothGravity_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_clothGravity_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultContactOffset() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_defaultContactOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_defaultContactOffset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMaxAngularSpeed() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_defaultMaxAngularSpeed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_defaultMaxAngularSpeed",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMaxDepenetrationVelocity() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_defaultMaxDepenetrationVelocity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_defaultMaxDepenetrationVelocity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPhysicsScene(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::UnityEngine::PhysicsScene, 0usize>(
                        "get_defaultPhysicsScene",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_defaultPhysicsScene",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::PhysicsScene =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPhysicsScene_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::PhysicsScene,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("get_defaultPhysicsScene_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_defaultPhysicsScene_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSolverIterations() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_defaultSolverIterations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_defaultSolverIterations",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSolverVelocityIterations() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_defaultSolverVelocityIterations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_defaultSolverVelocityIterations",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_gravity() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::UnityEngine::Vector3, 0usize>("get_gravity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_gravity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_gravity_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("get_gravity_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_gravity_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_improvedPatchFriction() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_improvedPatchFriction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_improvedPatchFriction",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_interCollisionDistance() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_interCollisionDistance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_interCollisionDistance",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_interCollisionSettingsToggle() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_interCollisionSettingsToggle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_interCollisionSettingsToggle",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_interCollisionStiffness() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_interCollisionStiffness")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_interCollisionStiffness",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_invokeCollisionCallbacks() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_invokeCollisionCallbacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_invokeCollisionCallbacks",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxAngularVelocity() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_maxAngularVelocity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_maxAngularVelocity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_minPenetrationForPenalty() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_minPenetrationForPenalty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_minPenetrationForPenalty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_penetrationPenaltyForce() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_penetrationPenaltyForce")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_penetrationPenaltyForce",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_queriesHitBackfaces() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_queriesHitBackfaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_queriesHitBackfaces",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_queriesHitTriggers() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_queriesHitTriggers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_queriesHitTriggers",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_reuseCollisionCallbacks() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_reuseCollisionCallbacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_reuseCollisionCallbacks",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_simulationMode() -> quest_hook::libil2cpp::Result<crate::UnityEngine::SimulationMode>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::UnityEngine::SimulationMode, 0usize>(
                        "get_simulationMode",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_simulationMode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SimulationMode =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sleepAngularVelocity() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_sleepAngularVelocity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_sleepAngularVelocity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sleepThreshold() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_sleepThreshold")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_sleepThreshold",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sleepVelocity() -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f32, 0usize>("get_sleepVelocity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_sleepVelocity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_solverIterationCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_solverIterationCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_solverIterationCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_solverVelocityIterationCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_solverVelocityIterationCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_solverVelocityIterationCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_ContactEvent(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Physics_ContactEventDelegate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Physics_ContactEventDelegate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_ContactEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_ContactEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_ContactModifyEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<crate::UnityEngine::ModifiableContactPair>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            crate::UnityEngine::PhysicsScene,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::ModifiableContactPair,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_ContactModifyEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_ContactModifyEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_ContactModifyEventCCD(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<crate::UnityEngine::ModifiableContactPair>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            crate::UnityEngine::PhysicsScene,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::ModifiableContactPair,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_ContactModifyEventCCD"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_ContactModifyEventCCD",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_GenericContactModifyEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                crate::UnityEngine::PhysicsScene,
                crate::System::IntPtr,
                i32,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_4<
                            crate::UnityEngine::PhysicsScene,
                            crate::System::IntPtr,
                            i32,
                            bool,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_GenericContactModifyEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_GenericContactModifyEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_autoSimulation(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_autoSimulation",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_autoSimulation",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_autoSyncTransforms(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_autoSyncTransforms",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_autoSyncTransforms",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_bounceThreshold(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_bounceThreshold",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_bounceThreshold",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_bounceTreshold(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_bounceTreshold",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_bounceTreshold",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_clothGravity(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_clothGravity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_clothGravity", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_clothGravity_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_clothGravity_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_clothGravity_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultContactOffset(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_defaultContactOffset",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_defaultContactOffset",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultMaxAngularSpeed(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_defaultMaxAngularSpeed",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_defaultMaxAngularSpeed",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultMaxDepenetrationVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_defaultMaxDepenetrationVelocity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_defaultMaxDepenetrationVelocity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultSolverIterations(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_defaultSolverIterations",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_defaultSolverIterations",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultSolverVelocityIterations(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_defaultSolverVelocityIterations",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_defaultSolverVelocityIterations",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_gravity(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_gravity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_gravity", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_gravity_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_gravity_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_gravity_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_improvedPatchFriction(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_improvedPatchFriction",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_improvedPatchFriction",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_interCollisionDistance(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_interCollisionDistance",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_interCollisionDistance",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_interCollisionSettingsToggle(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_interCollisionSettingsToggle",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_interCollisionSettingsToggle",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_interCollisionStiffness(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_interCollisionStiffness",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_interCollisionStiffness",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_invokeCollisionCallbacks(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_invokeCollisionCallbacks",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_invokeCollisionCallbacks",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_maxAngularVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_maxAngularVelocity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_maxAngularVelocity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_minPenetrationForPenalty(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_minPenetrationForPenalty",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_minPenetrationForPenalty",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_penetrationPenaltyForce(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_penetrationPenaltyForce",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_penetrationPenaltyForce",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_queriesHitBackfaces(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_queriesHitBackfaces",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_queriesHitBackfaces",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_queriesHitTriggers(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_queriesHitTriggers",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_queriesHitTriggers",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_reuseCollisionCallbacks(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_reuseCollisionCallbacks",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_reuseCollisionCallbacks",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_simulationMode(
        value: crate::UnityEngine::SimulationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::SimulationMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_simulationMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_simulationMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_sleepAngularVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_sleepAngularVelocity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_sleepAngularVelocity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_sleepThreshold(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_sleepThreshold",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_sleepThreshold",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_sleepVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_sleepVelocity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_sleepVelocity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_solverIterationCount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_solverIterationCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_solverIterationCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_solverVelocityIterationCount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "set_solverVelocityIterationCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_solverVelocityIterationCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Physics")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Physics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Physics+ContactEventDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics_ContactEventDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "cordl_class_UnityEngine+Physics+ContactEventDelegate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Physics_ContactEventDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Physics/ContactEventDelegate";
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
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl std::ops::Deref for crate::UnityEngine::Physics_ContactEventDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl std::ops::DerefMut for crate::UnityEngine::Physics_ContactEventDelegate {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl crate::UnityEngine::Physics_ContactEventDelegate {
    pub fn BeginInvoke(
        &mut self,
        scene: crate::UnityEngine::PhysicsScene,
        headerArray: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::ContactPairHeader,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::Unity::Collections::NativeArray_1_ReadOnly<
                            crate::UnityEngine::ContactPairHeader,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, 4usize>(
                        "BeginInvoke",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginInvoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (scene, headerArray, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        scene: crate::UnityEngine::PhysicsScene,
        headerArray: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::ContactPairHeader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::PhysicsScene,
                        crate::Unity::Collections::NativeArray_1_ReadOnly<
                            crate::UnityEngine::ContactPairHeader,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (scene, headerArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Physics+ContactEventDelegate")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Physics_ContactEventDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
