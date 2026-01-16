#[cfg(feature = "cordl_class_Zenject+MemoryPoolExpandBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryPoolExpandBinder_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
    pub _MemoryPoolBindInfo_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "cordl_class_Zenject+MemoryPoolExpandBinder_1")]
unsafe impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Zenject::MemoryPoolExpandBinder_1<TContract>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "MemoryPoolExpandBinder`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Zenject", "MemoryPoolExpandBinder`1")
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
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Zenject::MemoryPoolExpandBinder_1<TContract>
{
    type Target = crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Zenject::MemoryPoolExpandBinder_1<TContract>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPoolExpandBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> crate::Zenject::MemoryPoolExpandBinder_1<TContract> {
    pub fn ExpandByDoubling(
        &mut self,
        showExpandWarning: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>>,
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
                    .find_method::<(bool), quest_hook::libil2cpp::Gc<
                        crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
                    >, 1usize>("ExpandByDoubling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExpandByDoubling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (showExpandWarning))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExpandByOneAtATime(
        &mut self,
        showExpandWarning: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>>,
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
                    .find_method::<(bool), quest_hook::libil2cpp::Gc<
                        crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
                    >, 1usize>("ExpandByOneAtATime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExpandByOneAtATime",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryArgumentsToChoiceBinder_1<TContract>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (showExpandWarning))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
        poolBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContract: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (bindContainer, bindInfo, factoryBindInfo, poolBindInfo),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        factoryBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::FactoryBindInfo>,
        poolBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
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
                        quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (bindContainer, bindInfo, factoryBindInfo, poolBindInfo),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MemoryPoolBindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>>
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
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
                        0usize,
                    >("get_MemoryPoolBindInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_MemoryPoolBindInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_MemoryPoolBindInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>,
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
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::MemoryPoolBindInfo>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_MemoryPoolBindInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_MemoryPoolBindInfo", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+MemoryPoolExpandBinder_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::Zenject::MemoryPoolExpandBinder_1<TContract>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
