#[cfg(feature = "Zenject+StaticMemoryPool_4")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticMemoryPool_4<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::StaticMemoryPoolBase_1<TValue>,
    pub _onSpawnMethod: *mut crate::System::Action_4<TParam1, TParam2, TParam3, TValue>,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TParam3: std::marker::PhantomData<TParam3>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::StaticMemoryPool_4 < TParam1, TParam2,
    TParam3, TValue > => "Zenject"."StaticMemoryPool`4" < TParam1, TParam2, TParam3,
    TValue >
);
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    type Target = crate::Zenject::StaticMemoryPoolBase_1<TValue>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    pub fn New(
        onSpawnMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<TParam1, TParam2, TParam3, TValue>,
        >,
        onDespawnedMethod: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (onSpawnMethod, onDespawnedMethod))?;
        Ok(__cordl_object.into())
    }
    pub fn Spawn(
        &mut self,
        p1: TParam1,
        p2: TParam2,
        p3: TParam3,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Spawn", (p1, p2, p3))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        onSpawnMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<TParam1, TParam2, TParam3, TValue>,
        >,
        onDespawnedMethod: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (onSpawnMethod, onDespawnedMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_OnSpawnMethod(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<TParam1, TParam2, TParam3, TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
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
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn as_ref(&self) -> &crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IMemoryPool>
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn as_ref(&self) -> &crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IMemoryPool>
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IMemoryPool_4<TParam1, TParam2, TParam3, TValue>>
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn as_ref(
        &self,
    ) -> &crate::Zenject::IMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+StaticMemoryPool_4")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IMemoryPool_4<TParam1, TParam2, TParam3, TValue>>
for crate::Zenject::StaticMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Zenject::IMemoryPool_4<TParam1, TParam2, TParam3, TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
