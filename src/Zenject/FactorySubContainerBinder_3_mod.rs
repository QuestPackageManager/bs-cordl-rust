#[cfg(feature = "Zenject+FactorySubContainerBinder_3")]
#[repr(C)]
#[derive(Debug)]
pub struct FactorySubContainerBinder_3<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::FactorySubContainerBinderWithParams_1<TContract>,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_3")]
unsafe impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Zenject::FactorySubContainerBinder_3<TParam1, TParam2, TContract> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "FactorySubContainerBinder`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Zenject",
                        "FactorySubContainerBinder`3",
                    )
                    .unwrap()
                    .make_generic::<(TParam1, TParam2, TContract)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "Zenject+FactorySubContainerBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::FactorySubContainerBinder_3<TParam1, TParam2, TContract> {
    type Target = crate::Zenject::FactorySubContainerBinderWithParams_1<TContract>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::FactorySubContainerBinder_3<TParam1, TParam2, TContract> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactorySubContainerBinder_3<TParam1, TParam2, TContract> {
    pub fn ByMethod(
        &mut self,
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                TParam1,
                TParam2,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_3<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                TParam1,
                                TParam2,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("ByMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (installerMethod))? };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewGameObjectMethod(
        &mut self,
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                TParam1,
                TParam2,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_3<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                TParam1,
                                TParam2,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("ByNewGameObjectMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewGameObjectMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (installerMethod))? };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewPrefabMethod(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                TParam1,
                TParam2,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_3<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                    TParam1,
                                    TParam2,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("ByNewPrefabMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewPrefabMethod", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (prefab, installerMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewPrefabResourceMethod(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                TParam1,
                TParam2,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_3<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                    TParam1,
                                    TParam2,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("ByNewPrefabResourceMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewPrefabResourceMethod", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (resourcePath, installerMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
            .invoke_void(
                ".ctor",
                (bindContainer, bindInfo, factoryBindInfo, subIdentifier),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (bindContainer, bindInfo, factoryBindInfo, subIdentifier),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinder_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TContract: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactorySubContainerBinder_3<TParam1, TParam2, TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
