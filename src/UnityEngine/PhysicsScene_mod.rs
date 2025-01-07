#[cfg(feature = "UnityEngine+PhysicsScene")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsScene {
    pub m_Handle: i32,
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
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
#[cfg(feature = "UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::PhysicsScene {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::PhysicsScene {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
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
#[cfg(feature = "UnityEngine+PhysicsScene")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::PhysicsScene {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BoxCast",
            (center, halfExtents, direction, hitInfo),
        )?;
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BoxCast",
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
        )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BoxCast",
            (center, halfExtents, direction, results),
        )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BoxCast",
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
        )?;
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CapsuleCast",
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
        )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CapsuleCast",
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
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_PhysicsScene1(
        &mut self,
        other: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_BoxCast",
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
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_BoxCastNonAlloc",
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
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_BoxCastNonAlloc_Injected",
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
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_CapsuleCast",
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
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_CapsuleCastNonAlloc",
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
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_CapsuleCastNonAlloc_Injected",
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
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_Raycast",
                (physicsScene, ray, maxDistance, hit, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_RaycastNonAlloc",
                (
                    physicsScene,
                    ray,
                    raycastHits,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_RaycastNonAlloc_Injected",
                (
                    physicsScene,
                    ray,
                    raycastHits,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastTest(
        physicsScene: crate::UnityEngine::PhysicsScene,
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_RaycastTest",
                (physicsScene, ray, maxDistance, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastTest_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_RaycastTest_Injected",
                (physicsScene, ray, maxDistance, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_Raycast_Injected",
                (physicsScene, ray, maxDistance, hit, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_SphereCast",
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
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_SphereCastNonAlloc",
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
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_SphereCastNonAlloc_Injected",
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
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InterpolateBodies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InterpolateBodies",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsEmpty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEmpty_Internal", (physicsScene))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEmpty_Internal_Injected", (physicsScene))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid_Internal", (physicsScene))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid_Internal_Injected", (physicsScene))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapBoxNonAlloc_Internal",
                (
                    physicsScene,
                    center,
                    halfExtents,
                    results,
                    orientation,
                    mask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapBoxNonAlloc_Internal_Injected",
                (
                    physicsScene,
                    center,
                    halfExtents,
                    results,
                    orientation,
                    mask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OverlapBox",
            (
                center,
                halfExtents,
                results,
                orientation,
                layerMask,
                queryTriggerInteraction,
            ),
        )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OverlapBox",
            (center, halfExtents, results),
        )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OverlapCapsule",
            (point0, point1, radius, results, layerMask, queryTriggerInteraction),
        )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapCapsuleNonAlloc_Internal",
                (
                    physicsScene,
                    point0,
                    point1,
                    radius,
                    results,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapCapsuleNonAlloc_Internal_Injected",
                (
                    physicsScene,
                    point0,
                    point1,
                    radius,
                    results,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OverlapSphere",
            (position, radius, results, layerMask, queryTriggerInteraction),
        )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapSphereNonAlloc_Internal",
                (
                    physicsScene,
                    position,
                    radius,
                    results,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapSphereNonAlloc_Internal_Injected",
                (
                    physicsScene,
                    position,
                    radius,
                    results,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_BoxCast",
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
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_BoxCast_Injected",
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
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_CapsuleCast",
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
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_CapsuleCast_Injected",
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
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_SphereCast",
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
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_SphereCast_Injected",
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
            )?;
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, hitInfo, maxDistance, layerMask, queryTriggerInteraction),
        )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (
                origin,
                direction,
                raycastHits,
                maxDistance,
                layerMask,
                queryTriggerInteraction,
            ),
        )?;
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, maxDistance, layerMask, queryTriggerInteraction),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetInterpolationPoses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResetInterpolationPoses",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate(
        &mut self,
        step: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Simulate",
            (step),
        )?;
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SphereCast",
            (
                origin,
                radius,
                direction,
                hitInfo,
                maxDistance,
                layerMask,
                queryTriggerInteraction,
            ),
        )?;
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
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SphereCast",
            (
                origin,
                radius,
                direction,
                results,
                maxDistance,
                layerMask,
                queryTriggerInteraction,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::PhysicsScene,
        rhs: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::PhysicsScene,
        rhs: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene>>
for crate::UnityEngine::PhysicsScene {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene>>
for crate::UnityEngine::PhysicsScene {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene> {
        todo!()
    }
}
