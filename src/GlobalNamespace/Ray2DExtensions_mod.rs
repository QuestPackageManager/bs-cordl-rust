#[cfg(feature = "Ray2DExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Ray2DExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Ray2DExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Ray2DExtensions => ""
    ."Ray2DExtensions"
);
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
