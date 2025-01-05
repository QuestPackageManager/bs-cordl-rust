#[cfg(feature = "Zenject+StaticMemoryPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticMemoryPool_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TValue>,
    pub _onSpawnMethod: quest_hook::libil2cpp::Gc<TValue>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::StaticMemoryPool_1 < TValue > =>
    "Zenject"."StaticMemoryPool`1" < TValue >
);
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::StaticMemoryPool_1<TValue> {
    type Target = quest_hook::libil2cpp::Gc<TValue>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> crate::Zenject::StaticMemoryPool_1<TValue> {
    pub fn New(
        onSpawnMethod: quest_hook::libil2cpp::Gc<TValue>,
        onDespawnedMethod: quest_hook::libil2cpp::Gc<TValue>,
        initialSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (onSpawnMethod, onDespawnedMethod, initialSize))?;
        Ok(__cordl_object.into())
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
    pub fn _ctor(
        &mut self,
        onSpawnMethod: quest_hook::libil2cpp::Gc<TValue>,
        onDespawnedMethod: quest_hook::libil2cpp::Gc<TValue>,
        initialSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (onSpawnMethod, onDespawnedMethod, initialSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_OnSpawnMethod(
        &mut self,
        value: quest_hook::libil2cpp::Gc<TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OnSpawnMethod", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TValue>>
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TValue>>
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TValue>>
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TValue>>
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool>>
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool>>
for crate::Zenject::StaticMemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool> {
        unsafe { std::mem::transmute(self) }
    }
}
