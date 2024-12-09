#[cfg(feature = "UnityEngine+PhysicsScene")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PhysicsScene {
    pub m_Handle: i32,
}
#[cfg(feature = "UnityEngine+PhysicsScene")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PhysicsScene => "UnityEngine"
    ."PhysicsScene"
);
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn BoxCast_Il2CppArray3(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BoxCast",
            (center, halfExtents, direction, results),
        )?;
        Ok(__cordl_ret)
    }
    pub fn BoxCast_Il2CppArray_Quaternion_f32_i32_QueryTriggerInteraction2(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn CapsuleCast_Il2CppArray1(
        &mut self,
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
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
        Ok(__cordl_ret)
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn InterpolateBodies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InterpolateBodies",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsEmpty",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OverlapBox_Quaternion_i32_QueryTriggerInteraction0(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Collider,
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
        Ok(__cordl_ret)
    }
    pub fn OverlapBox_Vector3_Vector3_Il2CppArray1(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Collider,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OverlapBox",
            (center, halfExtents, results),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OverlapCapsule(
        &mut self,
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        results: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Collider,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OverlapCapsule",
            (point0, point1, radius, results, layerMask, queryTriggerInteraction),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OverlapSphere(
        &mut self,
        position: crate::UnityEngine::Vector3,
        radius: f32,
        results: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Collider,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OverlapSphere",
            (position, radius, results, layerMask, queryTriggerInteraction),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Raycast_Il2CppArray_f32_i32_QueryTriggerInteraction2(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        raycastHits: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::RaycastHit,
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ResetInterpolationPoses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResetInterpolationPoses",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SphereCast_Il2CppArray1(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        results: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
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
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
