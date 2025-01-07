#[cfg(feature = "Ray2DExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Ray2DExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Ray2DExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Ray2DExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Ray2DExtensions";
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
#[cfg(feature = "Ray2DExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::Ray2DExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ray2DExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::Ray2DExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ray2DExtensions")]
impl crate::GlobalNamespace::Ray2DExtensions {
    pub fn CircleIntersections(
        ray: crate::UnityEngine::Ray2D,
        circleCenter: crate::UnityEngine::Vector2,
        radius: f32,
        distances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CircleIntersections", (ray, circleCenter, radius, distances))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Ray2DExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Ray2DExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
