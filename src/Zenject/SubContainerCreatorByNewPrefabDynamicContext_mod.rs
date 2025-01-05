#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabDynamicContext")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefabDynamicContext {
    __cordl_parent: crate::Zenject::SubContainerCreatorDynamicContext,
    pub _prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
    pub _gameObjectBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::GameObjectCreationParameters,
    >,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabDynamicContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::SubContainerCreatorByNewPrefabDynamicContext => "Zenject"
    ."SubContainerCreatorByNewPrefabDynamicContext"
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabDynamicContext")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext {
    type Target = crate::Zenject::SubContainerCreatorDynamicContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabDynamicContext")]
impl std::ops::DerefMut
for crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabDynamicContext")]
impl crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext {
    pub fn CreateGameObject(
        &mut self,
        shouldMakeActive: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("CreateGameObject", (shouldMakeActive))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, prefabProvider, gameObjectBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, prefabProvider, gameObjectBindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabDynamicContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
