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
    #[cfg(feature = "System+Linq+Enumerable+_CastIterator_d__99_1")]
    pub type _CastIterator_d__99_1<TResult: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__CastIterator_d__99_1<
        TResult,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_ConcatIterator_d__59_1")]
    pub type _ConcatIterator_d__59_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__ConcatIterator_d__59_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_DefaultIfEmptyIterator_d__95_1")]
    pub type _DefaultIfEmptyIterator_d__95_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__DefaultIfEmptyIterator_d__95_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_DistinctIterator_d__68_1")]
    pub type _DistinctIterator_d__68_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__DistinctIterator_d__68_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_ExceptIterator_d__77_1")]
    pub type _ExceptIterator_d__77_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__ExceptIterator_d__77_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_IntersectIterator_d__74_1")]
    pub type _IntersectIterator_d__74_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__IntersectIterator_d__74_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_OfTypeIterator_d__97_1")]
    pub type _OfTypeIterator_d__97_1<TResult: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__OfTypeIterator_d__97_1<
        TResult,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_RangeIterator_d__115")]
    pub type _RangeIterator_d__115 = crate::System::Linq::Enumerable__RangeIterator_d__115;
    #[cfg(feature = "System+Linq+Enumerable+_ReverseIterator_d__79_1")]
    pub type _ReverseIterator_d__79_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__ReverseIterator_d__79_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_SelectIterator_d__5_2")]
    pub type _SelectIterator_d__5_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable__SelectIterator_d__5_2<TSource, TResult>;
    #[cfg(feature = "System+Linq+Enumerable+_SelectManyIterator_d__17_2")]
    pub type _SelectManyIterator_d__17_2<
        TSource: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable__SelectManyIterator_d__17_2<TSource, TResult>;
    #[cfg(feature = "System+Linq+Enumerable+_SkipIterator_d__31_1")]
    pub type _SkipIterator_d__31_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__SkipIterator_d__31_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_TakeIterator_d__25_1")]
    pub type _TakeIterator_d__25_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__TakeIterator_d__25_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_UnionIterator_d__71_1")]
    pub type _UnionIterator_d__71_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable__UnionIterator_d__71_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+_ZipIterator_d__65_3")]
    pub type _ZipIterator_d__65_3<
        TFirst: quest_hook::libil2cpp::Type,
        TSecond: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable__ZipIterator_d__65_3<TFirst, TSecond, TResult>;
    #[cfg(feature = "System+Linq+Enumerable+__c__DisplayClass6_0_1")]
    pub type __c__DisplayClass6_0_1<TSource: quest_hook::libil2cpp::Type> = crate::System::Linq::Enumerable___c__DisplayClass6_0_1<
        TSource,
    >;
    #[cfg(feature = "System+Linq+Enumerable+__c__DisplayClass7_0_3")]
    pub type __c__DisplayClass7_0_3<
        TSource: quest_hook::libil2cpp::Type,
        TMiddle: quest_hook::libil2cpp::Type,
        TResult: quest_hook::libil2cpp::Type,
    > = crate::System::Linq::Enumerable___c__DisplayClass7_0_3<
        TSource,
        TMiddle,
        TResult,
    >;
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
        *mut crate::System::Linq::Enumerable_Iterator_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Enumerable_Iterator_1<TSource> = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            TSource,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Select<TResult>(
        &mut self,
        selector: *mut crate::System::Func_2<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret)
    }
    pub fn Where(
        &mut self,
        predicate: *mut crate::System::Func_2<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TSource,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
#[cfg(feature = "System+Linq+Enumerable+WhereArrayIterator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_WhereArrayIterator_1<TSource: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Linq::Enumerable_Iterator_1<TSource>,
    pub source: *mut quest_hook::libil2cpp::Il2CppArray<TSource>,
    pub predicate: *mut crate::System::Func_2<TSource, bool>,
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
        *mut crate::System::Linq::Enumerable_Iterator_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Enumerable_Iterator_1<TSource> = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut quest_hook::libil2cpp::Il2CppArray<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object)
    }
    pub fn Select<TResult>(
        &mut self,
        selector: *mut crate::System::Func_2<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn Where(
        &mut self,
        predicate: *mut crate::System::Func_2<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TSource,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        source: *mut quest_hook::libil2cpp::Il2CppArray<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
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
        Ok(__cordl_ret)
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
    pub source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
    pub predicate: *mut crate::System::Func_2<TSource, bool>,
    pub enumerator: *mut crate::System::Collections::Generic::IEnumerator_1<TSource>,
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
        *mut crate::System::Linq::Enumerable_Iterator_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Enumerable_Iterator_1<TSource> = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object)
    }
    pub fn Select<TResult>(
        &mut self,
        selector: *mut crate::System::Func_2<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn Where(
        &mut self,
        predicate: *mut crate::System::Func_2<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TSource,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
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
        Ok(__cordl_ret)
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
    pub source: *mut crate::System::Collections::Generic::List_1<TSource>,
    pub predicate: *mut crate::System::Func_2<TSource, bool>,
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
        *mut crate::System::Linq::Enumerable_Iterator_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Enumerable_Iterator_1<TSource> = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::Collections::Generic::List_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object)
    }
    pub fn Select<TResult>(
        &mut self,
        selector: *mut crate::System::Func_2<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn Where(
        &mut self,
        predicate: *mut crate::System::Func_2<TSource, bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TSource,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Collections::Generic::List_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
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
        Ok(__cordl_ret)
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
    pub source: *mut quest_hook::libil2cpp::Il2CppArray<TSource>,
    pub predicate: *mut crate::System::Func_2<TSource, bool>,
    pub selector: *mut crate::System::Func_2<TSource, TResult>,
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
        *mut crate::System::Linq::Enumerable_Iterator_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Linq::Enumerable_Iterator_1<TResult> = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut quest_hook::libil2cpp::Il2CppArray<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
        selector: *mut crate::System::Func_2<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
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
        Ok(__cordl_object)
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: *mut crate::System::Func_2<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult2>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult2,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn Where(
        &mut self,
        predicate: *mut crate::System::Func_2<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        source: *mut quest_hook::libil2cpp::Il2CppArray<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
        selector: *mut crate::System::Func_2<TSource, TResult>,
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
        Ok(__cordl_ret)
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
    pub source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
    pub predicate: *mut crate::System::Func_2<TSource, bool>,
    pub selector: *mut crate::System::Func_2<TSource, TResult>,
    pub enumerator: *mut crate::System::Collections::Generic::IEnumerator_1<TSource>,
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
        *mut crate::System::Linq::Enumerable_Iterator_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Linq::Enumerable_Iterator_1<TResult> = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
        selector: *mut crate::System::Func_2<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
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
        Ok(__cordl_object)
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: *mut crate::System::Func_2<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult2>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult2,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn Where(
        &mut self,
        predicate: *mut crate::System::Func_2<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Collections::Generic::IEnumerable_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
        selector: *mut crate::System::Func_2<TSource, TResult>,
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
        Ok(__cordl_ret)
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
    pub source: *mut crate::System::Collections::Generic::List_1<TSource>,
    pub predicate: *mut crate::System::Func_2<TSource, bool>,
    pub selector: *mut crate::System::Func_2<TSource, TResult>,
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
        *mut crate::System::Linq::Enumerable_Iterator_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Linq::Enumerable_Iterator_1<TResult> = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::Collections::Generic::List_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
        selector: *mut crate::System::Func_2<TSource, TResult>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
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
        Ok(__cordl_object)
    }
    pub fn Select<TResult2>(
        &mut self,
        selector: *mut crate::System::Func_2<TResult, TResult2>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult2>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult2,
        > = __cordl_object.invoke("Select", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn Where(
        &mut self,
        predicate: *mut crate::System::Func_2<TResult, bool>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
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
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            TResult,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Collections::Generic::List_1<TSource>,
        predicate: *mut crate::System::Func_2<TSource, bool>,
        selector: *mut crate::System::Func_2<TSource, TResult>,
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
        Ok(__cordl_ret)
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
