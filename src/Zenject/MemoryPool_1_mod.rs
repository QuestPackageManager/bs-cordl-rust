#[cfg(feature = "Zenject+MemoryPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryPool_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::MemoryPoolBase_1<TValue>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+MemoryPool_1")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Zenject::MemoryPool_1<TValue> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "MemoryPool`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("Zenject", "MemoryPool`1")
                    .unwrap()
                    .make_generic::<(TValue)>()
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
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::MemoryPool_1<TValue> {
    type Target = crate::Zenject::MemoryPoolBase_1<TValue>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::MemoryPool_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> crate::Zenject::MemoryPool_1<TValue> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reinitialize(
        &mut self,
        item: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reinitialize", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn Spawn(&mut self) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Spawn", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Zenject_IFactory_TValue__Create(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke("Zenject.IFactory<TValue>.Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_ref(&self) -> &crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::Zenject::IFactory>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_ref(&self) -> &crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::Zenject::IFactory>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::Zenject::IFactory_1<TValue>>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_ref(&self) -> &crate::Zenject::IFactory_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::Zenject::IFactory_1<TValue>>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IFactory_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::Zenject::IMemoryPool>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_ref(&self) -> &crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::Zenject::IMemoryPool>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::Zenject::IMemoryPool_1<TValue>>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_ref(&self) -> &crate::Zenject::IMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+MemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::Zenject::IMemoryPool_1<TValue>>
for crate::Zenject::MemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
