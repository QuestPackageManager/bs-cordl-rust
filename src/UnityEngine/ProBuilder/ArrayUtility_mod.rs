#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ArrayUtility =>
    "UnityEngine.ProBuilder"."ArrayUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ArrayUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ArrayUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl crate::UnityEngine::ProBuilder::ArrayUtility {
    #[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
    pub type SearchRange = crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange;
    pub fn Add<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Add", (arr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddOrAppend<T, K>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<K>>,
            >,
        >,
        key: T,
        value: K,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddOrAppend", (dictionary, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddOrAppendRange<T, K>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                T,
                quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<K>>,
            >,
        >,
        key: T,
        value: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<K>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddOrAppendRange", (dictionary, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddRange<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddRange", (arr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllIndexesOf<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        lambda: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllIndexesOf", (list, lambda))?;
        Ok(__cordl_ret.into())
    }
    pub fn Concat<T>(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Concat", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsMatch_ByRefMut_ByRefMut1<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index_a: quest_hook::libil2cpp::ByRefMut<i32>,
        index_b: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsMatch", (a, b, index_a, index_b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsMatch_Il2CppArray_Il2CppArray0<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsMatch", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn DistinctBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DistinctBy", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fill_Func_2_i32_0<T>(
        ctor: quest_hook::libil2cpp::Gc<crate::System::Func_2<i32, T>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fill", (ctor, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fill_T_i32_1<T>(
        val: T,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fill", (val, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fill_i32_Func_2_2<T>(
        count: i32,
        ctor: quest_hook::libil2cpp::Gc<crate::System::Func_2<i32, T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fill", (count, ctor))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        InList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
            >,
        >,
        InValue: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (InList, InValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn NearestIndexPriorToValue<T>(
        sorted_list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<T>,
        >,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NearestIndexPriorToValue", (sorted_list, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt_IList_1_IEnumerable_1_1<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveAt", (list, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt_Il2CppArray_i32_0<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveAt", (arr, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_IEnumerable_1_1<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Remove", (arr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_T0<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        val: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Remove", (arr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortedRemoveAt<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        sorted: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortedRemoveAt", (list, sorted))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Dictionary_2_0<TKey, TValue>(
        dict: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (dict))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_IEnumerable_1_Il2CppString1<T>(
        arr: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (arr, separator))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValuesWithIndexes_Il2CppArray_Il2CppArray0<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValuesWithIndexes", (arr, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValuesWithIndexes_List_1_IList_1_1<T>(
        arr: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValuesWithIndexes", (arr, indexes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ArrayUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArrayUtility_SearchRange {
    pub begin: i32,
    pub end: i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::ArrayUtility_SearchRange => "UnityEngine.ProBuilder"
    ."ArrayUtility/SearchRange"
);
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ArrayUtility+SearchRange")]
impl crate::UnityEngine::ProBuilder::ArrayUtility_SearchRange {
    pub fn Center(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Center",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        begin: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (begin, end),
        )?;
        Ok(__cordl_ret.into())
    }
}
