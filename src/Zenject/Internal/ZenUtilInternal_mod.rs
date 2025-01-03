#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenUtilInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::ZenUtilInternal =>
    "Zenject.Internal"."ZenUtilInternal"
);
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl std::ops::Deref for crate::Zenject::Internal::ZenUtilInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl std::ops::DerefMut for crate::Zenject::Internal::ZenUtilInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl crate::Zenject::Internal::ZenUtilInternal {
    pub fn AddStateMachineBehaviourAutoInjectersInScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddStateMachineBehaviourAutoInjectersInScene", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStateMachineBehaviourAutoInjectersUnderGameObject(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddStateMachineBehaviourAutoInjectersUnderGameObject", (root))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreFunctionsEqual(
        left: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        right: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreFunctionsEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllSceneContexts() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Zenject::SceneContext,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Zenject::SceneContext,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllSceneContexts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInheritanceDelta(
        derived: quest_hook::libil2cpp::Gc<crate::System::Type>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInheritanceDelta", (derived, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInjectableMonoBehavioursInScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
        monoBehaviours: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::MonoBehaviour,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInjectableMonoBehavioursInScene", (scene, monoBehaviours))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInjectableMonoBehavioursUnderGameObject(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        injectableComponents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::MonoBehaviour,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetInjectableMonoBehavioursUnderGameObject",
                (gameObject, injectableComponents),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInjectableMonoBehavioursUnderGameObjectInternal(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        injectableComponents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::MonoBehaviour,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetInjectableMonoBehavioursUnderGameObjectInternal",
                (gameObject, injectableComponents),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootGameObjects(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::GameObject,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRootGameObjects", (scene))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInjectableMonoBehaviourType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInjectableMonoBehaviourType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNull", (obj))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::ZenUtilInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
