#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefabMethod {
    __cordl_parent: crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext,
    pub _installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByNewPrefabMethod =>
    "Zenject"."SubContainerCreatorByNewPrefabMethod"
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByNewPrefabMethod {
    type Target = crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByNewPrefabMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod")]
impl crate::Zenject::SubContainerCreatorByNewPrefabMethod {
    pub fn AddInstallers(
        &mut self,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::GameObjectContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInstallers", (args, context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        prefabProvider: *mut crate::Zenject::IPrefabProvider,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (container, prefabProvider, gameObjectBindInfo, installerMethod),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        prefabProvider: *mut crate::Zenject::IPrefabProvider,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        installerMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (container, prefabProvider, gameObjectBindInfo, installerMethod),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefabMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}