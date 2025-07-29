#[cfg(feature = "cordl_class_Zenject+FactorySubContainerBinderWithParams_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FactorySubContainerBinderWithParams_1<
    TContract: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::FactorySubContainerBinderBase_1<TContract>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "cordl_class_Zenject+FactorySubContainerBinderWithParams_1")]
unsafe impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "FactorySubContainerBinderWithParams`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Zenject",
                        "FactorySubContainerBinderWithParams`1",
                    )
                    .unwrap()
                    .make_generic::<(TContract)>()
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
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    type Target = crate::Zenject::FactorySubContainerBinderBase_1<TContract>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactorySubContainerBinderWithParams_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    pub fn ByNewContextPrefabResource_Il2CppString0<TInstaller>(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("ByNewContextPrefabResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewContextPrefabResource", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (resourcePath))? };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewContextPrefabResource_Type_Il2CppString1(
        &mut self,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("ByNewContextPrefabResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewContextPrefabResource", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (installerType, resourcePath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewContextPrefab_Object0<TInstaller>(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("ByNewContextPrefab")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewContextPrefab", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (prefab))? };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewContextPrefab_Type_Object1(
        &mut self,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("ByNewContextPrefab")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewContextPrefab", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (installerType, prefab))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewPrefabResource_Il2CppString0<TInstaller>(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("ByNewPrefabResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewPrefabResource", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (resourcePath))? };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewPrefabResource_Type_Il2CppString1(
        &mut self,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("ByNewPrefabResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewPrefabResource", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (installerType, resourcePath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewPrefab_Object1<TInstaller>(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstaller: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("ByNewPrefab")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewPrefab", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (prefab))? };
        Ok(__cordl_ret.into())
    }
    pub fn ByNewPrefab_Type_Object0(
        &mut self,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("ByNewPrefab")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ByNewPrefab", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (installerType, prefab))?
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
#[cfg(feature = "cordl_class_Zenject+FactorySubContainerBinderWithParams_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::FactorySubContainerBinderWithParams_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
