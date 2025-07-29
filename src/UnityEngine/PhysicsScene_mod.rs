#[cfg(feature = "cordl_class_UnityEngine+PhysicsScene")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsScene {
    pub m_Handle: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PhysicsScene {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "PhysicsScene";
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
#[cfg(feature = "cordl_class_UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::PhysicsScene {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::PhysicsScene {
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
#[cfg(feature = "cordl_class_UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::PhysicsScene {
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
#[cfg(feature = "cordl_class_UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::PhysicsScene {
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
#[cfg(feature = "cordl_class_UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::PhysicsScene {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
impl crate::UnityEngine::PhysicsScene {
    pub fn BoxCast_ByRefMut1(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                        ),
                        bool,
                        4usize,
                    >("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "BoxCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (center, halfExtents, direction, hitInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut_Quaternion_f32_i32_QueryTriggerInteraction0(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            crate::UnityEngine::Quaternion,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        8usize,
                    >("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "BoxCast",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
    pub fn BoxCast_Il2CppArray3(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                        ),
                        i32,
                        4usize,
                    >("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "BoxCast",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (center, halfExtents, direction, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Il2CppArray_Quaternion_f32_i32_QueryTriggerInteraction2(
        &mut self,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            crate::UnityEngine::Quaternion,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        8usize,
                    >("BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "BoxCast",
                            8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
    pub fn CapsuleCast_ByRefMut0(
        &mut self,
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        8usize,
                    >("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CapsuleCast", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
    pub fn CapsuleCast_Il2CppArray1(
        &mut self,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        8usize,
                    >("CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CapsuleCast", 8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_PhysicsScene1(
        &mut self,
        other: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::PhysicsScene),
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
    pub fn Internal_BoxCast(
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        9usize,
                    >("Internal_BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_BoxCast", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        center,
                        halfExtents,
                        orientation,
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
    pub fn Internal_BoxCastNonAlloc(
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            crate::UnityEngine::Quaternion,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        9usize,
                    >("Internal_BoxCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_BoxCastNonAlloc", 9usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        center,
                        halfExtents,
                        direction,
                        raycastHits,
                        orientation,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BoxCastNonAlloc_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        halfExtents: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        orientation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Quaternion,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        9usize,
                    >("Internal_BoxCastNonAlloc_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_BoxCastNonAlloc_Injected", 9usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        center,
                        halfExtents,
                        direction,
                        raycastHits,
                        orientation,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CapsuleCast(
        physicsScene: crate::UnityEngine::PhysicsScene,
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        9usize,
                    >("Internal_CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_CapsuleCast", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn Internal_CapsuleCastNonAlloc(
        physicsScene: crate::UnityEngine::PhysicsScene,
        p0: crate::UnityEngine::Vector3,
        p1: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        9usize,
                    >("Internal_CapsuleCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_CapsuleCastNonAlloc", 9usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        p0,
                        p1,
                        radius,
                        direction,
                        raycastHits,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CapsuleCastNonAlloc_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        p0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        9usize,
                    >("Internal_CapsuleCastNonAlloc_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_CapsuleCastNonAlloc_Injected", 9usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        p0,
                        p1,
                        radius,
                        direction,
                        raycastHits,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Raycast(
        physicsScene: crate::UnityEngine::PhysicsScene,
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Ray,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        6usize,
                    >("Internal_Raycast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_Raycast", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        ray,
                        maxDistance,
                        hit,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastNonAlloc(
        physicsScene: crate::UnityEngine::PhysicsScene,
        ray: crate::UnityEngine::Ray,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Ray,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        6usize,
                    >("Internal_RaycastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_RaycastNonAlloc", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        ray,
                        raycastHits,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastNonAlloc_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        6usize,
                    >("Internal_RaycastNonAlloc_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_RaycastNonAlloc_Injected", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        ray,
                        raycastHits,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastTest(
        physicsScene: crate::UnityEngine::PhysicsScene,
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Ray,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        5usize,
                    >("Internal_RaycastTest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_RaycastTest", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (physicsScene, ray, maxDistance, layerMask, queryTriggerInteraction),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastTest_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        5usize,
                    >("Internal_RaycastTest_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_RaycastTest_Injected", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (physicsScene, ray, maxDistance, layerMask, queryTriggerInteraction),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Raycast_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        maxDistance: f32,
        hit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        6usize,
                    >("Internal_Raycast_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_Raycast_Injected", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        ray,
                        maxDistance,
                        hit,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SphereCast(
        physicsScene: crate::UnityEngine::PhysicsScene,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        8usize,
                    >("Internal_SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SphereCast", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn Internal_SphereCastNonAlloc(
        physicsScene: crate::UnityEngine::PhysicsScene,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        8usize,
                    >("Internal_SphereCastNonAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SphereCastNonAlloc", 8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        origin,
                        radius,
                        direction,
                        raycastHits,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SphereCastNonAlloc_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        8usize,
                    >("Internal_SphereCastNonAlloc_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SphereCastNonAlloc_Injected", 8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        origin,
                        radius,
                        direction,
                        raycastHits,
                        maxDistance,
                        mask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InterpolateBodies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("InterpolateBodies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InterpolateBodies", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
    pub fn IsEmpty_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::PhysicsScene),
                        bool,
                        1usize,
                    >("IsEmpty_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsEmpty_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (physicsScene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::PhysicsScene,
                        >),
                        bool,
                        1usize,
                    >("IsEmpty_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsEmpty_Internal_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (physicsScene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsValid",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::PhysicsScene),
                        bool,
                        1usize,
                    >("IsValid_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValid_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (physicsScene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::PhysicsScene,
                        >),
                        bool,
                        1usize,
                    >("IsValid_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValid_Internal_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (physicsScene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
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
                        ),
                        i32,
                        7usize,
                    >("OverlapBoxNonAlloc_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapBoxNonAlloc_Internal", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn OverlapBoxNonAlloc_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        halfExtents: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        orientation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Quaternion,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        7usize,
                    >("OverlapBoxNonAlloc_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapBoxNonAlloc_Internal_Injected", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn OverlapBox_Quaternion_i32_QueryTriggerInteraction0(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        orientation: crate::UnityEngine::Quaternion,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
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
                        ),
                        i32,
                        6usize,
                    >("OverlapBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapBox", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        center,
                        halfExtents,
                        results,
                        orientation,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Vector3_Vector3_Il2CppArray1(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                                >,
                            >,
                        ),
                        i32,
                        3usize,
                    >("OverlapBox")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapBox", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (center, halfExtents, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule(
        &mut self,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
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
                        ),
                        i32,
                        6usize,
                    >("OverlapCapsule")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapCapsule", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (point0, point1, radius, results, layerMask, queryTriggerInteraction),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsuleNonAlloc_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
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
                        ),
                        i32,
                        7usize,
                    >("OverlapCapsuleNonAlloc_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapCapsuleNonAlloc_Internal", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn OverlapCapsuleNonAlloc_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        point0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        point1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                                >,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        7usize,
                    >("OverlapCapsuleNonAlloc_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapCapsuleNonAlloc_Internal_Injected", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn OverlapSphere(
        &mut self,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                                >,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        5usize,
                    >("OverlapSphere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapSphere", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (position, radius, results, layerMask, queryTriggerInteraction),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphereNonAlloc_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                                >,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        6usize,
                    >("OverlapSphereNonAlloc_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapSphereNonAlloc_Internal", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn OverlapSphereNonAlloc_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
            >,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                                >,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        6usize,
                    >("OverlapSphereNonAlloc_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OverlapSphereNonAlloc_Internal_Injected", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
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
    pub fn Query_BoxCast(
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
        outHit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Quaternion,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        9usize,
                    >("Query_BoxCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Query_BoxCast", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        center,
                        halfExtents,
                        direction,
                        orientation,
                        maxDistance,
                        outHit,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_BoxCast_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        halfExtents: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        orientation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        maxDistance: f32,
        outHit: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Quaternion,
                            >,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        9usize,
                    >("Query_BoxCast_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Query_BoxCast_Injected", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        center,
                        halfExtents,
                        direction,
                        orientation,
                        maxDistance,
                        outHit,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_CapsuleCast(
        physicsScene: crate::UnityEngine::PhysicsScene,
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        9usize,
                    >("Query_CapsuleCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Query_CapsuleCast", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        point1,
                        point2,
                        radius,
                        direction,
                        maxDistance,
                        hitInfo,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_CapsuleCast_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        point1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        point2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        maxDistance: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        9usize,
                    >("Query_CapsuleCast_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Query_CapsuleCast_Injected", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        point1,
                        point2,
                        radius,
                        direction,
                        maxDistance,
                        hitInfo,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_SphereCast(
        physicsScene: crate::UnityEngine::PhysicsScene,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        8usize,
                    >("Query_SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Query_SphereCast", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        origin,
                        radius,
                        direction,
                        maxDistance,
                        hitInfo,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Query_SphereCast_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        maxDistance: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        8usize,
                    >("Query_SphereCast_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Query_SphereCast_Injected", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        physicsScene,
                        origin,
                        radius,
                        direction,
                        maxDistance,
                        hitInfo,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_ByRefMut_f32_i32_QueryTriggerInteraction1(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
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
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
    pub fn Raycast_Il2CppArray_f32_i32_QueryTriggerInteraction2(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        raycastHits: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
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
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        origin,
                        direction,
                        raycastHits,
                        maxDistance,
                        layerMask,
                        queryTriggerInteraction,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_i32_QueryTriggerInteraction0(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
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
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (origin, direction, maxDistance, layerMask, queryTriggerInteraction),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetInterpolationPoses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ResetInterpolationPoses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetInterpolationPoses", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Simulate(
        &mut self,
        step: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Simulate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Simulate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (step))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_ByRefMut0(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RaycastHit,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        bool,
                        7usize,
                    >("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SphereCast", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
    pub fn SphereCast_Il2CppArray1(
        &mut self,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            f32,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::RaycastHit,
                                >,
                            >,
                            f32,
                            i32,
                            crate::UnityEngine::QueryTriggerInteraction,
                        ),
                        i32,
                        7usize,
                    >("SphereCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SphereCast", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::PhysicsScene,
        rhs: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::PhysicsScene,
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
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::PhysicsScene,
        rhs: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::PhysicsScene,
                            crate::UnityEngine::PhysicsScene,
                        ),
                        bool,
                        2usize,
                    >("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "op_Inequality", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::PhysicsScene>>
for crate::UnityEngine::PhysicsScene {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::PhysicsScene> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::PhysicsScene>>
for crate::UnityEngine::PhysicsScene {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::PhysicsScene> {
        todo!()
    }
}
