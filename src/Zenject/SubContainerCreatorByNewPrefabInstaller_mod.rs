#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefabInstaller {
    __cordl_parent: crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext,
    pub _installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _extraArgs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
    >,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByNewPrefabInstaller
    => "Zenject"."SubContainerCreatorByNewPrefabInstaller"
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabInstaller")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByNewPrefabInstaller {
    type Target = crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabInstaller")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByNewPrefabInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabInstaller")]
impl crate::Zenject::SubContainerCreatorByNewPrefabInstaller {
    pub fn AddInstallers(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInstallers", (args, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (container, prefabProvider, gameObjectBindInfo, installerType, extraArgs),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (container, prefabProvider, gameObjectBindInfo, installerType, extraArgs),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefabInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
