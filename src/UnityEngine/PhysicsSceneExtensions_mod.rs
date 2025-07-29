#[cfg(feature = "cordl_class_UnityEngine+PhysicsSceneExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PhysicsSceneExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+PhysicsSceneExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PhysicsSceneExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "PhysicsSceneExtensions";
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
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
impl std::ops::Deref for crate::UnityEngine::PhysicsSceneExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::PhysicsSceneExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PhysicsSceneExtensions")]
impl crate::UnityEngine::PhysicsSceneExtensions {
    pub fn GetPhysicsScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::SceneManagement::Scene),
                        crate::UnityEngine::PhysicsScene,
                        1usize,
                    >("GetPhysicsScene")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicsScene", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::PhysicsScene = unsafe {
            cordl_method_info.invoke_unchecked((), (scene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsScene_Internal(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::SceneManagement::Scene),
                        crate::UnityEngine::PhysicsScene,
                        1usize,
                    >("GetPhysicsScene_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicsScene_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::PhysicsScene = unsafe {
            cordl_method_info.invoke_unchecked((), (scene))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsScene_Internal_Injected(
        scene: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::SceneManagement::Scene,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PhysicsScene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::SceneManagement::Scene,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::PhysicsScene,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetPhysicsScene_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPhysicsScene_Internal_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (scene, ret))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PhysicsSceneExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PhysicsSceneExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
