#[cfg(feature = "Zenject+IMemoryPool_5")]
#[repr(C)]
#[derive(Debug)]
pub struct IMemoryPool_5<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TParam3: std::marker::PhantomData<TParam3>,
    __cordl_phantom_TParam4: std::marker::PhantomData<TParam4>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
unsafe impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "IMemoryPool`5";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("Zenject", "IMemoryPool`5")
                    .unwrap()
                    .make_generic::<(TParam1, TParam2, TParam3, TParam4, TValue)>()
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
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    pub fn Spawn(
        &mut self,
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TParam1, TParam2, TParam3, TParam4), TValue, 4usize>("Spawn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Spawn", 4usize
                )
            });
        let __cordl_ret: TValue = unsafe {
            method.invoke_unchecked(self, (param1, param2, param3, param4))
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    fn as_ref(&self) -> &crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IMemoryPool>
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    fn as_ref(&self) -> &crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_5")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IMemoryPool>
for crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
