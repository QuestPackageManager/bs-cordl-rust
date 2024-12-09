#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefab")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefab {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    pub _prefabProvider: *mut crate::Zenject::IPrefabProvider,
    pub _container: *mut crate::Zenject::DiContainer,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefab")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByNewPrefab =>
    "Zenject"."SubContainerCreatorByNewPrefab"
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefab")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByNewPrefab {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefab")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByNewPrefab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefab")]
impl crate::Zenject::SubContainerCreatorByNewPrefab {
    pub fn CreateSubContainer(
        &mut self,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        parentContext: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("CreateSubContainer", (args, parentContext))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        prefabProvider: *mut crate::Zenject::IPrefabProvider,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, prefabProvider, gameObjectBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        prefabProvider: *mut crate::Zenject::IPrefabProvider,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, prefabProvider, gameObjectBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefab")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefab {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
