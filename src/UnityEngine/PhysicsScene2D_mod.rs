#[cfg(feature = "UnityEngine+PhysicsScene2D")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PhysicsScene2D {
    pub m_Handle: i32,
}
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PhysicsScene2D => "UnityEngine"
    ."PhysicsScene2D"
);
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::PhysicsScene2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
impl crate::UnityEngine::PhysicsScene2D {
    pub fn Equals_Object0(
        &mut self,
        other: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_PhysicsScene2D1(
        &mut self,
        other: crate::UnityEngine::PhysicsScene2D,
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
    pub fn GetRayIntersection_Il2CppArray_i32_1(
        &mut self,
        ray: crate::UnityEngine::Ray,
        distance: f32,
        results: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::RaycastHit2D,
        >,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRayIntersection",
            (ray, distance, results, layerMask),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetRayIntersection_i32_0(
        &mut self,
        ray: crate::UnityEngine::Ray,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRayIntersection",
            (ray, distance, layerMask),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Raycast_ContactFilter2D1(
        &mut self,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, distance, contactFilter),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Raycast_ContactFilter2D_Il2CppArray2(
        &mut self,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::RaycastHit2D,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, distance, contactFilter, results),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Raycast_ContactFilter2D_List_1_3(
        &mut self,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::RaycastHit2D,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, distance, contactFilter, results),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Raycast_i32_0(
        &mut self,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, distance, layerMask),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}