#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder4Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder4Extensions =>
    "Zenject"."FactoryFromBinder4Extensions"
);
#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder4Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder4Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
impl crate::Zenject::FactoryFromBinder4Extensions {
    #[cfg(feature = "Zenject+FactoryFromBinder4Extensions+__c__1_5")]
    pub type __c__1_5<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder4Extensions___c__1_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder4Extensions+__c__3_5")]
    pub type __c__3_5<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder4Extensions___c__3_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder4Extensions+__c__5_6")]
    pub type __c__5_6<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder4Extensions___c__5_6<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
        TMemoryPool,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder4Extensions+__c__DisplayClass0_0_5")]
    pub type __c__DisplayClass0_0_5<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder4Extensions___c__DisplayClass0_0_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder4Extensions+__c__DisplayClass6_0_6")]
    pub type __c__DisplayClass6_0_6<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder4Extensions___c__DisplayClass6_0_6<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
        TMemoryPool,
    >;
    pub fn FromIFactory<TParam1, TParam2, TParam3, TParam4, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_5<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TContract,
            >,
        >,
        factoryBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::ConcreteBinderGeneric_1<
                    *mut crate::Zenject::IFactory_5<
                        TParam1,
                        TParam2,
                        TParam3,
                        TParam4,
                        TContract,
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
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
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
    pub fn FromMonoPoolableMemoryPool_Action_1_1<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
    >(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_5<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TContract,
            >,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
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
    pub fn FromMonoPoolableMemoryPool_FactoryFromBinder_5_0<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
    >(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_5<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TContract,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
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
    pub fn FromPoolableMemoryPool_Action_1_1<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
    >(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_5<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TContract,
            >,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
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
    pub fn FromPoolableMemoryPool_Action_1_3<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
        TMemoryPool,
    >(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_5<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TContract,
            >,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
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
    pub fn FromPoolableMemoryPool_FactoryFromBinder_5_0<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
    >(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_5<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TContract,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
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
    pub fn FromPoolableMemoryPool_FactoryFromBinder_5_2<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TContract,
        TMemoryPool,
    >(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_5<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TContract,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
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
#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder4Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
