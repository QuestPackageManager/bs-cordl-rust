#[cfg(feature = "UnityEngine+Physics")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Physics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Physics => "UnityEngine"."Physics"
);
#[cfg(feature = "UnityEngine+Physics")]
impl std::ops::Deref for crate::UnityEngine::Physics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics")]
impl std::ops::DerefMut for crate::UnityEngine::Physics {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BakeMesh", (meshID, convex, cookingOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeMesh_i32__cordl_bool1(
        meshID: i32,
        convex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BakeMesh", (meshID, convex))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoxCastAll", (center, halfExtents, direction, orientation))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCastAll",
                (center, halfExtents, direction, orientation, maxDistance),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCastAll",
                (center, halfExtents, direction, orientation, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCastAll",
                (
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoxCastAll", (center, halfExtents, direction))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCastNonAlloc",
                (center, halfExtents, direction, results, orientation),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCastNonAlloc",
                (center, halfExtents, direction, results, orientation, maxDistance),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCastNonAlloc",
                (
                    center,
                    halfExtents,
                    direction,
                    results,
                    orientation,
                    maxDistance,
                    layerMask,
                ),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCastNonAlloc",
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
    pub fn BoxCastNonAlloc_Vector3_Vector3_Vector3_Il2CppArray4(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoxCastNonAlloc", (center, halfExtents, direction, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut9(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoxCast", (center, halfExtents, direction, hitInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_ByRefMut_Quaternion8(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoxCast", (center, halfExtents, direction, hitInfo, orientation))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCast",
                (center, halfExtents, direction, hitInfo, orientation, maxDistance),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCast",
                (
                    center,
                    halfExtents,
                    direction,
                    hitInfo,
                    orientation,
                    maxDistance,
                    layerMask,
                ),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
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
    pub fn BoxCast_Quaternion3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoxCast", (center, halfExtents, direction, orientation))?;
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Quaternion_f32_2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCast",
                (center, halfExtents, direction, orientation, maxDistance),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCast",
                (center, halfExtents, direction, orientation, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BoxCast",
                (
                    center,
                    halfExtents,
                    direction,
                    orientation,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BoxCast_Vector3_Vector3_Vector3_4(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BoxCast", (center, halfExtents, direction))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CapsuleCastAll", (point1, point2, radius, direction))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CapsuleCastAll", (point1, point2, radius, direction, maxDistance))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCastAll",
                (point1, point2, radius, direction, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCastAll",
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCastNonAlloc",
                (point1, point2, radius, direction, results),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCastNonAlloc",
                (point1, point2, radius, direction, results, maxDistance),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCastNonAlloc",
                (point1, point2, radius, direction, results, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCastNonAlloc",
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
    pub fn CapsuleCast_ByRefMut7(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CapsuleCast", (point1, point2, radius, direction, hitInfo))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCast",
                (point1, point2, radius, direction, hitInfo, maxDistance),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCast",
                (point1, point2, radius, direction, hitInfo, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
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
    pub fn CapsuleCast_Vector3_Vector3_f32_Vector3_3(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CapsuleCast", (point1, point2, radius, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn CapsuleCast_f32_2(
        point1: crate::UnityEngine::Vector3,
        point2: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CapsuleCast", (point1, point2, radius, direction, maxDistance))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCast",
                (point1, point2, radius, direction, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CapsuleCast",
                (
                    point1,
                    point2,
                    radius,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckBox_Internal",
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layermask,
                    queryTriggerInteraction,
                ),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckBox_Internal_Injected",
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layermask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Quaternion2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckBox", (center, halfExtents, orientation))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Quaternion_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckBox", (center, halfExtents, orientation, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Quaternion_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layermask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckBox",
                (center, halfExtents, orientation, layermask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckBox_Vector3_Vector3_3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckBox", (center, halfExtents))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckCapsule_Internal",
                (physicsScene, start, end, radius, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckCapsule_Internal_Injected",
                (physicsScene, start, end, radius, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_Vector3_Vector3_f32_2(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckCapsule", (start, end, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_i32_1(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckCapsule", (start, end, radius, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCapsule_i32_QueryTriggerInteraction0(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckCapsule",
                (start, end, radius, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckSphere_Internal",
                (physicsScene, position, radius, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckSphere_Internal_Injected",
                (physicsScene, position, radius, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_Vector3_f32_2(
        position: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckSphere", (position, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_i32_1(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckSphere", (position, radius, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckSphere_i32_QueryTriggerInteraction0(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckSphere",
                (position, radius, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClosestPoint(
        point: crate::UnityEngine::Vector3,
        collider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClosestPoint", (point, collider, position, rotation))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ComputePenetration",
                (
                    colliderA,
                    positionA,
                    rotationA,
                    colliderB,
                    positionB,
                    rotationB,
                    direction,
                    distance,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActorAngularVelocity(
        actorPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActorAngularVelocity", (actorPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActorAngularVelocity_Injected(
        actorPtr: crate::System::IntPtr,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActorAngularVelocity_Injected", (actorPtr, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActorLinearVelocity(
        actorPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActorLinearVelocity", (actorPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActorLinearVelocity_Injected(
        actorPtr: crate::System::IntPtr,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActorLinearVelocity_Injected", (actorPtr, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBodyByInstanceID(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBodyByInstanceID", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColliderByInstanceID(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetColliderByInstanceID", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCollisionToReport(
        header: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPairHeader>,
        pair: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
        flipped: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCollisionToReport", (header, pair, flipped))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIgnoreCollision(
        collider1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        collider2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIgnoreCollision", (collider1, collider2))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIgnoreLayerCollision(
        layer1: i32,
        layer2: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIgnoreLayerCollision", (layer1, layer2))?;
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreCollision_Collider_Collider1(
        collider1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        collider2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IgnoreCollision", (collider1, collider2))?;
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreCollision__cordl_bool0(
        collider1: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        collider2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        ignore: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IgnoreCollision", (collider1, collider2, ignore))?;
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreLayerCollision__cordl_bool0(
        layer1: i32,
        layer2: i32,
        ignore: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IgnoreLayerCollision", (layer1, layer2, ignore))?;
        Ok(__cordl_ret.into())
    }
    pub fn IgnoreLayerCollision_i32_i32_1(
        layer1: i32,
        layer2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IgnoreLayerCollision", (layer1, layer2))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_BoxCastAll",
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
            )?;
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_BoxCastAll_Injected",
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
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_RaycastAll",
                (physicsScene, ray, maxDistance, mask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RaycastAll_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        maxDistance: f32,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_RaycastAll_Injected",
                (physicsScene, ray, maxDistance, mask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RebuildBroadphaseRegions(
        bounds: crate::UnityEngine::Bounds,
        subdivisions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_RebuildBroadphaseRegions", (bounds, subdivisions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RebuildBroadphaseRegions_Injected(
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        subdivisions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_RebuildBroadphaseRegions_Injected",
                (bounds, subdivisions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InterpolateBodies_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InterpolateBodies_Internal", (physicsScene))?;
        Ok(__cordl_ret.into())
    }
    pub fn InterpolateBodies_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InterpolateBodies_Internal_Injected", (physicsScene))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsShapeTrigger(
        shapePtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsShapeTrigger", (shapePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_ByRefMut5(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Linecast", (start, end, hitInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_ByRefMut_i32_4(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Linecast", (start, end, hitInfo, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_ByRefMut_i32_QueryTriggerInteraction3(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Linecast",
                (start, end, hitInfo, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_Vector3_Vector3_2(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Linecast", (start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_i32_1(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Linecast", (start, end, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Linecast_i32_QueryTriggerInteraction0(
        start: crate::UnityEngine::Vector3,
        end: crate::UnityEngine::Vector3,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Linecast", (start, end, layerMask, queryTriggerInteraction))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnSceneContact(
        scene: crate::UnityEngine::PhysicsScene,
        buffer: crate::System::IntPtr,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSceneContact", (scene, buffer, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSceneContactModify(
        scene: crate::UnityEngine::PhysicsScene,
        buffer: crate::System::IntPtr,
        count: i32,
        isCCD: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSceneContactModify", (scene, buffer, count, isCCD))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Quaternion2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapBoxNonAlloc", (center, halfExtents, results, orientation))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Quaternion_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
        orientation: crate::UnityEngine::Quaternion,
        mask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapBoxNonAlloc",
                (center, halfExtents, results, orientation, mask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBoxNonAlloc_Quaternion_i32_QueryTriggerInteraction0(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
        orientation: crate::UnityEngine::Quaternion,
        mask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapBoxNonAlloc",
                (
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
    pub fn OverlapBoxNonAlloc_Vector3_Vector3_Il2CppArray3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapBoxNonAlloc", (center, halfExtents, results))?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapBox_Internal",
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapBox_Internal_Injected",
                (
                    physicsScene,
                    center,
                    halfExtents,
                    orientation,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Quaternion2(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapBox", (center, halfExtents, orientation))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Quaternion_i32_1(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapBox", (center, halfExtents, orientation, layerMask))?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapBox",
                (center, halfExtents, orientation, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapBox_Vector3_Vector3_3(
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapBox", (center, halfExtents))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsuleNonAlloc_Vector3_Vector3_f32_Il2CppArray2(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapCapsuleNonAlloc", (point0, point1, radius, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsuleNonAlloc_i32_1(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapCapsuleNonAlloc",
                (point0, point1, radius, results, layerMask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsuleNonAlloc_i32_QueryTriggerInteraction0(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapCapsuleNonAlloc",
                (point0, point1, radius, results, layerMask, queryTriggerInteraction),
            )?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapCapsule_Internal",
                (
                    physicsScene,
                    point0,
                    point1,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapCapsule_Internal_Injected",
                (
                    physicsScene,
                    point0,
                    point1,
                    radius,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule_Vector3_Vector3_f32_2(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapCapsule", (point0, point1, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapCapsule_i32_1(
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapCapsule", (point0, point1, radius, layerMask))?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapCapsule",
                (point0, point1, radius, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphereNonAlloc_Vector3_f32_Il2CppArray2(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapSphereNonAlloc", (position, radius, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphereNonAlloc_i32_1(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapSphereNonAlloc", (position, radius, results, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphereNonAlloc_i32_QueryTriggerInteraction0(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapSphereNonAlloc",
                (position, radius, results, layerMask, queryTriggerInteraction),
            )?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapSphere_Internal",
                (physicsScene, position, radius, layerMask, queryTriggerInteraction),
            )?;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapSphere_Internal_Injected",
                (physicsScene, position, radius, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_Vector3_f32_2(
        position: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapSphere", (position, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_i32_1(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverlapSphere", (position, radius, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverlapSphere_i32_QueryTriggerInteraction0(
        position: crate::UnityEngine::Vector3,
        radius: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverlapSphere",
                (position, radius, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_CapsuleCastAll",
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
            )?;
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_CapsuleCastAll_Injected",
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
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Query_ClosestPoint(
        collider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Query_ClosestPoint", (collider, position, rotation, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn Query_ClosestPoint_Injected(
        collider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        point: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_ClosestPoint_Injected",
                (collider, position, rotation, point, ret),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_ComputePenetration",
                (
                    colliderA,
                    positionA,
                    rotationA,
                    colliderB,
                    positionB,
                    rotationB,
                    direction,
                    distance,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Query_ComputePenetration_Injected(
        colliderA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        positionA: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotationA: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        colliderB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        positionB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotationB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_ComputePenetration_Injected",
                (
                    colliderA,
                    positionA,
                    rotationA,
                    colliderB,
                    positionB,
                    rotationB,
                    direction,
                    distance,
                ),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_SphereCastAll",
                (
                    physicsScene,
                    origin,
                    radius,
                    direction,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                ),
            )?;
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Query_SphereCastAll_Injected",
                (
                    physicsScene,
                    origin,
                    radius,
                    direction,
                    maxDistance,
                    mask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastAll_Ray7(
        ray: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("RaycastAll", (ray))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastAll", (ray, maxDistance))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastAll", (ray, maxDistance, layerMask))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastAll",
                (ray, maxDistance, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastAll", (origin, direction))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastAll", (origin, direction, maxDistance))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastAll", (origin, direction, maxDistance, layerMask))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastAll",
                (origin, direction, maxDistance, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Ray_Il2CppArray3(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastNonAlloc", (ray, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Ray_Il2CppArray_f32_2(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastNonAlloc", (ray, results, maxDistance))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastNonAlloc", (ray, results, maxDistance, layerMask))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastNonAlloc",
                (ray, results, maxDistance, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastNonAlloc_Vector3_Vector3_Il2CppArray7(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastNonAlloc", (origin, direction, results))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastNonAlloc", (origin, direction, results, maxDistance))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastNonAlloc",
                (origin, direction, results, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastNonAlloc",
                (
                    origin,
                    direction,
                    results,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray11(
        ray: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (ray))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut15(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (ray, hitInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut_f32_14(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (ray, hitInfo, maxDistance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut_f32_i32_13(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (ray, hitInfo, maxDistance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_ByRefMut_f32_i32_QueryTriggerInteraction12(
        ray: crate::UnityEngine::Ray,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Raycast",
                (ray, hitInfo, maxDistance, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_f32_10(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (ray, maxDistance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_f32_i32_9(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (ray, maxDistance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Ray_f32_i32_QueryTriggerInteraction8(
        ray: crate::UnityEngine::Ray,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (ray, maxDistance, layerMask, queryTriggerInteraction))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_3(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_ByRefMut7(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, hitInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_ByRefMut_f32_6(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, hitInfo, maxDistance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_ByRefMut_f32_i32_5(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, hitInfo, maxDistance, layerMask))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Raycast",
                (
                    origin,
                    direction,
                    hitInfo,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_f32_2(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, maxDistance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_f32_i32_1(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, maxDistance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector3_Vector3_f32_i32_QueryTriggerInteraction0(
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Raycast",
                (origin, direction, maxDistance, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RebuildBroadphaseRegions(
        worldBounds: crate::UnityEngine::Bounds,
        subdivisions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildBroadphaseRegions", (worldBounds, subdivisions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReportContacts(
        array: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::ContactPairHeader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReportContacts", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetInterpolationPoses_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetInterpolationPoses_Internal", (physicsScene))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetInterpolationPoses_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetInterpolationPoses_Internal_Injected", (physicsScene))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveActorToComponent(
        actorPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveActorToComponent", (actorPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveActorToInstanceID(
        actorPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveActorToInstanceID", (actorPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveShapeToCollider(
        shapePtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveShapeToCollider", (shapePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveShapeToInstanceID(
        shapePtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveShapeToInstanceID", (shapePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionEnter(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendOnCollisionEnter", (component, collision))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionExit(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendOnCollisionExit", (component, collision))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendOnCollisionStay(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
        collision: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collision>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendOnCollisionStay", (component, collision))?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate(
        step: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Simulate", (step))?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene,
        step: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Simulate_Internal", (physicsScene, step))?;
        Ok(__cordl_ret.into())
    }
    pub fn Simulate_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
        step: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Simulate_Internal_Injected", (physicsScene, step))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastAll", (ray, radius))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastAll", (ray, radius, maxDistance))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastAll", (ray, radius, maxDistance, layerMask))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastAll",
                (ray, radius, maxDistance, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastAll", (origin, radius, direction))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastAll", (origin, radius, direction, maxDistance))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastAll",
                (origin, radius, direction, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastAll",
                (
                    origin,
                    radius,
                    direction,
                    maxDistance,
                    layerMask,
                    queryTriggerInteraction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCastNonAlloc_Ray_Il2CppArray7(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastNonAlloc", (ray, radius, results))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastNonAlloc", (ray, radius, results, maxDistance))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastNonAlloc",
                (ray, radius, results, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastNonAlloc",
                (ray, radius, results, maxDistance, layerMask, queryTriggerInteraction),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCastNonAlloc", (origin, radius, direction, results))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastNonAlloc",
                (origin, radius, direction, results, maxDistance),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastNonAlloc",
                (origin, radius, direction, results, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCastNonAlloc",
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
    pub fn SphereCast_Ray7(
        ray: crate::UnityEngine::Ray,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (ray, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_ByRefMut11(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (ray, radius, hitInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_ByRefMut_f32_10(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (ray, radius, hitInfo, maxDistance))?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_ByRefMut_f32_i32_9(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (ray, radius, hitInfo, maxDistance, layerMask))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCast",
                (ray, radius, hitInfo, maxDistance, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_f32_6(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (ray, radius, maxDistance))?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_f32_i32_5(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (ray, radius, maxDistance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Ray_f32_i32_QueryTriggerInteraction4(
        ray: crate::UnityEngine::Ray,
        radius: f32,
        maxDistance: f32,
        layerMask: i32,
        queryTriggerInteraction: crate::UnityEngine::QueryTriggerInteraction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCast",
                (ray, radius, maxDistance, layerMask, queryTriggerInteraction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Vector3_Vector3_ByRefMut3(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (origin, radius, direction, hitInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SphereCast_Vector3_Vector3_ByRefMut_f32_2(
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        hitInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit>,
        maxDistance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphereCast", (origin, radius, direction, hitInfo, maxDistance))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SphereCast",
                (origin, radius, direction, hitInfo, maxDistance, layerMask),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
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
    pub fn SyncTransforms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SyncTransforms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateTriangleIndex(
        shapePtr: crate::System::IntPtr,
        rawIndex: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateTriangleIndex", (shapePtr, rawIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateTriangleIndexFromID(
        instanceID: i32,
        faceIndex: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateTriangleIndexFromID", (instanceID, faceIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_ContactEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Physics_ContactEventDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_ContactEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_ContactModifyEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<
                    crate::UnityEngine::ModifiableContactPair,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_ContactModifyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_ContactModifyEventCCD(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<
                    crate::UnityEngine::ModifiableContactPair,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_ContactModifyEventCCD", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_autoSimulation() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_autoSimulation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_autoSyncTransforms() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_autoSyncTransforms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bounceThreshold() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bounceThreshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bounceTreshold() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bounceTreshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clothGravity() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector3,
    > {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_clothGravity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clothGravity_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_clothGravity_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultContactOffset() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultContactOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMaxAngularSpeed() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultMaxAngularSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMaxDepenetrationVelocity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultMaxDepenetrationVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPhysicsScene() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::PhysicsScene,
    > {
        let __cordl_ret: crate::UnityEngine::PhysicsScene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultPhysicsScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPhysicsScene_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultPhysicsScene_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSolverIterations() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultSolverIterations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSolverVelocityIterations() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultSolverVelocityIterations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gravity() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_gravity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gravity_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_gravity_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_improvedPatchFriction() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_improvedPatchFriction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interCollisionDistance() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_interCollisionDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interCollisionSettingsToggle() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_interCollisionSettingsToggle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interCollisionStiffness() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_interCollisionStiffness", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_invokeCollisionCallbacks() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_invokeCollisionCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxAngularVelocity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxAngularVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minPenetrationForPenalty() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_minPenetrationForPenalty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_penetrationPenaltyForce() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_penetrationPenaltyForce", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_queriesHitBackfaces() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_queriesHitBackfaces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_queriesHitTriggers() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_queriesHitTriggers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reuseCollisionCallbacks() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_reuseCollisionCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_simulationMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SimulationMode,
    > {
        let __cordl_ret: crate::UnityEngine::SimulationMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_simulationMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sleepAngularVelocity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sleepAngularVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sleepThreshold() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sleepThreshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sleepVelocity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sleepVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_solverIterationCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_solverIterationCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_solverVelocityIterationCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_solverVelocityIterationCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_ContactEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Physics_ContactEventDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_ContactEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_ContactModifyEvent(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<
                    crate::UnityEngine::ModifiableContactPair,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_ContactModifyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_ContactModifyEventCCD(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::PhysicsScene,
                crate::Unity::Collections::NativeArray_1<
                    crate::UnityEngine::ModifiableContactPair,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_ContactModifyEventCCD", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_autoSimulation(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_autoSimulation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_autoSyncTransforms(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_autoSyncTransforms", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bounceThreshold(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_bounceThreshold", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bounceTreshold(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_bounceTreshold", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clothGravity(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_clothGravity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clothGravity_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_clothGravity_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultContactOffset(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultContactOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultMaxAngularSpeed(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultMaxAngularSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultMaxDepenetrationVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultMaxDepenetrationVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultSolverIterations(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultSolverIterations", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultSolverVelocityIterations(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultSolverVelocityIterations", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gravity(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_gravity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gravity_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_gravity_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_improvedPatchFriction(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_improvedPatchFriction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_interCollisionDistance(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_interCollisionDistance", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_interCollisionSettingsToggle(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_interCollisionSettingsToggle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_interCollisionStiffness(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_interCollisionStiffness", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_invokeCollisionCallbacks(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_invokeCollisionCallbacks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxAngularVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_maxAngularVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minPenetrationForPenalty(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_minPenetrationForPenalty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_penetrationPenaltyForce(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_penetrationPenaltyForce", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_queriesHitBackfaces(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_queriesHitBackfaces", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_queriesHitTriggers(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_queriesHitTriggers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_reuseCollisionCallbacks(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_reuseCollisionCallbacks", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_simulationMode(
        value: crate::UnityEngine::SimulationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_simulationMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sleepAngularVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_sleepAngularVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sleepThreshold(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_sleepThreshold", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sleepVelocity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_sleepVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_solverIterationCount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_solverIterationCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_solverVelocityIterationCount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_solverVelocityIterationCount", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Physics")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Physics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics_ContactEventDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Physics_ContactEventDelegate =>
    "UnityEngine"."Physics/ContactEventDelegate"
);
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl std::ops::Deref for crate::UnityEngine::Physics_ContactEventDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl std::ops::DerefMut for crate::UnityEngine::Physics_ContactEventDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (scene, headerArray, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        scene: crate::UnityEngine::PhysicsScene,
        headerArray: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::ContactPairHeader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (scene, headerArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Physics+ContactEventDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Physics_ContactEventDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
