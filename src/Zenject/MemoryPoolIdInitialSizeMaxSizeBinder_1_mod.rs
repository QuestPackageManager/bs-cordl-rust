#[cfg(feature = "Zenject+MemoryPoolIdInitialSizeMaxSizeBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryPoolIdInitialSizeMaxSizeBinder_1<
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+MemoryPoolIdInitialSizeMaxSizeBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1
    < TContract > => "Zenject"."MemoryPoolIdInitialSizeMaxSizeBinder`1" < TContract >
);
#[cfg(feature = "Zenject+MemoryPoolIdInitialSizeMaxSizeBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TContract> {
    type Target = crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolIdInitialSizeMaxSizeBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolIdInitialSizeMaxSizeBinder_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TContract> {
    pub fn New(
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
        poolBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindContainer, bindInfo, factoryBindInfo, poolBindInfo),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn WithId(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
        > = __cordl_object.invoke("WithId", (identifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
        poolBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, factoryBindInfo, poolBindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+MemoryPoolIdInitialSizeMaxSizeBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::MemoryPoolIdInitialSizeMaxSizeBinder_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
