#[cfg(feature = "Zenject+IFactoryProvider_3")]
#[repr(C)]
#[derive(Debug)]
pub struct IFactoryProvider_3<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::IFactoryProviderBase_1<TContract>,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+IFactoryProvider_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IFactoryProvider_3 < TParam1, TParam2,
    TContract > => "Zenject"."IFactoryProvider`3" < TParam1, TParam2, TContract >
);
#[cfg(feature = "Zenject+IFactoryProvider_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::IFactoryProvider_3<TParam1, TParam2, TContract> {
    type Target = crate::Zenject::IFactoryProviderBase_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IFactoryProvider_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::IFactoryProvider_3<TParam1, TParam2, TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IFactoryProvider_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::IFactoryProvider_3<TParam1, TParam2, TContract> {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetAllInstancesWithInjectSplit",
                (context, args, injectAction, buffer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        factoryId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, factoryId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        factoryId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, factoryId))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+IFactoryProvider_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::IFactoryProvider_3<TParam1, TParam2, TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
