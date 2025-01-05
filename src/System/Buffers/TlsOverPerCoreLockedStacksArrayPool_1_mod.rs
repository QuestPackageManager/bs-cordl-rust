#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsOverPerCoreLockedStacksArrayPool_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Buffers::ArrayPool_1<T>,
    pub _bucketArraySizes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _buckets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
                T,
            >,
        >,
    >,
    pub _callbackCreated: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1 < T > => "System.Buffers"
    ."TlsOverPerCoreLockedStacksArrayPool`1" < T >
);
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1<T> {
    type Target = crate::System::Buffers::ArrayPool_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1<T> {
    #[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+LockedStack")]
    pub type LockedStack = crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_LockedStack<
        T,
    >;
    #[cfg(
        feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+MemoryPressure"
    )]
    pub type MemoryPressure = crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_MemoryPressure;
    #[cfg(
        feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+PerCoreLockedStacks"
    )]
    pub type PerCoreLockedStacks = crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
        T,
    >;
    pub fn CreatePerCoreLockedStacks(
        &mut self,
        bucketIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
                T,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
                T,
            >,
        > = __cordl_object.invoke("CreatePerCoreLockedStacks", (bucketIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Gen2GcCallbackFunc(
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Gen2GcCallbackFunc", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemoryPressure() -> quest_hook::libil2cpp::Result<
        crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_MemoryPressure<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_MemoryPressure<
            T,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMemoryPressure", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrimBuffers() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrimBuffers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Rent(
        &mut self,
        minimumLength: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = __cordl_object.invoke("Rent", (minimumLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn Return(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        clearArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Return", (array, clearArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn Trim(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Trim", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Id(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Id", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+LockedStack")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsOverPerCoreLockedStacksArrayPool_1_LockedStack<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _arrays: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppArray<T>>,
    >,
    pub _count: i32,
    pub _firstStackItemMS: u32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+LockedStack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_LockedStack < T > =>
    "System.Buffers"."TlsOverPerCoreLockedStacksArrayPool`1/LockedStack" < T >
);
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+LockedStack")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_LockedStack<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+LockedStack")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_LockedStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+LockedStack")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_LockedStack<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Trim(
        &mut self,
        tickCount: u32,
        id: i32,
        pressure: crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_MemoryPressure<
            T,
        >,
        bucketSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Trim", (tickCount, id, pressure, bucketSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryPop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = __cordl_object.invoke("TryPop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryPush(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryPush", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+LockedStack")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_LockedStack<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+MemoryPressure")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TlsOverPerCoreLockedStacksArrayPool_1_MemoryPressure {
    #[default]
    High = 2i32,
    Low = 0i32,
    Medium = 1i32,
}
#[cfg(feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+MemoryPressure")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_MemoryPressure =>
    "System.Buffers"."TlsOverPerCoreLockedStacksArrayPool`1/MemoryPressure<T>"
);
#[cfg(
    feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+PerCoreLockedStacks"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _perCoreStacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_LockedStack<
                T,
            >,
        >,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+PerCoreLockedStacks"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks < T > =>
    "System.Buffers"."TlsOverPerCoreLockedStacksArrayPool`1/PerCoreLockedStacks" < T >
);
#[cfg(
    feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+PerCoreLockedStacks"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
    T,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+PerCoreLockedStacks"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
    T,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+PerCoreLockedStacks"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Trim(
        &mut self,
        tickCount: u32,
        id: i32,
        pressure: crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_MemoryPressure<
            T,
        >,
        bucketSizes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Trim", (tickCount, id, pressure, bucketSizes))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryPop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = __cordl_object.invoke("TryPop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryPush(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryPush", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(
    feature = "System+Buffers+TlsOverPerCoreLockedStacksArrayPool_1+PerCoreLockedStacks"
)]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Buffers::TlsOverPerCoreLockedStacksArrayPool_1_PerCoreLockedStacks<
    T,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
