#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IDespawnableMemoryPool_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IDespawnableMemoryPool_1 < TValue > =>
    "Zenject"."IDespawnableMemoryPool`1" < TValue >
);
#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::IDespawnableMemoryPool_1<TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::IDespawnableMemoryPool_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::Zenject::IDespawnableMemoryPool_1<TValue> {
    pub fn Despawn(
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
            .invoke("Despawn", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::IDespawnableMemoryPool_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool>>
for crate::Zenject::IDespawnableMemoryPool_1<TValue> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IDespawnableMemoryPool_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool>>
for crate::Zenject::IDespawnableMemoryPool_1<TValue> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool> {
        unsafe { std::mem::transmute(self) }
    }
}
