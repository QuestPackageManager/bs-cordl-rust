#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
#[repr(C)]
#[derive(Debug)]
pub struct PoolableMemoryPoolProvider_2<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::PoolableMemoryPoolProviderBase_1<TContract>,
    pub _pool: TMemoryPool,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
    __cordl_phantom_TMemoryPool: std::marker::PhantomData<TMemoryPool>,
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
unsafe impl<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Zenject::PoolableMemoryPoolProvider_2<TContract, TMemoryPool> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "PoolableMemoryPoolProvider`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Zenject",
                        "PoolableMemoryPoolProvider`2",
                    )
                    .unwrap()
                    .make_generic::<(TContract, TMemoryPool)>()
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
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
impl<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::PoolableMemoryPoolProvider_2<TContract, TMemoryPool> {
    type Target = crate::Zenject::PoolableMemoryPoolProviderBase_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
impl<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::PoolableMemoryPoolProvider_2<TContract, TMemoryPool> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
impl<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> crate::Zenject::PoolableMemoryPoolProvider_2<TContract, TMemoryPool> {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Action>,
        >,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::PoolableMemoryPoolProvider_2<
            TContract,
            TMemoryPool,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::Zenject::TypeValuePair,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Action>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("GetAllInstancesWithInjectSplit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::PoolableMemoryPoolProvider_2 < TContract,
                    TMemoryPool > as quest_hook::libil2cpp::Type > ::class(),
                    "GetAllInstancesWithInjectSplit", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (context, args, injectAction, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        poolId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, poolId))?;
        Ok(__cordl_object.into())
    }
    pub fn Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::PoolableMemoryPoolProvider_2<
            TContract,
            TMemoryPool,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Validate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::PoolableMemoryPoolProvider_2 < TContract,
                    TMemoryPool > as quest_hook::libil2cpp::Type > ::class(), "Validate",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        poolId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::PoolableMemoryPoolProvider_2<
            TContract,
            TMemoryPool,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    crate::System::Guid,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::PoolableMemoryPoolProvider_2 < TContract,
                    TMemoryPool > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, poolId))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
impl<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::PoolableMemoryPoolProvider_2<TContract, TMemoryPool> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
impl<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IValidatable>
for crate::Zenject::PoolableMemoryPoolProvider_2<TContract, TMemoryPool> {
    fn as_ref(&self) -> &crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PoolableMemoryPoolProvider_2")]
impl<
    TContract: quest_hook::libil2cpp::Type,
    TMemoryPool: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IValidatable>
for crate::Zenject::PoolableMemoryPoolProvider_2<TContract, TMemoryPool> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}
