#[cfg(feature = "Zenject+DisposeBlock")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposeBlock {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _disposables: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
        >,
    >,
    pub _objectPoolPairs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::Zenject::DisposeBlock_SpawnedObjectPoolPair,
        >,
    >,
}
#[cfg(feature = "Zenject+DisposeBlock")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::DisposeBlock {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "DisposeBlock";
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
#[cfg(feature = "Zenject+DisposeBlock")]
impl std::ops::Deref for crate::Zenject::DisposeBlock {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        disposable: quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (disposable))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddRange<T>(
        &mut self,
        disposables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<T>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LazyInitializeDisposableList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyInitializeDisposableList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDespawned(
        that: quest_hook::libil2cpp::Gc<crate::Zenject::DisposeBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnDespawned", (that))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpawned(
        that: quest_hook::libil2cpp::Gc<crate::Zenject::DisposeBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSpawned", (that))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        disposable: quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (disposable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnList_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = __cordl_object.invoke("SpawnList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnList_IEnumerable_1_0<T>(
        &mut self,
        elements: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = __cordl_object.invoke("SpawnList", (elements))?;
        Ok(__cordl_ret.into())
    }
    pub fn Spawn_8() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DisposeBlock>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DisposeBlock> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Spawn", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Spawn_IMemoryPool_1_0<T>(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Spawn", (pool))?;
        Ok(__cordl_ret.into())
    }
    pub fn Spawn_IMemoryPool_2_TParam1_1<TValue, TParam1>(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool_2<TParam1, TValue>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Spawn_IMemoryPool_3_TParam1_TParam2_2<TValue, TParam1, TParam2>(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<
            crate::Zenject::IMemoryPool_3<TParam1, TParam2, TValue>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Spawn_IMemoryPool_4_TParam1_TParam2_TParam3_3<
        TValue,
        TParam1,
        TParam2,
        TParam3,
    >(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<
            crate::Zenject::IMemoryPool_4<TParam1, TParam2, TParam3, TValue>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Spawn_IMemoryPool_5_TParam1_TParam2_TParam3_TParam4_4<
        TValue,
        TParam1,
        TParam2,
        TParam3,
        TParam4,
    >(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<
            crate::Zenject::IMemoryPool_5<TParam1, TParam2, TParam3, TParam4, TValue>,
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
        Ok(__cordl_ret.into())
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
        pool: quest_hook::libil2cpp::Gc<
            crate::Zenject::IMemoryPool_6<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TParam5,
                TValue,
            >,
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
        Ok(__cordl_ret.into())
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
        pool: quest_hook::libil2cpp::Gc<
            crate::Zenject::IMemoryPool_7<
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TParam5,
                TParam6,
                TValue,
            >,
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
        Ok(__cordl_ret.into())
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
        pool: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_ret.into())
    }
    pub fn StoreSpawnedObject<T>(
        &mut self,
        obj: T,
        pool: quest_hook::libil2cpp::Gc<crate::Zenject::IDespawnableMemoryPool_1<T>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Zenject+DisposeBlock")]
impl AsRef<crate::System::IDisposable> for crate::Zenject::DisposeBlock {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+DisposeBlock")]
impl AsMut<crate::System::IDisposable> for crate::Zenject::DisposeBlock {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+DisposeBlock+SpawnedObjectPoolPair")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisposeBlock_SpawnedObjectPoolPair {
    pub Pool: quest_hook::libil2cpp::Gc<crate::Zenject::IMemoryPool>,
    pub Object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Zenject+DisposeBlock+SpawnedObjectPoolPair")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::DisposeBlock_SpawnedObjectPoolPair {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "SpawnedObjectPoolPair";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::Zenject::DisposeBlock_SpawnedObjectPoolPair {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Zenject::DisposeBlock_SpawnedObjectPoolPair {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::Zenject::DisposeBlock_SpawnedObjectPoolPair {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::Zenject::DisposeBlock_SpawnedObjectPoolPair {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
