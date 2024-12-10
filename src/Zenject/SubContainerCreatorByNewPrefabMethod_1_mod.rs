#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefabMethod_1<TParam1: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext,
    pub _installerMethod: *mut crate::System::Action_2<
        *mut crate::Zenject::DiContainer,
        TParam1,
    >,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByNewPrefabMethod_1
    < TParam1 > => "Zenject"."SubContainerCreatorByNewPrefabMethod`1" < TParam1 >
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_1")]
impl<TParam1: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::SubContainerCreatorByNewPrefabMethod_1<TParam1> {
    type Target = crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_1")]
impl<TParam1: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::SubContainerCreatorByNewPrefabMethod_1<TParam1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_1")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
> crate::Zenject::SubContainerCreatorByNewPrefabMethod_1<TParam1> {
    #[cfg(
        feature = "Zenject+SubContainerCreatorByNewPrefabMethod_1+__c__DisplayClass2_0"
    )]
    pub type __c__DisplayClass2_0 = crate::Zenject::SubContainerCreatorByNewPrefabMethod_1___c__DisplayClass2_0<
        TParam1,
    >;
    pub fn AddInstallers(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<*mut crate::Zenject::DiContainer, TParam1>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (container, prefabProvider, gameObjectBindInfo, installerMethod),
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
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<*mut crate::Zenject::DiContainer, TParam1>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (container, prefabProvider, gameObjectBindInfo, installerMethod),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_1")]
impl<TParam1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefabMethod_1<TParam1> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
