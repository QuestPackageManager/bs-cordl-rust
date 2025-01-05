#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder1Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder1Extensions =>
    "Zenject"."FactoryFromBinder1Extensions"
);
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder1Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder1Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl crate::Zenject::FactoryFromBinder1Extensions {
    pub fn FromIFactory<TParam1, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_2<TParam1, TContract>,
        >,
        factoryBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ConcreteBinderGeneric_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::IFactory_2<TParam1, TContract>,
                        >,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromIFactory", (fromBinder, factoryBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMonoPoolableMemoryPool_Action_1_1<TParam1, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_2<TParam1, TContract>,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMonoPoolableMemoryPool", (fromBinder, poolBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMonoPoolableMemoryPool_FactoryFromBinder_2_0<TParam1, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_2<TParam1, TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMonoPoolableMemoryPool", (fromBinder))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_Action_1_1<TParam1, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_2<TParam1, TContract>,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder, poolBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_Action_1_3<TParam1, TContract, TMemoryPool>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_2<TParam1, TContract>,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder, poolBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_FactoryFromBinder_2_0<TParam1, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_2<TParam1, TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_FactoryFromBinder_2_2<TParam1, TContract, TMemoryPool>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_2<TParam1, TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder1Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder1Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
