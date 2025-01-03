#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PhysicsSceneExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PhysicsSceneExtensions =>
    "UnityEngine"."PhysicsSceneExtensions"
);
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
impl std::ops::Deref for crate::UnityEngine::PhysicsSceneExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::PhysicsSceneExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
impl crate::UnityEngine::PhysicsSceneExtensions {
    pub fn GetPhysicsScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        let __cordl_ret: crate::UnityEngine::PhysicsScene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPhysicsScene", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsScene_Internal(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        let __cordl_ret: crate::UnityEngine::PhysicsScene = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPhysicsScene_Internal", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsScene_Internal_Injected(
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPhysicsScene_Internal_Injected", (scene, ret))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PhysicsSceneExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
