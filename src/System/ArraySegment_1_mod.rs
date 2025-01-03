#[cfg(feature = "System+ArraySegment_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArraySegment_1<T: quest_hook::libil2cpp::Type> {
    pub _array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    pub _offset: i32,
    pub _count: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+ArraySegment_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ArraySegment_1 < T > => "System"
    ."ArraySegment`1<T>" < T >
);
#[cfg(feature = "System+ArraySegment_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::ArraySegment_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::ArraySegment_1<T> {
    #[cfg(feature = "System+ArraySegment_1+Enumerator")]
    pub type Enumerator = crate::System::ArraySegment_1_Enumerator<T>;
    pub fn CopyTo(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        destinationIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyTo",
            (destination, destinationIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ArraySegment_1_1(
        &mut self,
        obj: crate::System::ArraySegment_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ArraySegment_1_Enumerator<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::ArraySegment_1_Enumerator<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_ICollection_T__Add(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.ICollection<T>.Add",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_ICollection_T__Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.ICollection<T>.Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_ICollection_T__Contains(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.ICollection<T>.Contains",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_ICollection_T__Remove(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.ICollection<T>.Remove",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_ICollection_T__get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.ICollection<T>.get_IsReadOnly",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_T__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<T>.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IList_T__IndexOf(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IList<T>.IndexOf",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IList_T__Insert(
        &mut self,
        index: i32,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IList<T>.Insert",
            (index, item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IList_T__RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IList<T>.RemoveAt",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IList_T__get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IList<T>.get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IList_T__set_Item(
        &mut self,
        index: i32,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IList<T>.set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IReadOnlyList_T__get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IReadOnlyList<T>.get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationIfDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ThrowInvalidOperationIfDefault",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (array),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_1(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (array, offset, count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Array(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Array", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Empty() -> quest_hook::libil2cpp::Result<crate::System::ArraySegment_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::ArraySegment_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Empty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Offset(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Offset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::ICollection_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::ICollection_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::ICollection_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::ICollection_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IList_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IList_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IList_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IList_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyCollection_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyCollection_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyList_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyList_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyList_1<T>>
for crate::System::ArraySegment_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyList_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::System::ArraySegment_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::System::ArraySegment_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArraySegment_1_Enumerator<T: quest_hook::libil2cpp::Type> {
    pub _array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    pub _start: i32,
    pub _end: i32,
    pub _current: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ArraySegment_1_Enumerator < T > =>
    "System"."ArraySegment`1/Enumerator<T>" < T >
);
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::ArraySegment_1_Enumerator<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> crate::System::ArraySegment_1_Enumerator<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        arraySegment: crate::System::ArraySegment_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (arraySegment),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::ArraySegment_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::ArraySegment_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::System::ArraySegment_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::System::ArraySegment_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::System::ArraySegment_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+ArraySegment_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::System::ArraySegment_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
