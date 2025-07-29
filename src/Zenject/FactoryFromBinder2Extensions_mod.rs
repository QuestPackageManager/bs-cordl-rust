#[cfg(feature = "cordl_class_Zenject+FactoryFromBinder2Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder2Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Zenject+FactoryFromBinder2Extensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::FactoryFromBinder2Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "FactoryFromBinder2Extensions";
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
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder2Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder2Extensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder2Extensions")]
impl crate::Zenject::FactoryFromBinder2Extensions {
    pub fn FromIFactory<TParam1, TParam2, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_3<TParam1, TParam2, TContract>,
        >,
        factoryBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ConcreteBinderGeneric_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::IFactory_3<TParam1, TParam2, TContract>,
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
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::FactoryFromBinder_3<
                                    TParam1,
                                    TParam2,
                                    TContract,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Zenject::ConcreteBinderGeneric_1<
                                            quest_hook::libil2cpp::Gc<
                                                crate::Zenject::IFactory_3<TParam1, TParam2, TContract>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("FromIFactory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromIFactory", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (fromBinder, factoryBindGenerator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromMonoPoolableMemoryPool_Action_1_1<TParam1, TParam2, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_3<TParam1, TParam2, TContract>,
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
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::FactoryFromBinder_3<
                                    TParam1,
                                    TParam2,
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
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("FromMonoPoolableMemoryPool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromMonoPoolableMemoryPool", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (fromBinder, poolBindGenerator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromMonoPoolableMemoryPool_FactoryFromBinder_3_0<TParam1, TParam2, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_3<TParam1, TParam2, TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Zenject::FactoryFromBinder_3<
                                TParam1,
                                TParam2,
                                TContract,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("FromMonoPoolableMemoryPool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromMonoPoolableMemoryPool", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked((), (fromBinder))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_Action_1_1<TParam1, TParam2, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_3<TParam1, TParam2, TContract>,
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
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::FactoryFromBinder_3<
                                    TParam1,
                                    TParam2,
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
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("FromPoolableMemoryPool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromPoolableMemoryPool", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (fromBinder, poolBindGenerator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_Action_1_3<TParam1, TParam2, TContract, TMemoryPool>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_3<TParam1, TParam2, TContract>,
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
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::FactoryFromBinder_3<
                                    TParam1,
                                    TParam2,
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
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("FromPoolableMemoryPool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromPoolableMemoryPool", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (fromBinder, poolBindGenerator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_FactoryFromBinder_3_0<TParam1, TParam2, TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_3<TParam1, TParam2, TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Zenject::FactoryFromBinder_3<
                                TParam1,
                                TParam2,
                                TContract,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("FromPoolableMemoryPool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromPoolableMemoryPool", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked((), (fromBinder))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_FactoryFromBinder_3_2<
        TParam1,
        TParam2,
        TContract,
        TMemoryPool,
    >(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_3<TParam1, TParam2, TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Zenject::FactoryFromBinder_3<
                                TParam1,
                                TParam2,
                                TContract,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("FromPoolableMemoryPool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromPoolableMemoryPool", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked((), (fromBinder))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+FactoryFromBinder2Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder2Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
