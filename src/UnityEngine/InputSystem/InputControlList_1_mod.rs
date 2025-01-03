#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlList_1<TControl: quest_hook::libil2cpp::Type> {
    pub m_Count: i32,
    pub m_Indices: crate::Unity::Collections::NativeArray_1<u64>,
    pub m_Allocator: crate::Unity::Collections::Allocator,
    __cordl_phantom_TControl: std::marker::PhantomData<TControl>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputControlList_1 <
    TControl > => "UnityEngine.InputSystem"."InputControlList`1<TControl>" < TControl >
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    pub const kInvalidIndex: u64 = 18446744073709551615u64;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<
        TControl,
    >;
    pub fn Add(
        &mut self,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddRange(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TControl>,
        >,
        count: i32,
        destinationIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddRange",
            (list, count, destinationIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSlice<TList>(
        &mut self,
        list: TList,
        count: i32,
        destinationIndex: i32,
        sourceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddSlice",
            (list, count, destinationIndex, sourceIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendTo(
        &mut self,
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<TControl>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AppendTo",
            (array, count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_TControl0(
        &mut self,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_i32_i32_1(
        &mut self,
        item: TControl,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (item, startIndex, count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyTo",
            (array, arrayIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromIndex(index: u64) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromIndex", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TControl>,
        >,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TControl>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_TControl0(
        &mut self,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IndexOf",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_i32_i32_1(
        &mut self,
        item: TControl,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IndexOf",
            (item, startIndex, count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Insert(
        &mut self,
        index: i32,
        item: TControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Insert",
            (index, item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(&mut self, item: TControl) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveAt",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Resize",
            (_cordl_size),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort<TCompare>(
        &mut self,
        startIndex: i32,
        count: i32,
        comparer: TCompare,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCompare: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Sort",
            (startIndex, count, comparer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapElements(
        &mut self,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SwapElements",
            (index1, index2),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn ToArray(
        &mut self,
        dispose: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TControl>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToArray", (dispose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIndex(control: TControl) -> quest_hook::libil2cpp::Result<u64>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToIndex", (control))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Allocator_i32_0(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
        initialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (allocator, initialCapacity),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_Allocator1(
        &mut self,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TControl>,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (values, allocator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TControl>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (values),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Capacity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsReadOnly",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Capacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Capacity",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: TControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::ICollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::ICollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::ICollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::ICollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyCollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyCollection_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyList_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyList_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<TControl: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1")]
impl<TControl: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlList_1_Enumerator<TControl: quest_hook::libil2cpp::Type> {
    pub m_Indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Count: i32,
    pub m_Current: i32,
    __cordl_phantom_TControl: std::marker::PhantomData<TControl>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlList_1_Enumerator < TControl > =>
    "UnityEngine.InputSystem"."InputControlList`1/Enumerator<TControl>" < TControl >
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
unsafe impl<TControl: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
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
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        list: crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (list),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<TControl>>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<TControl> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<
    TControl: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<TControl: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlList_1+Enumerator")]
impl<TControl: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlList_1_Enumerator<TControl> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
