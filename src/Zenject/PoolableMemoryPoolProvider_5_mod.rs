#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_5")]
#[repr(C)]
#[derive(Debug)]
pub struct PoolableMemoryPoolProvider_5<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::PoolableMemoryPoolProviderBase_1<TContract>,
    pub _pool: TMemoryPool,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TParam3: std::marker::PhantomData<TParam3>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
    __cordl_phantom_TMemoryPool: std::marker::PhantomData<TMemoryPool>,
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_5")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PoolableMemoryPoolProvider_5 < TParam1,
    TParam2, TParam3, TContract, TMemoryPool > => "Zenject"
    ."PoolableMemoryPoolProvider`5" < TParam1, TParam2, TParam3, TContract, TMemoryPool >
);
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::PoolableMemoryPoolProvider_5<
    TParam1,
    TParam2,
    TParam3,
    TContract,
    TMemoryPool,
> {
    type Target = crate::Zenject::PoolableMemoryPoolProviderBase_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::PoolableMemoryPoolProvider_5<
    TParam1,
    TParam2,
    TParam3,
    TContract,
    TMemoryPool,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> crate::Zenject::PoolableMemoryPoolProvider_5<
    TParam1,
    TParam2,
    TParam3,
    TContract,
    TMemoryPool,
> {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        poolId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, poolId))?;
        Ok(__cordl_object)
    }
    pub fn Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Validate", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        poolId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, poolId))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::PoolableMemoryPoolProvider_5<
    TParam1,
    TParam2,
    TParam3,
    TContract,
    TMemoryPool,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}