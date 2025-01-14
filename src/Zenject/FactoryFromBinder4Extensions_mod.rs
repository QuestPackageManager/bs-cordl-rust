#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder4Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+FactoryFromBinder4Extensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::FactoryFromBinder4Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "FactoryFromBinder4Extensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ConcreteBinderGeneric_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::IFactory_5<
                                TParam1,
                                TParam2,
                                TParam3,
                                TParam4,
                                TContract,
                            >,
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
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::FactoryFromBinder_5<
                            TParam1,
                            TParam2,
                            TParam3,
                            TParam4,
                            TContract,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::ConcreteBinderGeneric_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Zenject::IFactory_5<
                                            TParam1,
                                            TParam2,
                                            TParam3,
                                            TParam4,
                                            TContract,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                2usize,
            >("FromIFactory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromIFactory", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked((), (fromBinder, factoryBindGenerator)) };
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
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::FactoryFromBinder_5<
                            TParam1,
                            TParam2,
                            TParam3,
                            TParam4,
                            TContract,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<
                                    TContract,
                                >,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                2usize,
            >("FromMonoPoolableMemoryPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromMonoPoolableMemoryPool", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked((), (fromBinder, poolBindGenerator)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Zenject::FactoryFromBinder_5<
                        TParam1,
                        TParam2,
                        TParam3,
                        TParam4,
                        TContract,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                1usize,
            >("FromMonoPoolableMemoryPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromMonoPoolableMemoryPool", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked((), (fromBinder)) };
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
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::FactoryFromBinder_5<
                            TParam1,
                            TParam2,
                            TParam3,
                            TParam4,
                            TContract,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<
                                    TContract,
                                >,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                2usize,
            >("FromPoolableMemoryPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromPoolableMemoryPool", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked((), (fromBinder, poolBindGenerator)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::FactoryFromBinder_5<
                            TParam1,
                            TParam2,
                            TParam3,
                            TParam4,
                            TContract,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<
                                    TContract,
                                >,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                2usize,
            >("FromPoolableMemoryPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromPoolableMemoryPool", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked((), (fromBinder, poolBindGenerator)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Zenject::FactoryFromBinder_5<
                        TParam1,
                        TParam2,
                        TParam3,
                        TParam4,
                        TContract,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                1usize,
            >("FromPoolableMemoryPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromPoolableMemoryPool", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked((), (fromBinder)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Zenject::FactoryFromBinder_5<
                        TParam1,
                        TParam2,
                        TParam3,
                        TParam4,
                        TContract,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
                1usize,
            >("FromPoolableMemoryPool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromPoolableMemoryPool", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked((), (fromBinder)) };
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
