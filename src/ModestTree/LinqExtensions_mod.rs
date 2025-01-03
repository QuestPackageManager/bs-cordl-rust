#[cfg(feature = "ModestTree+LinqExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct LinqExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ModestTree+LinqExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::LinqExtensions => "ModestTree"
    ."LinqExtensions"
);
#[cfg(feature = "ModestTree+LinqExtensions")]
impl std::ops::Deref for crate::ModestTree::LinqExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+LinqExtensions")]
impl std::ops::DerefMut for crate::ModestTree::LinqExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+LinqExtensions")]
impl crate::ModestTree::LinqExtensions {
    #[cfg(feature = "ModestTree+LinqExtensions+_Yield_d__0_1")]
    pub type _Yield_d__0_1<T: quest_hook::libil2cpp::Type> = crate::ModestTree::LinqExtensions__Yield_d__0_1<
        T,
    >;
    #[cfg(feature = "ModestTree+LinqExtensions+__c__8_1")]
    pub type __c__8_1<T: quest_hook::libil2cpp::Type> = crate::ModestTree::LinqExtensions___c__8_1<
        T,
    >;
    #[cfg(feature = "ModestTree+LinqExtensions+__c__DisplayClass10_0_1")]
    pub type __c__DisplayClass10_0_1<T: quest_hook::libil2cpp::Type> = crate::ModestTree::LinqExtensions___c__DisplayClass10_0_1<
        T,
    >;
    pub fn ContainsItem<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        value: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsItem", (list, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Except<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        item: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Except", (list, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDuplicates<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDuplicates", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAtLeast<T>(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        amount: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAtLeast", (enumerable, amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAtMost<T>(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        amount: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAtMost", (enumerable, amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLessThan<T>(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        amount: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasLessThan", (enumerable, amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasMoreThan<T>(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        amount: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasMoreThan", (enumerable, amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty_IEnumerable_1_1<T>(
        enumerable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEmpty", (enumerable))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEmpty_List_1_0<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEmpty", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnlyOrDefault<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnlyOrDefault", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Yield<T>(
        item: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Yield", (item))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ModestTree+LinqExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::LinqExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
