#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectMethod_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewGameObjectMethod_2<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext,
    pub _installerMethod: *mut crate::System::Action_3<
        *mut crate::Zenject::DiContainer,
        TParam1,
        TParam2,
    >,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectMethod_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::SubContainerCreatorByNewGameObjectMethod_2 < TParam1, TParam2 > =>
    "Zenject"."SubContainerCreatorByNewGameObjectMethod`2" < TParam1, TParam2 >
);
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectMethod_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::SubContainerCreatorByNewGameObjectMethod_2<TParam1, TParam2> {
    type Target = crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectMethod_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::SubContainerCreatorByNewGameObjectMethod_2<TParam1, TParam2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectMethod_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
> crate::Zenject::SubContainerCreatorByNewGameObjectMethod_2<TParam1, TParam2> {
    #[cfg(
        feature = "Zenject+SubContainerCreatorByNewGameObjectMethod_2+__c__DisplayClass2_0"
    )]
    pub type __c__DisplayClass2_0 = crate::Zenject::SubContainerCreatorByNewGameObjectMethod_2___c__DisplayClass2_0<
        TParam1,
        TParam2,
    >;
    pub fn AddInstallers(
        &mut self,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::GameObjectContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInstallers", (args, context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        installerMethod: *mut crate::System::Action_3<
            *mut crate::Zenject::DiContainer,
            TParam1,
            TParam2,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, gameObjectBindInfo, installerMethod))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        gameObjectBindInfo: *mut crate::Zenject::GameObjectCreationParameters,
        installerMethod: *mut crate::System::Action_3<
            *mut crate::Zenject::DiContainer,
            TParam1,
            TParam2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, gameObjectBindInfo, installerMethod))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectMethod_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewGameObjectMethod_2<TParam1, TParam2> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
