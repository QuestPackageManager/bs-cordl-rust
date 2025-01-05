#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
#[repr(C)]
#[derive(Debug)]
pub struct DecoratorProvider_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cachedInstances: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _factoryBindIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::System::Guid>,
    >,
    pub _decoratorFactories: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Zenject::IFactory_2<TContract, TContract>>,
        >,
    >,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::DecoratorProvider_1 <
    TContract > => "Zenject.Internal"."DecoratorProvider`1" < TContract >
);
#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::Internal::DecoratorProvider_1<TContract> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::Internal::DecoratorProvider_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::Internal::DecoratorProvider_1<TContract> {
    pub fn AddFactoryId(
        &mut self,
        factoryBindId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFactoryId", (factoryBindId))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecorateInstance(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("DecorateInstance", (instance, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllInstances(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllInstances", (provider, context, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn LazyInitializeDecoratorFactories(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyInitializeDecoratorFactories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container))?;
        Ok(__cordl_object.into())
    }
    pub fn WrapProviderInstances(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WrapProviderInstances", (provider, context, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::DecoratorProvider_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::Internal::IDecoratorProvider>
for crate::Zenject::Internal::DecoratorProvider_1<TContract> {
    fn as_ref(&self) -> &crate::Zenject::Internal::IDecoratorProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Internal+DecoratorProvider_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::Internal::IDecoratorProvider>
for crate::Zenject::Internal::DecoratorProvider_1<TContract> {
    fn as_mut(&mut self) -> &mut crate::Zenject::Internal::IDecoratorProvider {
        unsafe { std::mem::transmute(self) }
    }
}
