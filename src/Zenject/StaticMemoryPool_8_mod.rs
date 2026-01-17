#[cfg(feature = "cordl_class_Zenject+StaticMemoryPool_8")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct StaticMemoryPool_8<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
    TParam7: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::StaticMemoryPoolBase_1<TValue>,
    pub _onSpawnMethod: quest_hook::libil2cpp::Gc<
        crate::System::Action_8<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TValue,
        >,
    >,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TParam3: std::marker::PhantomData<TParam3>,
    __cordl_phantom_TParam4: std::marker::PhantomData<TParam4>,
    __cordl_phantom_TParam5: std::marker::PhantomData<TParam5>,
    __cordl_phantom_TParam6: std::marker::PhantomData<TParam6>,
    __cordl_phantom_TParam7: std::marker::PhantomData<TParam7>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "cordl_class_Zenject+StaticMemoryPool_8")]
unsafe impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "StaticMemoryPool`8";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Zenject", "StaticMemoryPool`8")
                .unwrap()
                .make_generic::<(
                    TParam1,
                    TParam2,
                    TParam3,
                    TParam4,
                    TParam5,
                    TParam6,
                    TParam7,
                    TValue,
                )>()
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
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > std::ops::Deref
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    type Target = crate::Zenject::StaticMemoryPoolBase_1<TValue>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    >
    crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    pub fn New(
        onSpawnMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_8<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TParam5,
                TParam6,
                TParam7,
                TValue,
            >,
        >,
        onDespawnedMethod: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TParam1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (onSpawnMethod, onDespawnedMethod))?;
        Ok(__cordl_object.into())
    }
    pub fn Spawn(
        &mut self,
        p1: TParam1,
        p2: TParam2,
        p3: TParam3,
        p4: TParam4,
        p5: TParam5,
        p6: TParam6,
        p7: TParam7,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TParam1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
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
                        TParam1,
                        TParam2,
                        TParam3,
                        TParam4,
                        TParam5,
                        TParam6,
                        TParam7,
                    ), TValue, 7usize>("Spawn")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Spawn",
                            7usize
                        )
                    })
            });
        let __cordl_ret: TValue =
            unsafe { cordl_method_info.invoke_unchecked(self, (p1, p2, p3, p4, p5, p6, p7))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        onSpawnMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_8<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TParam5,
                TParam6,
                TParam7,
                TValue,
            >,
        >,
        onDespawnedMethod: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
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
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_8<
                                TParam1,
                                TParam2,
                                TParam3,
                                TParam4,
                                TParam5,
                                TParam6,
                                TParam7,
                                TValue,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (onSpawnMethod, onDespawnedMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnSpawnMethod(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_8<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TParam5,
                TParam6,
                TParam7,
                TValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_8<
                            TParam1,
                            TParam2,
                            TParam3,
                            TParam4,
                            TParam5,
                            TParam6,
                            TParam7,
                            TValue,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("set_OnSpawnMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnSpawnMethod",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsRef<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn as_ref(&self) -> &crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsMut<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn as_mut(&mut self) -> &mut crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsRef<crate::Zenject::IMemoryPool>
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn as_ref(&self) -> &crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsMut<crate::Zenject::IMemoryPool>
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn as_mut(&mut self) -> &mut crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    >
    AsRef<
        crate::Zenject::IMemoryPool_8<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TValue,
        >,
    >
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn as_ref(
        &self,
    ) -> &crate::Zenject::IMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_8")]
impl<
        TParam1: quest_hook::libil2cpp::Type,
        TParam2: quest_hook::libil2cpp::Type,
        TParam3: quest_hook::libil2cpp::Type,
        TParam4: quest_hook::libil2cpp::Type,
        TParam5: quest_hook::libil2cpp::Type,
        TParam6: quest_hook::libil2cpp::Type,
        TParam7: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    >
    AsMut<
        crate::Zenject::IMemoryPool_8<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TValue,
        >,
    >
    for crate::Zenject::StaticMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    >
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Zenject::IMemoryPool_8<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
        TValue,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
