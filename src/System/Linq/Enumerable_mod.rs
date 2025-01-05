#[cfg(feature = "System+Linq+Enumerable")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Enumerable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Enumerable => "System.Linq"
    ."Enumerable"
);
#[cfg(feature = "System+Linq+Enumerable")]
impl std::ops::Deref for crate::System::Linq::Enumerable {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn Aggregate_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        func: quest_hook::libil2cpp::Gc<TSource, TSource, TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Aggregate", (source, func))?;
        Ok(__cordl_ret.into())
    }
    pub fn Aggregate_TAccumulate_Gc1<TSource, TAccumulate>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        seed: TAccumulate,
        func: quest_hook::libil2cpp::Gc<TAccumulate, TSource, TAccumulate>,
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
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("All", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Any_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Any", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Any_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Any", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Average(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Average", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cast<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cast", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn CastIterator<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CastIterator", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombinePredicates<TSource>(
        predicate1: quest_hook::libil2cpp::Gc<TSource, bool>,
        predicate2: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource, bool>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource, bool> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombinePredicates", (predicate1, predicate2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineSelectors<TSource, TMiddle, TResult>(
        selector1: quest_hook::libil2cpp::Gc<TSource, TMiddle>,
        selector2: quest_hook::libil2cpp::Gc<TMiddle, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource, TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMiddle: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource, TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineSelectors", (selector1, selector2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Concat<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Concat", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConcatIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConcatIterator", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        value: TSource,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (source, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Gc_TSource0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
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
    pub fn Count_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Count", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Count_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Count", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultIfEmpty<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        defaultValue: TSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultIfEmpty", (source, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultIfEmptyIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        defaultValue: TSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultIfEmptyIterator", (source, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn Distinct<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Distinct", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn DistinctIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DistinctIterator", (source, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ElementAt<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
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
        quest_hook::libil2cpp::Gc<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Empty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Except<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Except", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExceptIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExceptIterator", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn FirstOrDefault_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FirstOrDefault", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn FirstOrDefault_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FirstOrDefault", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn First_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("First", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn First_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("First", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GroupBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<TKey, TSource>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<TKey, TSource>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GroupBy", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Intersect<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Intersect", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectIterator", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Last<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Last", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastOrDefault_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastOrDefault", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastOrDefault_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastOrDefault", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc0(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc1(
        source: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc2<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc3<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Gc4<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
    pub fn Min_Gc0(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_Gc1(
        source: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_Gc2<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_Gc3<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
    pub fn OfType<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OfType", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn OfTypeIterator<TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OfTypeIterator", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrderByDescending<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrderByDescending", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrderBy_Gc1<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
        comparer: quest_hook::libil2cpp::Gc<TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrderBy", (source, keySelector, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrderBy_Gc_Gc0<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrderBy", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Range(
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<i32>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Range", (start, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn RangeIterator(
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<i32>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RangeIterator", (start, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reverse<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reverse", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReverseIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReverseIterator", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectIterator<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectIterator", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectMany<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, quest_hook::libil2cpp::Gc<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectMany", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectManyIterator<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, quest_hook::libil2cpp::Gc<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectManyIterator", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Select_Gc_Gc0<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Select", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Select_Gc_Gc1<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Select", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_Gc1<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceEqual", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_Gc_Gc0<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceEqual", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn SingleOrDefault_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SingleOrDefault", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn SingleOrDefault_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SingleOrDefault", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Single_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Single", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Single_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Single", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Skip<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Skip", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SkipIterator", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sum_Gc0(
        source: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sum", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sum_Gc1(
        source: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sum", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sum_Gc2<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        selector: quest_hook::libil2cpp::Gc<TSource, i32>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sum", (source, selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Take<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Take", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn TakeIterator<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TakeIterator", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThenBy<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThenBy", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToArray<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
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
    pub fn ToDictionary_Gc1<TSource, TKey, TElement>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
        elementSelector: quest_hook::libil2cpp::Gc<TSource, TElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TKey, TElement>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TKey, TElement> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDictionary", (source, keySelector, elementSelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_Gc_Gc0<TSource, TKey>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TKey, TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TKey, TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDictionary", (source, keySelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDictionary_Gc_Gc2<TSource, TKey, TElement>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        keySelector: quest_hook::libil2cpp::Gc<TSource, TKey>,
        elementSelector: quest_hook::libil2cpp::Gc<TSource, TElement>,
        comparer: quest_hook::libil2cpp::Gc<TKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TKey, TElement>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TKey, TElement> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDictionary", (source, keySelector, elementSelector, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHashSet_Gc0<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHashSet", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHashSet_Gc1<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHashSet", (source, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToList<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToList", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Union<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Union", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnionIterator<TSource>(
        first: quest_hook::libil2cpp::Gc<TSource>,
        second: quest_hook::libil2cpp::Gc<TSource>,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnionIterator", (first, second, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Where", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Zip<TFirst, TSecond, TResult>(
        first: quest_hook::libil2cpp::Gc<TFirst>,
        second: quest_hook::libil2cpp::Gc<TSecond>,
        resultSelector: quest_hook::libil2cpp::Gc<TFirst, TSecond, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Zip", (first, second, resultSelector))?;
        Ok(__cordl_ret.into())
    }
    pub fn ZipIterator<TFirst, TSecond, TResult>(
        first: quest_hook::libil2cpp::Gc<TFirst>,
        second: quest_hook::libil2cpp::Gc<TSecond>,
        resultSelector: quest_hook::libil2cpp::Gc<TFirst, TSecond, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Clone", ())?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("GetEnumerator", ())?;
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
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Select", (selector))?;
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
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Where", (predicate))?;
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
impl<TSource: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<TSource: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<TSource>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+Iterator_1")]
impl<
    TSource: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::Linq::Enumerable_Iterator_1<TSource> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereArrayIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<TSource>,
    pub source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
    type Target = quest_hook::libil2cpp::Gc<TSource>;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Clone", ())?;
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
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
    __cordl_parent: quest_hook::libil2cpp::Gc<TSource>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub enumerator: quest_hook::libil2cpp::Gc<TSource>,
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
    type Target = quest_hook::libil2cpp::Gc<TSource>;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Clone", ())?;
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
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
    __cordl_parent: quest_hook::libil2cpp::Gc<TSource>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
    type Target = quest_hook::libil2cpp::Gc<TSource>;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Clone", ())?;
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
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TSource>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TSource> = __cordl_object
            .invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
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
    __cordl_parent: quest_hook::libil2cpp::Gc<TResult>,
    pub source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
    type Target = quest_hook::libil2cpp::Gc<TResult>;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Clone", ())?;
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
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
        selector: quest_hook::libil2cpp::Gc<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult2>>
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult2> = __cordl_object
            .invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TSource>>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
    __cordl_parent: quest_hook::libil2cpp::Gc<TResult>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
    pub enumerator: quest_hook::libil2cpp::Gc<TSource>,
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
    type Target = quest_hook::libil2cpp::Gc<TResult>;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Clone", ())?;
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
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
        selector: quest_hook::libil2cpp::Gc<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult2>>
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult2> = __cordl_object
            .invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
    __cordl_parent: quest_hook::libil2cpp::Gc<TResult>,
    pub source: quest_hook::libil2cpp::Gc<TSource>,
    pub predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
    pub selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
    type Target = quest_hook::libil2cpp::Gc<TResult>;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Clone", ())?;
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
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
        selector: quest_hook::libil2cpp::Gc<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult2>>
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult2> = __cordl_object
            .invoke("Select", (selector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = __cordl_object
            .invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<TSource>,
        predicate: quest_hook::libil2cpp::Gc<TSource, bool>,
        selector: quest_hook::libil2cpp::Gc<TSource, TResult>,
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
