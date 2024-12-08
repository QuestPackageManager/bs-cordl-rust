#[cfg(feature = "Zenject+DisposeBlock")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposeBlock {
    __cordl_parent: crate::System::Object,
    pub _disposables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::IDisposable,
    >,
    pub _objectPoolPairs: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::DisposeBlock_SpawnedObjectPoolPair,
    >,
}
#[cfg(feature = "Zenject+DisposeBlock")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DisposeBlock => "Zenject"
    ."DisposeBlock"
);
#[cfg(feature = "Zenject+DisposeBlock")]
impl std::ops::Deref for crate::Zenject::DisposeBlock {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DisposeBlock")]
impl std::ops::DerefMut for crate::Zenject::DisposeBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DisposeBlock")]
impl crate::Zenject::DisposeBlock {
    #[cfg(feature = "Zenject+DisposeBlock+SpawnedObjectPoolPair")]
    pub type SpawnedObjectPoolPair = crate::Zenject::DisposeBlock_SpawnedObjectPoolPair;
    pub fn Add(
        &mut self,
        disposable: *mut crate::System::IDisposable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (disposable))?;
        Ok(__cordl_ret)
    }
    pub fn AddRange<T>(
        &mut self,
        disposables: *mut crate::System::Collections::Generic::IList_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRange", (disposables))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn LazyInitializeDisposableList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyInitializeDisposableList", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        disposable: *mut crate::System::IDisposable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (disposable))?;
        Ok(__cordl_ret)
    }
    pub fn SpawnList_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<T> = __cordl_object
            .invoke("SpawnList", ())?;
        Ok(__cordl_ret)
    }
    pub fn SpawnList_IEnumerable_1_0<T>(
        &mut self,
        elements: *mut crate::System::Collections::Generic::IEnumerable_1<T>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<T> = __cordl_object
            .invoke("SpawnList", (elements))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_1_0<T>(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_1<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Spawn", (pool))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_2_TParam1_1<TValue, TParam1>(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_2<TParam1, TValue>,
        p1: TParam1,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Spawn", (pool, p1))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_3_TParam1_TParam2_2<TValue, TParam1, TParam2>(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue>,
        p1: TParam1,
        p2: TParam2,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Spawn", (pool, p1, p2))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_4_TParam1_TParam2_TParam3_3<
        TValue,
        TParam1,
        TParam2,
        TParam3,
    >(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_4<TParam1, TParam2, TParam3, TValue>,
        p1: TParam1,
        p2: TParam2,
        p3: TParam3,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Spawn", (pool, p1, p2, p3))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_5_TParam1_TParam2_TParam3_TParam4_4<
        TValue,
        TParam1,
        TParam2,
        TParam3,
        TParam4,
    >(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_5<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TValue,
        >,
        p1: TParam1,
        p2: TParam2,
        p3: TParam3,
        p4: TParam4,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke("Spawn", (pool, p1, p2, p3, p4))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_6_TParam1_TParam2_TParam3_TParam4_TParam5_5<
        TValue,
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
    >(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_6<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TValue,
        >,
        p1: TParam1,
        p2: TParam2,
        p3: TParam3,
        p4: TParam4,
        p5: TParam5,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke("Spawn", (pool, p1, p2, p3, p4, p5))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_7_TParam1_TParam2_TParam3_TParam4_TParam5_TParam6_6<
        TValue,
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
    >(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_7<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TValue,
        >,
        p1: TParam1,
        p2: TParam2,
        p3: TParam3,
        p4: TParam4,
        p5: TParam5,
        p6: TParam6,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke("Spawn", (pool, p1, p2, p3, p4, p5, p6))?;
        Ok(__cordl_ret)
    }
    pub fn Spawn_IMemoryPool_8_TParam1_TParam2_TParam3_TParam4_TParam5_TParam6_TParam7_7<
        TValue,
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
        TParam7,
    >(
        &mut self,
        pool: *mut crate::Zenject::IMemoryPool_8<
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
            TParam7,
            TValue,
        >,
        p1: TParam1,
        p2: TParam2,
        p3: TParam3,
        p4: TParam4,
        p5: TParam5,
        p6: TParam6,
        p7: TParam7,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object
            .invoke("Spawn", (pool, p1, p2, p3, p4, p5, p6, p7))?;
        Ok(__cordl_ret)
    }
    pub fn StoreSpawnedObject<T>(
        &mut self,
        obj: T,
        pool: *mut crate::Zenject::IDespawnableMemoryPool_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StoreSpawnedObject", (obj, pool))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DisposeBlock")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::DisposeBlock {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+DisposeBlock+SpawnedObjectPoolPair")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DisposeBlock_SpawnedObjectPoolPair {
    pub Pool: *mut crate::Zenject::IMemoryPool,
    pub Object: *mut crate::System::Object,
}
#[cfg(feature = "Zenject+DisposeBlock+SpawnedObjectPoolPair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DisposeBlock_SpawnedObjectPoolPair =>
    "Zenject"."DisposeBlock/SpawnedObjectPoolPair"
);
#[cfg(feature = "Zenject+DisposeBlock+SpawnedObjectPoolPair")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Zenject::DisposeBlock_SpawnedObjectPoolPair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Zenject+DisposeBlock+SpawnedObjectPoolPair")]
impl crate::Zenject::DisposeBlock_SpawnedObjectPoolPair {}
