#[cfg(feature = "cordl_class_Zenject+FactoryFromBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::FactoryFromBinderBase,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "cordl_class_Zenject+FactoryFromBinder_1")]
unsafe impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Zenject::FactoryFromBinder_1<TContract>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "FactoryFromBinder`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Zenject", "FactoryFromBinder`1")
                .unwrap()
                .make_generic::<(TContract)>()
                .unwrap()
                .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Zenject::FactoryFromBinder_1<TContract>
{
    type Target = crate::Zenject::FactoryFromBinderBase;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Zenject::FactoryFromBinder_1<TContract>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> crate::Zenject::FactoryFromBinder_1<TContract> {
    pub fn FromComponentInHierarchy(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("FromComponentInHierarchy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromComponentInHierarchy", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (includeInactive))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromFactory<TSubFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSubFactory: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ArgConditionCopyNonLazyBinder,
                        >,
                        0usize,
                    >("FromFactory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromFactory", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FromMethod(
        &mut self,
        method: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                TContract,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                TContract,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("FromMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveGetter_Func_2_0<TObj>(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<TObj, TContract>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConditionCopyNonLazyBinder,
                        >,
                        1usize,
                    >("FromResolveGetter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromResolveGetter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveGetter_Il2CppObject_Func_2_1<TObj>(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<TObj, TContract>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConditionCopyNonLazyBinder,
                        >,
                        2usize,
                    >("FromResolveGetter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromResolveGetter", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (subIdentifier, method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveGetter_Il2CppObject_Func_2_InjectSources2<TObj>(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TObj: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<TObj, TContract>,
                            >,
                            crate::Zenject::InjectSources,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConditionCopyNonLazyBinder,
                        >,
                        3usize,
                    >("FromResolveGetter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromResolveGetter", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ConditionCopyNonLazyBinder> =
            unsafe { cordl_method_info.invoke_unchecked(self, (subIdentifier, method, source))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromSubContainerResolve_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FactorySubContainerBinder_1<TContract>>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::Zenject::FactorySubContainerBinder_1<TContract>,
                    >, 0usize>("FromSubContainerResolve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FromSubContainerResolve",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactorySubContainerBinder_1<TContract>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FromSubContainerResolve_Il2CppObject1(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FactorySubContainerBinder_1<TContract>>,
    >
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::FactorySubContainerBinder_1<TContract>,
                        >,
                        1usize,
                    >("FromSubContainerResolve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromSubContainerResolve", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactorySubContainerBinder_1<TContract>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (subIdentifier))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, bindInfo, factoryBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (container, bindInfo, factoryBindInfo))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+FactoryFromBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::Zenject::FactoryFromBinder_1<TContract>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
