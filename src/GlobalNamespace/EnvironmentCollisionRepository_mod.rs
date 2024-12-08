#[cfg(feature = "EnvironmentCollisionRepository")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentCollisionRepository {
    __cordl_parent: crate::System::Object,
    pub _colliders: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::Collider,
        *mut crate::GlobalNamespace::ColliderEventEffect,
    >,
}
#[cfg(feature = "EnvironmentCollisionRepository")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentCollisionRepository
    => ""."EnvironmentCollisionRepository"
);
#[cfg(feature = "EnvironmentCollisionRepository")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentCollisionRepository {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentCollisionRepository")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentCollisionRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentCollisionRepository")]
impl crate::GlobalNamespace::EnvironmentCollisionRepository {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RegisterColliderEventEffect(
        &mut self,
        colliderEventEffect: *mut crate::GlobalNamespace::ColliderEventEffect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterColliderEventEffect", (colliderEventEffect))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetColliderEventEffect(
        &mut self,
        collider: *mut crate::UnityEngine::Collider,
        colliderEventEffect: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::ColliderEventEffect,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetColliderEventEffect", (collider, colliderEventEffect))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "EnvironmentCollisionRepository")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentCollisionRepository {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
