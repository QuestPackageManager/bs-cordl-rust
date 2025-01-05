#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewGameObjectDynamicContext {
    __cordl_parent: crate::Zenject::SubContainerCreatorDynamicContext,
    pub _gameObjectBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::GameObjectCreationParameters,
    >,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::SubContainerCreatorByNewGameObjectDynamicContext => "Zenject"
    ."SubContainerCreatorByNewGameObjectDynamicContext"
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl std::ops::Deref
for crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    type Target = crate::Zenject::SubContainerCreatorDynamicContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl std::ops::DerefMut
for crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
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
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, gameObjectBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, gameObjectBindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
