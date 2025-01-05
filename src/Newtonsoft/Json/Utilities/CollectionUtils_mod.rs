#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionUtils {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::CollectionUtils =>
    "Newtonsoft.Json.Utilities"."CollectionUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl crate::Newtonsoft::Json::Utilities::CollectionUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
    pub type EmptyArrayContainer_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<
        T,
    >;
    pub fn AddDistinct_Gc1<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        value: T,
        comparer: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDistinct", (list, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDistinct_Gc_T0<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDistinct", (list, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddRange<T>(
        initial: quest_hook::libil2cpp::Gc<T>,
        collection: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddRange", (initial, collection))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddRangeDistinct<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        values: quest_hook::libil2cpp::Gc<T>,
        comparer: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddRangeDistinct", (list, values, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayEmpty<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ArrayEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        value: T,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (list, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsValue<TSource>(
        source: quest_hook::libil2cpp::Gc<TSource>,
        value: TSource,
        comparer: quest_hook::libil2cpp::Gc<TSource>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsValue", (source, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromJaggedToMultidimensionalArray(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        multidimensionalArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyFromJaggedToMultidimensionalArray",
                (values, multidimensionalArray, indices),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FastReverse<T>(
        list: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FastReverse", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDimensions(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        dimensionsCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<i32>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDimensions", (values, dimensionsCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        collection: quest_hook::libil2cpp::Gc<T>,
        predicate: quest_hook::libil2cpp::Gc<T, bool>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (collection, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        item: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfReference", (list, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDictionaryType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDictionaryType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullOrEmpty<T>(
        collection: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullOrEmpty", (collection))?;
        Ok(__cordl_ret.into())
    }
    pub fn JaggedArrayGetValue(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("JaggedArrayGetValue", (values, indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveEnumerableCollectionConstructor_Gc1(
        collectionType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        collectionItemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        constructorArgumentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ResolveEnumerableCollectionConstructor",
                (collectionType, collectionItemType, constructorArgumentType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveEnumerableCollectionConstructor_Gc_Gc0(
        collectionType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        collectionItemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ResolveEnumerableCollectionConstructor",
                (collectionType, collectionItemType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToMultidimensionalArray(
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        rank: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToMultidimensionalArray", (values, _cordl_type, rank))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::CollectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionUtils_EmptyArrayContainer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1 < T > =>
    "Newtonsoft.Json.Utilities"."CollectionUtils/EmptyArrayContainer`1" < T >
);
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {}
#[cfg(feature = "Newtonsoft+Json+Utilities+CollectionUtils+EmptyArrayContainer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::CollectionUtils_EmptyArrayContainer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
