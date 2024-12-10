#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefabWithParams {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _prefabProvider: *mut crate::Zenject::IPrefabProvider,
    pub _installerType: *mut crate::System::Type,
    pub _gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::SubContainerCreatorByNewPrefabWithParams => "Zenject"
    ."SubContainerCreatorByNewPrefabWithParams"
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    #[cfg(
        feature = "Zenject+SubContainerCreatorByNewPrefabWithParams+__c__DisplayClass7_0"
    )]
    pub type __c__DisplayClass7_0 = crate::Zenject::SubContainerCreatorByNewPrefabWithParams___c__DisplayClass7_0;
    pub fn CreateSubContainer(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        parentContext: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("CreateSubContainer", (args, parentContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTempContainer(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("CreateTempContainer", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (installerType, container, prefabProvider, gameObjectBindInfo),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
            .invoke(
                ".ctor",
                (installerType, container, prefabProvider, gameObjectBindInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("get_Container", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl AsRef<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn as_ref(&self) -> &crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabWithParams")]
impl AsMut<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByNewPrefabWithParams {
    fn as_mut(&mut self) -> &mut crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
