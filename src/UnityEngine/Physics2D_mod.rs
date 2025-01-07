#[cfg(feature = "UnityEngine+Physics2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics2D {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Physics2D")]
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
#[cfg(feature = "UnityEngine+Physics2D")]
impl std::ops::Deref for crate::UnityEngine::Physics2D {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics2D")]
impl std::ops::DerefMut for crate::UnityEngine::Physics2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRayIntersection", (ray, distance, layerMask))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRayIntersectionAll_Internal",
                (physicsScene, origin, direction, distance, layerMask),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRayIntersectionAll_Internal_Injected",
                (physicsScene, origin, direction, distance, layerMask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionAll_Ray0(
        ray: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRayIntersectionAll", (ray))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRayIntersectionAll", (ray, distance))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRayIntersectionAll", (ray, distance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionNonAlloc_Ray_Il2CppArray0(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRayIntersectionNonAlloc", (ray, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionNonAlloc_f32_1(
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRayIntersectionNonAlloc", (ray, results, distance))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRayIntersectionNonAlloc", (ray, results, distance, layerMask))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, contactFilter, results))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, contactFilter, results, distance))?;
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
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, contactFilter, results, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Vector2_Vector2_0(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_1(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_i32_2(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, distance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_f32_i32_f32_3(
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        layerMask: i32,
        minDepth: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Raycast", (origin, direction, distance, layerMask, minDepth))?;
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
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Raycast",
                (origin, direction, distance, layerMask, minDepth, maxDepth),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPhysicsScene() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::PhysicsScene2D,
    > {
        let __cordl_ret: crate::UnityEngine::PhysicsScene2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultPhysicsScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_queriesHitTriggers() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_queriesHitTriggers", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Physics2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Physics2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
