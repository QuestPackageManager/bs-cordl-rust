#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::UnsafeUtility =>
    "Unity.Collections.LowLevel.Unsafe"."UnsafeUtility"
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility")]
impl std::ops::Deref for crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility")]
impl std::ops::DerefMut for crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility")]
impl crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility {
    #[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+AlignOfHelper_1")]
    pub type AlignOfHelper_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_AlignOfHelper_1<
        T,
    >;
    #[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+TypeFlagsCache_1")]
    pub type TypeFlagsCache_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_TypeFlagsCache_1<
        T,
    >;
    pub fn AddressOf<T>(
        output: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddressOf", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn AlignOf<T>() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignOf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn As<U, T>(
        from: quest_hook::libil2cpp::ByRefMut<U>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("As", (from))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsRef<T>(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsRef", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumEquals<T>(lhs: T, rhs: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnumEquals", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumToInt<T>(enumValue: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnumToInt", (enumValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn Free(
        memory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Free", (memory, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeTracked(
        memory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeTracked", (memory, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReasonForArrayNonBlittable(
        arr: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReasonForArrayNonBlittable", (arr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReasonForGenericListNonBlittable<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReasonForGenericListNonBlittable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReasonForTypeNonBlittableImpl(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReasonForTypeNonBlittableImpl", (t, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScriptingTypeFlags(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetScriptingTypeFlags", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalEnumToInt<T>(
        enumValue: quest_hook::libil2cpp::ByRefMut<T>,
        intValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalEnumToInt", (enumValue, intValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsArrayBlittable(
        arr: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsArrayBlittable", (arr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBlittableValueType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBlittableValueType", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBlittable_0<T>() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBlittable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBlittable_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBlittable", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericListBlittable<T>() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericListBlittable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUnmanaged<T>() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUnmanaged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LeakErase(
        handle: crate::System::IntPtr,
        category: crate::Unity::Collections::LeakCategory,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LeakErase", (handle, category))?;
        Ok(__cordl_ret.into())
    }
    pub fn LeakRecord(
        handle: crate::System::IntPtr,
        category: crate::Unity::Collections::LeakCategory,
        callstacksToSkip: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LeakRecord", (handle, category, callstacksToSkip))?;
        Ok(__cordl_ret.into())
    }
    pub fn Malloc(
        _cordl_size: i64,
        alignment: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Malloc", (_cordl_size, alignment, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn MallocTracked(
        _cordl_size: i64,
        alignment: i32,
        allocator: crate::Unity::Collections::Allocator,
        callstacksToSkip: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MallocTracked",
                (_cordl_size, alignment, allocator, callstacksToSkip),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MemClear(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemClear", (destination, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemCmp(
        ptr1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ptr2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemCmp", (ptr1, ptr2, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemCpy(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemCpy", (destination, source, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemCpyStride(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destinationStride: i32,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        sourceStride: i32,
        elementSize: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MemCpyStride",
                (
                    destination,
                    destinationStride,
                    source,
                    sourceStride,
                    elementSize,
                    count,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MemMove(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemMove", (destination, source, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemSet(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u8,
        _cordl_size: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemSet", (destination, value, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadArrayElement<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadArrayElement", (source, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadArrayElementWithStride<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        stride: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadArrayElementWithStride", (source, index, stride))?;
        Ok(__cordl_ret.into())
    }
    pub fn SizeOf<T>() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SizeOf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteArrayElement<T>(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteArrayElement", (destination, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteArrayElementWithStride<T>(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        stride: i32,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteArrayElementWithStride", (destination, index, stride, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+AlignOfHelper_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnsafeUtility_AlignOfHelper_1<T: quest_hook::libil2cpp::Type> {
    pub dummy: u8,
    pub data: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+AlignOfHelper_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_AlignOfHelper_1 < T > =>
    "Unity.Collections.LowLevel.Unsafe"."UnsafeUtility/AlignOfHelper`1<T>" < T >
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+AlignOfHelper_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_AlignOfHelper_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+AlignOfHelper_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_AlignOfHelper_1<T> {}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+TypeFlagsCache_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnsafeUtility_TypeFlagsCache_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+TypeFlagsCache_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_TypeFlagsCache_1 < T > =>
    "Unity.Collections.LowLevel.Unsafe"."UnsafeUtility/TypeFlagsCache`1<T>" < T >
);
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+TypeFlagsCache_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_TypeFlagsCache_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeUtility+TypeFlagsCache_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Collections::LowLevel::Unsafe::UnsafeUtility_TypeFlagsCache_1<T> {
    pub fn Init(
        flags: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Init", (flags))?;
        Ok(__cordl_ret.into())
    }
}
