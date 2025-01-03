#[cfg(feature = "Zenject+IMemoryPool_3")]
#[repr(C)]
#[derive(Debug)]
pub struct IMemoryPool_3<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IMemoryPool_3 < TParam1, TParam2,
    TValue > => "Zenject"."IMemoryPool`3" < TParam1, TParam2, TValue >
);
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    pub fn Spawn(
        &mut self,
        param1: TParam1,
        param2: TParam2,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Spawn", (param1, param2))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    fn as_ref(&self) -> &crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IDespawnableMemoryPool_1<TValue>>
for crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IDespawnableMemoryPool_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IMemoryPool>
for crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    fn as_ref(&self) -> &crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IMemoryPool_3")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IMemoryPool>
for crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IMemoryPool {
        unsafe { std::mem::transmute(self) }
    }
}
