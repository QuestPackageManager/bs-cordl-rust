#[cfg(feature = "UnityEngine+PhysicsScene2D")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsScene2D {
    pub m_Handle: i32,
}
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PhysicsScene2D {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "PhysicsScene2D";
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
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::PhysicsScene2D {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::PhysicsScene2D {
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
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::PhysicsScene2D {
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
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::PhysicsScene2D {
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
    pub fn Equals_PhysicsScene2D1(
        &mut self,
        other: crate::UnityEngine::PhysicsScene2D,
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
    pub fn GetRayIntersectionArray_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene2D,
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRayIntersectionArray_Internal",
                (physicsScene, origin, direction, distance, layerMask, results),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersectionArray_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::PhysicsScene2D,
        >,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: f32,
        layerMask: i32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRayIntersectionArray_Internal_Injected",
                (physicsScene, origin, direction, distance, layerMask, results),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersection_Il2CppArray_i32_1(
        &mut self,
        ray: crate::UnityEngine::Ray,
        distance: f32,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRayIntersection",
            (ray, distance, results, layerMask),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersection_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene2D,
        origin: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRayIntersection_Internal",
                (physicsScene, origin, direction, distance, layerMask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRayIntersection_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::PhysicsScene2D,
        >,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: f32,
        layerMask: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRayIntersection_Internal_Injected",
                (physicsScene, origin, direction, distance, layerMask, ret),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn RaycastArray_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene2D,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastArray_Internal",
                (physicsScene, origin, direction, distance, contactFilter, results),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastArray_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::PhysicsScene2D,
        >,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        distance: f32,
        contactFilter: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ContactFilter2D,
        >,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastArray_Internal_Injected",
                (physicsScene, origin, direction, distance, contactFilter, results),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastList_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene2D,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastList_Internal",
                (physicsScene, origin, direction, distance, contactFilter, results),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastList_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::PhysicsScene2D,
        >,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        distance: f32,
        contactFilter: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ContactFilter2D,
        >,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastList_Internal_Injected",
                (physicsScene, origin, direction, distance, contactFilter, results),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_ContactFilter2D_Il2CppArray2(
        &mut self,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, distance, contactFilter, results),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_ContactFilter2D_List_1_3(
        &mut self,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::RaycastHit2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (origin, direction, distance, contactFilter, results),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Internal(
        physicsScene: crate::UnityEngine::PhysicsScene2D,
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
        distance: f32,
        contactFilter: crate::UnityEngine::ContactFilter2D,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RaycastHit2D> {
        let __cordl_ret: crate::UnityEngine::RaycastHit2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Raycast_Internal",
                (physicsScene, origin, direction, distance, contactFilter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Internal_Injected(
        physicsScene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::PhysicsScene2D,
        >,
        origin: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        direction: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        distance: f32,
        contactFilter: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ContactFilter2D,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RaycastHit2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Raycast_Internal_Injected",
                (physicsScene, origin, direction, distance, contactFilter, ret),
            )?;
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene2D>>
for crate::UnityEngine::PhysicsScene2D {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene2D> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+PhysicsScene2D")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene2D>>
for crate::UnityEngine::PhysicsScene2D {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::PhysicsScene2D> {
        todo!()
    }
}
