#[cfg(feature = "System+Linq+Enumerable")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Enumerable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Enumerable => "System.Linq"
    ."Enumerable"
);
#[cfg(feature = "System+Linq+Enumerable")]
impl std::ops::Deref for crate::System::Linq::Enumerable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable")]
impl std::ops::DerefMut for crate::System::Linq::Enumerable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable")]
impl crate::System::Linq::Enumerable {
    #[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
    pub type Iterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_Iterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
    pub type WhereArrayIterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_WhereArrayIterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
    pub type WhereEnumerableIterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_WhereEnumerableIterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
    pub type WhereListIterator_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable_WhereListIterator_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
    pub type WhereSelectArrayIterator_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult>;
    #[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
    pub type WhereSelectEnumerableIterator_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<
        TSource,
        TResult,
    >;
    #[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
    pub type WhereSelectListIterator_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult>;
    pub fn Aggregate_Func_3_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        func: quest_hook::libil2cpp::Gc<crate::System::Func_3<TSource, TSource, TSource>>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Aggregate", (source, func))?;
        Ok(__cordl_ret.into())
    }
    pub fn Aggregate_TAccumulate_Func_3_1<TSource, TAccumulate>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        seed: TAccumulate,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<TAccumulate, TSource, TAccumulate>,
        >,
    ) -> quest_hook::libil2cpp::Result<TAccumulate>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TAccumulate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TAccumulate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Aggregate", (source, seed, func))?;
        Ok(__cordl_ret.into())
    }
    pub fn All<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("All", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Any_Func_2_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Any", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Any_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Any", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Average(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Average", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cast<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Cast", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn CastIterator<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CastIterator", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombinePredicates<TSource>(
        predicate1: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
        predicate2: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<TSource, bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombinePredicates", (predicate1, predicate2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineSelectors<TSource, TMiddle, TResult>(
        selector1: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TMiddle>>,
        selector2: quest_hook::libil2cpp::Gc<crate::System::Func_2<TMiddle, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMiddle: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<TSource, TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineSelectors", (selector1, selector2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Concat<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Concat", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConcatIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConcatIterator", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_IEnumerable_1_TSource0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        value: TSource,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (source, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_IEqualityComparer_1_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        value: TSource,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (source, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Count_Func_2_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Count", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Count_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Count", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultIfEmpty<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        defaultValue: TSource,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultIfEmpty", (source, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultIfEmptyIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        defaultValue: TSource,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultIfEmptyIterator", (source, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn Distinct<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Distinct", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn DistinctIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DistinctIterator", (source, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ElementAt<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ElementAt", (source, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn Empty<TResult>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Empty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Except<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Except", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExceptIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExceptIterator", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn FirstOrDefault_Func_2_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FirstOrDefault", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn FirstOrDefault_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FirstOrDefault", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn First_Func_2_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("First", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn First_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("First", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn GroupBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::IGrouping_2<TKey, TSource>,
                >,
            >,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::IGrouping_2<TKey, TSource>,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GroupBy", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Intersect<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Intersect", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectIterator", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Last<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Last", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastOrDefault_Func_2_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastOrDefault", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastOrDefault_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastOrDefault", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Func_2_3<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, i32>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Func_2_4<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_IEnumerable_1_0(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_IEnumerable_1_1(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_IEnumerable_1_2<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_Func_2_3<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_IEnumerable_1_0(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_IEnumerable_1_1(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_IEnumerable_1_2<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn OfType<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("OfType", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn OfTypeIterator<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OfTypeIterator", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrderByDescending<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IOrderedEnumerable_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrderByDescending", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrderBy_IComparer_1_1<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IOrderedEnumerable_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrderBy", (source, keySelector, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrderBy_IEnumerable_1_Func_2_0<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IOrderedEnumerable_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrderBy", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Range(
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Range", (start, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn RangeIterator(
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RangeIterator", (start, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reverse<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Reverse", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReverseIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReverseIterator", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectIterator<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_3<TSource, i32, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectIterator", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectMany<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TSource,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<TResult>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectMany", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectManyIterator<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TSource,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<TResult>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectManyIterator", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Select_Func_2_0<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Select", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Select_Func_3_1<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_3<TSource, i32, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Select", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_IEnumerable_1_IEnumerable_1_0<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceEqual", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_IEqualityComparer_1_1<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceEqual", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SingleOrDefault_Func_2_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SingleOrDefault", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn SingleOrDefault_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SingleOrDefault", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Single_Func_2_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Single", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Single_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Single", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Skip<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Skip", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SkipIterator", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sum_Func_2_2<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, i32>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sum", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sum_IEnumerable_1_0(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sum", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sum_IEnumerable_1_1(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sum", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Take<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Take", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn TakeIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TakeIterator", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThenBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IOrderedEnumerable_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThenBy", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToArray<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToArray", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_Func_2_1<TSource, TKey, TElement>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
        elementSelector: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<TSource, TElement>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TElement>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TElement>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDictionary", (source, keySelector, elementSelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_Func_2_IEqualityComparer_1_2<TSource, TKey, TElement>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
        elementSelector: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<TSource, TElement>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TElement>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TElement>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDictionary", (source, keySelector, elementSelector, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_IEnumerable_1_Func_2_0<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<TKey, TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDictionary", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHashSet_IEnumerable_1_0<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHashSet", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHashSet_IEqualityComparer_1_1<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHashSet", (source, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToList<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToList", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Union<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Union", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnionIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnionIterator", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Where", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Zip<TFirst, TSecond, TResult>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TFirst>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSecond>,
        >,
        resultSelector: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<TFirst, TSecond, TResult>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Zip", (first, second, resultSelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ZipIterator<TFirst, TSecond, TResult>(
        first: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TFirst>,
        >,
        second: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSecond>,
        >,
        resultSelector: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<TFirst, TSecond, TResult>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZipIterator", (first, second, resultSelector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Enumerable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_Iterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub threadId: i32,
    pub state: i32,
    pub current: TSource,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Enumerable_Iterator_1 < TSource >
    => "System.Linq"."Enumerable/Iterator`1" < TSource >
);
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_Iterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Enumerable_Iterator_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Enumerable_Iterator_1<TSource>,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TSource>,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IEnumerator.Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TSource = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereArrayIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Linq::Enumerable_Iterator_1<TSource>,
    pub source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    pub predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    pub index: i32,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Enumerable_WhereArrayIterator_1 <
    TSource > => "System.Linq"."Enumerable/WhereArrayIterator`1" < TSource >
);
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    type Target = crate::System::Linq::Enumerable_Iterator_1<TSource>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Enumerable_Iterator_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Enumerable_Iterator_1<TSource>,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereArrayIterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereEnumerableIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Linq::Enumerable_Iterator_1<TSource>,
    pub source: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<TSource>,
    >,
    pub predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    pub enumerator: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerator_1<TSource>,
    >,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Enumerable_WhereEnumerableIterator_1 < TSource > => "System.Linq"
    ."Enumerable/WhereEnumerableIterator`1" < TSource >
);
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    type Target = crate::System::Linq::Enumerable_Iterator_1<TSource>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Enumerable_Iterator_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Enumerable_Iterator_1<TSource>,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereEnumerableIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereEnumerableIterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereListIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Linq::Enumerable_Iterator_1<TSource>,
    pub source: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<TSource>,
    >,
    pub predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    pub enumerator: crate::System::Collections::Generic::List_1_Enumerator<TSource>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
}
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Enumerable_WhereListIterator_1 <
    TSource > => "System.Linq"."Enumerable/WhereListIterator`1" < TSource >
);
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    type Target = crate::System::Linq::Enumerable_Iterator_1<TSource>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Enumerable_Iterator_1<TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Enumerable_Iterator_1<TSource>,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereListIterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereListIterator_1<TSource> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereSelectArrayIterator_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Linq::Enumerable_Iterator_1<TResult>,
    pub source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    pub predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    pub selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    pub index: i32,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Enumerable_WhereSelectArrayIterator_2 < TSource, TResult > =>
    "System.Linq"."Enumerable/WhereSelectArrayIterator`2" < TSource, TResult >
);
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    type Target = crate::System::Linq::Enumerable_Iterator_1<TResult>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Enumerable_Iterator_1<TResult>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Enumerable_Iterator_1<TResult>,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate, selector))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TResult, TResult2>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult2>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult2>,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TResult, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, predicate, selector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectArrayIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereSelectArrayIterator_2<TSource, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereSelectEnumerableIterator_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Linq::Enumerable_Iterator_1<TResult>,
    pub source: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<TSource>,
    >,
    pub predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    pub selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    pub enumerator: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerator_1<TSource>,
    >,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Enumerable_WhereSelectEnumerableIterator_2 < TSource, TResult > =>
    "System.Linq"."Enumerable/WhereSelectEnumerableIterator`2" < TSource, TResult >
);
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    type Target = crate::System::Linq::Enumerable_Iterator_1<TResult>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Enumerable_Iterator_1<TResult>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Enumerable_Iterator_1<TResult>,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate, selector))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TResult, TResult2>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult2>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult2>,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TResult, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, predicate, selector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectEnumerableIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereSelectEnumerableIterator_2<TSource, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereSelectListIterator_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Linq::Enumerable_Iterator_1<TResult>,
    pub source: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<TSource>,
    >,
    pub predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
    pub selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    pub enumerator: crate::System::Collections::Generic::List_1_Enumerator<TSource>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Enumerable_WhereSelectListIterator_2 < TSource, TResult > =>
    "System.Linq"."Enumerable/WhereSelectListIterator`2" < TSource, TResult >
);
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    type Target = crate::System::Linq::Enumerable_Iterator_1<TResult>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Enumerable_Iterator_1<TResult>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Enumerable_Iterator_1<TResult>,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate, selector))?;
        Ok(__cordl_object.into())
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TResult, TResult2>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult2>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult2>,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TResult, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TResult>,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TSource>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, bool>>,
        selector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, predicate, selector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereSelectListIterator_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Enumerable_WhereSelectListIterator_2<TSource, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
