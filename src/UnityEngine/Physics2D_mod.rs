#[cfg(feature = "cordl_class_UnityEngine+Physics2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics2D {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Physics2D")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Physics2D {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Physics2D";
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
#[cfg(feature = "cordl_class_UnityEngine+Physics2D")]
impl std::ops::Deref for crate::UnityEngine::Physics2D {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Physics2D")]
impl std::ops::DerefMut for crate::UnityEngine::Physics2D {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics2D")]
impl crate::UnityEngine::Physics2D {
    pub fn GetRayIntersection(
        ray: crate::UnityEngine::Ray,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Ray, f32, i32),
                        crate::UnityEngine::RaycastHit2D,
                        3usize,
                    >("GetRayIntersection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersection", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, distance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionAll_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene2D,
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene2D,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit2D,
                            >,
                        >,
                        5usize,
                    >("GetRayIntersectionAll_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionAll_Internal", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (physicsScene, origin, direction, distance, layerMask),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionAll_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::PhysicsScene2D,
        >,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene2D,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit2D,
                            >,
                        >,
                        5usize,
                    >("GetRayIntersectionAll_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionAll_Internal_Injected", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (physicsScene, origin, direction, distance, layerMask),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionAll_Ray0(
        ray: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Ray),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit2D,
                            >,
                        >,
                        1usize,
                    >("GetRayIntersectionAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionAll", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ray))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionAll_f32_1(
        ray: crate::UnityEngine::Ray,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Ray, f32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit2D,
                            >,
                        >,
                        2usize,
                    >("GetRayIntersectionAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionAll", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ray, distance))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionAll_f32_i32_2(
        ray: crate::UnityEngine::Ray,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Ray, f32, i32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::RaycastHit2D,
                            >,
                        >,
                        3usize,
                    >("GetRayIntersectionAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionAll", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, distance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionNonAlloc_Ray_Il2CppArray0(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Ray,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit2D,
                                >,
                            >,
                        ),
                        i32,
                        2usize,
                    >("GetRayIntersectionNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionNonAlloc", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionNonAlloc_f32_1(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Ray,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit2D,
                                >,
                            >,
                            f32,
                        ),
                        i32,
                        3usize,
                    >("GetRayIntersectionNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionNonAlloc", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, results, distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionNonAlloc_f32_i32_2(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Ray,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit2D,
                                >,
                            >,
                            f32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("GetRayIntersectionNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRayIntersectionNonAlloc", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (ray, results, distance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_ContactFilter2D_Il2CppArray5(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::ContactFilter2D,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit2D,
                                >,
                            >,
                        ),
                        i32,
                        4usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (origin, direction, contactFilter, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_ContactFilter2D_Il2CppArray_f32_6(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::ContactFilter2D,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit2D,
                                >,
                            >,
                            f32,
                        ),
                        i32,
                        5usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (origin, direction, contactFilter, results, distance),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_ContactFilter2D_List_1_f32_7(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::RaycastHit2D>,
        >,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::ContactFilter2D,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::RaycastHit2D,
                                >,
                            >,
                            f32,
                        ),
                        i32,
                        5usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (origin, direction, contactFilter, results, distance),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector2_Vector2_0(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2),
                        crate::UnityEngine::RaycastHit2D,
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
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, direction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_1(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2, f32),
                        crate::UnityEngine::RaycastHit2D,
                        3usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            cordl_method_info.invoke_unchecked((), (origin, direction, distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_i32_2(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            f32,
                            i32,
                        ),
                        crate::UnityEngine::RaycastHit2D,
                        4usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            cordl_method_info
                .invoke_unchecked((), (origin, direction, distance, layerMask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_i32_f32_3(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        layerMask: i32,
        minDepth: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            f32,
                            i32,
                            f32,
                        ),
                        crate::UnityEngine::RaycastHit2D,
                        5usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (origin, direction, distance, layerMask, minDepth),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_i32_f32_f32_4(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        layerMask: i32,
        minDepth: f32,
        maxDepth: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            f32,
                            i32,
                            f32,
                            f32,
                        ),
                        crate::UnityEngine::RaycastHit2D,
                        6usize,
                    >("Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Raycast",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (origin, direction, distance, layerMask, minDepth, maxDepth),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPhysicsScene() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::PhysicsScene2D,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::PhysicsScene2D,
                        0usize,
                    >("get_defaultPhysicsScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_defaultPhysicsScene", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::PhysicsScene2D = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_queriesHitTriggers() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_queriesHitTriggers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_queriesHitTriggers", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Physics2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Physics2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
