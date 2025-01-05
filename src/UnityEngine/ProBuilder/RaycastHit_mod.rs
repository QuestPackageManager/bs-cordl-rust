#[cfg(feature = "UnityEngine+ProBuilder+RaycastHit")]
#[repr(C)]
#[derive(Debug)]
pub struct RaycastHit {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub distance: f32,
    pub point: crate::UnityEngine::Vector3,
    pub normal: crate::UnityEngine::Vector3,
    pub face: i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+RaycastHit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::RaycastHit =>
    "UnityEngine.ProBuilder"."RaycastHit"
);
#[cfg(feature = "UnityEngine+ProBuilder+RaycastHit")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::RaycastHit {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+RaycastHit")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::RaycastHit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+RaycastHit")]
impl crate::UnityEngine::ProBuilder::RaycastHit {
    pub fn New(
        distance: f32,
        point: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
        face: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (distance, point, normal, face))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        distance: f32,
        point: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
        face: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (distance, point, normal, face))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+RaycastHit")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::RaycastHit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
