#[cfg(feature = "MemoryPoolContainer_2")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryPoolContainer_2<
    T0: quest_hook::libil2cpp::Type,
    T1: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub _activeItems: *mut LazyCopyHashSet_1<T1>,
    pub _memoryPool: *mut crate::Zenject::IMemoryPool_1<T0>,
    __cordl_phantom_T0: std::marker::PhantomData<T0>,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "MemoryPoolContainer_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MemoryPoolContainer_2 < T0, T1 > => ""
    ."MemoryPoolContainer`2" < T0, T1 >
);
#[cfg(feature = "MemoryPoolContainer_2")]
impl<T0: quest_hook::libil2cpp::Type, T1: quest_hook::libil2cpp::Type> std::ops::Deref
for MemoryPoolContainer_2<T0, T1> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MemoryPoolContainer_2")]
impl<T0: quest_hook::libil2cpp::Type, T1: quest_hook::libil2cpp::Type> std::ops::DerefMut
for MemoryPoolContainer_2<T0, T1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MemoryPoolContainer_2")]
impl<
    T0: quest_hook::libil2cpp::Type,
    T1: quest_hook::libil2cpp::Type,
> MemoryPoolContainer_2<T0, T1> {
    pub fn Despawn(
        &mut self,
        item: T0,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Despawn", (item))?;
        Ok(__cordl_ret)
    }
    pub fn get_activeItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<T1>,
    >
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<T1> = __cordl_object
            .invoke("get_activeItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn Spawn(&mut self) -> quest_hook::libil2cpp::Result<T0>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T0 = __cordl_object.invoke("Spawn", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        memoryPool: *mut crate::Zenject::IMemoryPool_1<T0>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (memoryPool))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        memoryPool: *mut crate::Zenject::IMemoryPool_1<T0>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (memoryPool))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MemoryPoolContainer_2")]
impl<
    T0: quest_hook::libil2cpp::Type,
    T1: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for MemoryPoolContainer_2<T0, T1> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
