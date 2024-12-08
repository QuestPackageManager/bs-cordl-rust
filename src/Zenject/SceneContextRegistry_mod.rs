#[cfg(feature = "Zenject+SceneContextRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneContextRegistry {
    __cordl_parent: crate::System::Object,
    pub _map: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::SceneManagement::Scene,
        *mut crate::Zenject::SceneContext,
    >,
}
#[cfg(feature = "Zenject+SceneContextRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SceneContextRegistry => "Zenject"
    ."SceneContextRegistry"
);
#[cfg(feature = "Zenject+SceneContextRegistry")]
impl std::ops::Deref for crate::Zenject::SceneContextRegistry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SceneContextRegistry")]
impl std::ops::DerefMut for crate::Zenject::SceneContextRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SceneContextRegistry")]
impl crate::Zenject::SceneContextRegistry {
    pub fn Add(
        &mut self,
        context: *mut crate::Zenject::SceneContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (context))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetContainerForScene(
        &mut self,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("TryGetContainerForScene", (scene))?;
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
    pub fn Remove(
        &mut self,
        context: *mut crate::Zenject::SceneContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (context))?;
        Ok(__cordl_ret)
    }
    pub fn get_SceneContexts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::SceneContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::SceneContext,
        > = __cordl_object.invoke("get_SceneContexts", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetSceneContextForScene_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SceneContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SceneContext = __cordl_object
            .invoke("TryGetSceneContextForScene", (name))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetSceneContextForScene_Scene1(
        &mut self,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SceneContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SceneContext = __cordl_object
            .invoke("TryGetSceneContextForScene", (scene))?;
        Ok(__cordl_ret)
    }
    pub fn GetSceneContextForScene_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SceneContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SceneContext = __cordl_object
            .invoke("GetSceneContextForScene", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetSceneContextForScene_Scene1(
        &mut self,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SceneContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SceneContext = __cordl_object
            .invoke("GetSceneContextForScene", (scene))?;
        Ok(__cordl_ret)
    }
    pub fn GetContainerForScene(
        &mut self,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("GetContainerForScene", (scene))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SceneContextRegistry")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SceneContextRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
