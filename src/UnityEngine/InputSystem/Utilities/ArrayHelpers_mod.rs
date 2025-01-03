#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::ArrayHelpers =>
    "UnityEngine.InputSystem.Utilities"."ArrayHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    pub fn AppendListWithCapacity<TValue, TValues>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        values: TValues,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValues: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AppendListWithCapacity",
                (array, length, values, capacityIncrement),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendToImmutable<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendToImmutable", (array, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendWithCapacity_Allocator1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        value: TValue,
        capacityIncrement: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AppendWithCapacity",
                (array, count, value, capacityIncrement, allocator),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendWithCapacity_ByRefMut_ByRefMut_TValue_i32_0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        value: TValue,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendWithCapacity", (array, count, value, capacityIncrement))?;
        Ok(__cordl_ret.into())
    }
    pub fn Append_IEnumerable_1_1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Append", (array, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn Append_TValue0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Append", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear_ByRefMut2<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", (array, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear_Il2CppArray0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear_i32_1<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", (array, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Clone", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsReference_TValue0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsReference", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsReference_i32_TSecond1<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        count: i32,
        value: TSecond,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsReference", (array, count, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsReference_i32_i32_TSecond2<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        startIndex: i32,
        count: i32,
        value: TSecond,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsReference", (array, startIndex, count, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Copy", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn DuplicateWithCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        count: i32,
        capacity: i32,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DuplicateWithCapacity",
                (array, count, capacity, capacityIncrement),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        count: i32,
        capacity: i32,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureCapacity", (array, count, capacity, capacityIncrement))?;
        Ok(__cordl_ret.into())
    }
    pub fn Erase<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Erase", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseAt<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EraseAt", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseAtByMovingTail<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EraseAtByMovingTail", (array, count, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseAtWithCapacity_Il2CppArray0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EraseAtWithCapacity", (array, count, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseAtWithCapacity_NativeArray_1_1<TValue>(
        array: crate::Unity::Collections::NativeArray_1<TValue>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EraseAtWithCapacity", (array, count, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseSliceWithCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EraseSliceWithCapacity", (array, length, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrowBy_Allocator1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        count: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GrowBy", (array, count, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrowBy_ByRefMut_i32_0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GrowBy", (array, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrowWithCapacity_Allocator1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        growBy: i32,
        capacityIncrement: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GrowWithCapacity",
                (array, count, growBy, capacityIncrement, allocator),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GrowWithCapacity_ByRefMut_ByRefMut_i32_i32_0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        growBy: i32,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GrowWithCapacity", (array, count, growBy, capacityIncrement))?;
        Ok(__cordl_ret.into())
    }
    pub fn HaveDuplicateReferences<TFirst>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HaveDuplicateReferences", (first, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn HaveEqualElements<TValue>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HaveEqualElements", (first, second, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference_Il2CppArray_TSecond_i32_0<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        value: TSecond,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfReference", (array, value, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference_i32_1<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        value: TSecond,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfReference", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfValue<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        value: TValue,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfValue", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Predicate_1_1<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Predicate_1_i32_i32_2<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<TValue>>,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, predicate, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_TValue_i32_i32_0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        value: TValue,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertAt<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        index: i32,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertAt", (array, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertAtWithCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
        value: TValue,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InsertAtWithCapacity",
                (array, count, index, value, capacityIncrement),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Join<TValue>(
        value: TValue,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Join", (value, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn LengthSafe<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LengthSafe", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Merge_IEqualityComparer_1_1<TValue>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Merge", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Merge_Il2CppArray_Il2CppArray0<TValue>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Merge", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveSlice<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        sourceIndex: i32,
        destinationIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveSlice", (array, sourceIndex, destinationIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn PutAtIfNotSet<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
        >,
        index: i32,
        valueFn: quest_hook::libil2cpp::Gc<crate::System::Func_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PutAtIfNotSet", (array, index, valueFn))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        newSize: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Resize", (array, newSize, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn Select<TOld, TNew>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TOld>>,
        converter: quest_hook::libil2cpp::Gc<crate::System::Func_2<TOld, TNew>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TNew>>,
    >
    where
        TOld: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TNew: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TNew>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Select", (array, converter))?;
        Ok(__cordl_ret.into())
    }
    pub fn Swap<TValue>(
        first: quest_hook::libil2cpp::ByRefMut<TValue>,
        second: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Swap", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapElements_Il2CppArray0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwapElements", (array, index1, index2))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapElements_NativeArray_1_1<TValue>(
        array: crate::Unity::Collections::NativeArray_1<TValue>,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwapElements", (array, index1, index2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
