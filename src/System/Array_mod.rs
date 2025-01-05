#[cfg(feature = "System+Array")]
#[repr(C)]
#[derive(Debug)]
pub struct Array {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Array")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Array => "System"."Array"
);
#[cfg(feature = "System+Array")]
impl std::ops::Deref for crate::System::Array {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array")]
impl std::ops::DerefMut for crate::System::Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array")]
impl crate::System::Array {
    #[cfg(feature = "System+Array+ArrayEnumerator")]
    pub type ArrayEnumerator = crate::System::Array_ArrayEnumerator;
    #[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
    pub type EmptyInternalEnumerator_1<T: quest_hook::libil2cpp::Type> = crate::System::Array_EmptyInternalEnumerator_1<
        T,
    >;
    #[cfg(feature = "System+Array+InternalEnumerator_1")]
    pub type InternalEnumerator_1<T: quest_hook::libil2cpp::Type> = crate::System::Array_InternalEnumerator_1<
        T,
    >;
    #[cfg(feature = "System+Array+RawData")]
    pub type RawData = crate::System::Array_RawData;
    #[cfg(feature = "System+Array+SorterGenericArray")]
    pub type SorterGenericArray = crate::System::Array_SorterGenericArray;
    #[cfg(feature = "System+Array+SorterObjectArray")]
    pub type SorterObjectArray = crate::System::Array_SorterObjectArray;
    pub fn AsReadOnly<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsReadOnly", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Array_Il2CppObject0(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Array_Il2CppObject_IComparer2(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Array_i32_i32_Il2CppObject1(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, index, length, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Array_i32_i32_Il2CppObject_IComparer3(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, index, length, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Il2CppArray_T4<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Il2CppArray_T_IComparer_1_5<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Il2CppArray_i32_i32_T6<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
        length: i32,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, index, length, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BinarySearch_Il2CppArray_i32_i32_T_IComparer_1_7<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
        length: i32,
        value: T,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BinarySearch", (array, index, length, value, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanAssignArrayElement(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanAssignArrayElement", (source, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", (array, index, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearInternal(
        a: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearInternal", (a, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes(h1: i32, h2: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHashCodes", (h1, h2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConstrainedCopy(
        sourceArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        sourceIndex: i32,
        destinationArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        destinationIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConstrainedCopy",
                (sourceArray, sourceIndex, destinationArray, destinationIndex, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertAll<TInput, TOutput>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TInput>>,
        converter: quest_hook::libil2cpp::Gc<crate::System::Converter_2<TInput, TOutput>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TOutput>>,
    >
    where
        TInput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TOutput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TOutput>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertAll", (array, converter))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_i32_0(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_i64_1(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_Array_i32_2(
        sourceArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        destinationArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (sourceArray, destinationArray, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_Array_i64_0(
        sourceArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        destinationArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        length: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (sourceArray, destinationArray, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_Array_i32_i32_3(
        sourceArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        sourceIndex: i32,
        destinationArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        destinationIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Copy",
                (sourceArray, sourceIndex, destinationArray, destinationIndex, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i64_Array_i64_i64_1(
        sourceArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        sourceIndex: i64,
        destinationArray: quest_hook::libil2cpp::Gc<crate::System::Array>,
        destinationIndex: i64,
        length: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Copy",
                (sourceArray, sourceIndex, destinationArray, destinationIndex, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArrayTypeMismatchException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArrayTypeMismatchException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArrayTypeMismatchException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArrayTypeMismatchException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstanceImpl(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lengths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bounds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstanceImpl", (elementType, lengths, bounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Il2CppArray0(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lengths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (elementType, lengths))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Il2CppArray4(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lengths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (elementType, lengths))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Il2CppArray_Il2CppArray5(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lengths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        lowerBounds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (elementType, lengths, lowerBounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_i32_1(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (elementType, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_i32_i32_2(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        length1: i32,
        length2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (elementType, length1, length2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_i32_i32_i32_3(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        length1: i32,
        length2: i32,
        length3: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (elementType, length1, length2, length3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Empty<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Empty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Exists<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exists", (array, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FastCopy(
        source: quest_hook::libil2cpp::Gc<crate::System::Array>,
        source_idx: i32,
        dest: quest_hook::libil2cpp::Gc<crate::System::Array>,
        dest_idx: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FastCopy", (source, source_idx, dest, dest_idx, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fill_Il2CppArray_T0<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fill", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fill_i32_i32_1<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fill", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Find<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Find", (array, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindAll<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
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
            .invoke("FindAll", (array, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIndex_Predicate_1_0<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIndex", (array, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIndex_i32_Predicate_1_1<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        startIndex: i32,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIndex", (array, startIndex, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIndex_i32_i32_Predicate_1_2<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        startIndex: i32,
        count: i32,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIndex", (array, startIndex, count, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindLast<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindLast", (array, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindLastIndex_Predicate_1_0<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindLastIndex", (array, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindLastIndex_i32_Predicate_1_1<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        startIndex: i32,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindLastIndex", (array, startIndex, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindLastIndex_i32_i32_Predicate_1_2<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        startIndex: i32,
        count: i32,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindLastIndex", (array, startIndex, count, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForEach<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForEach", (array, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGenericValueImpl<T>(
        &mut self,
        pos: i32,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGenericValueImpl", (pos, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLength(&mut self, dimension: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLength", (dimension))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLongLength(
        &mut self,
        dimension: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetLongLength", (dimension))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLowerBound(
        &mut self,
        dimension: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLowerBound", (dimension))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMedian(low: i32, hi: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMedian", (low, hi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRank(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetRank", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawSzArrayData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<u8> = __cordl_object
            .invoke("GetRawSzArrayData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUpperBound(
        &mut self,
        dimension: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUpperBound", (dimension))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueImpl(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValueImpl", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_Il2CppArray3(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_Il2CppArray4(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_i32_5(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_i32_i32_6(
        &mut self,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (index1, index2))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_i32_i32_i32_7(
        &mut self,
        index1: i32,
        index2: i32,
        index3: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (index1, index2, index3))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_i64_0(
        &mut self,
        index: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_i64_i64_1(
        &mut self,
        index1: i64,
        index2: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (index1, index2))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue_i64_i64_i64_2(
        &mut self,
        index1: i64,
        index2: i64,
        index3: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (index1, index2, index3))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfImpl<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfImpl", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Array_Il2CppObject0(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Array_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Array_Il2CppObject_i32_i32_2(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Il2CppArray_T3<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Il2CppArray_T_i32_4<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Il2CppArray_T_i32_i32_5<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__ICollection_Add<T>(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalArray__ICollection_Add", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__ICollection_Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalArray__ICollection_Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__ICollection_Contains<T>(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalArray__ICollection_Contains", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__ICollection_CopyTo<T>(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalArray__ICollection_CopyTo", (array, arrayIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__ICollection_Remove<T>(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalArray__ICollection_Remove", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__ICollection_get_Count(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalArray__ICollection_get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__ICollection_get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalArray__ICollection_get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__IEnumerable_GetEnumerator<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<T>,
        > = __cordl_object.invoke("InternalArray__IEnumerable_GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__IReadOnlyCollection_get_Count(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalArray__IReadOnlyCollection_get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__IReadOnlyList_get_Item<T>(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("InternalArray__IReadOnlyList_get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__IndexOf<T>(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("InternalArray__IndexOf", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__Insert<T>(
        &mut self,
        index: i32,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalArray__Insert", (index, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalArray__RemoveAt", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__get_Item<T>(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("InternalArray__get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalArray__set_Item<T>(
        &mut self,
        index: i32,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalArray__set_Item", (index, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOfImpl<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOfImpl", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Array_Il2CppObject0(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOf", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Array_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOf", (array, value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Array_Il2CppObject_i32_i32_2(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOf", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Il2CppArray_T3<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOf", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Il2CppArray_T_i32_4<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOf", (array, value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Il2CppArray_T_i32_i32_5<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        value: T,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOf", (array, value, startIndex, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Resize<T>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
        newSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Resize", (array, newSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reverse_Array0(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reverse", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reverse_Array_i32_i32_1(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reverse", (array, index, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reverse_Il2CppArray2<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reverse", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reverse_Il2CppArray_i32_i32_3<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reverse", (array, index, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGenericValueImpl<T>(
        &mut self,
        pos: i32,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGenericValueImpl", (pos, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValueImpl(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueImpl", (value, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_Il2CppArray3(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_Il2CppArray4(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_i32_5(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_i32_i32_6(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, index1, index2))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_i32_i32_i32_7(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index1: i32,
        index2: i32,
        index3: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, index1, index2, index3))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_i64_0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_i64_i64_1(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index1: i64,
        index2: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, index1, index2))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue_i64_i64_i64_2(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index1: i64,
        index2: i64,
        index3: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, index1, index2, index3))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortImpl(
        keys: quest_hook::libil2cpp::Gc<crate::System::Array>,
        items: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortImpl", (keys, items, index, length, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array0(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array_Array4(
        keys: quest_hook::libil2cpp::Gc<crate::System::Array>,
        items: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array_Array_IComparer5(
        keys: quest_hook::libil2cpp::Gc<crate::System::Array>,
        items: quest_hook::libil2cpp::Gc<crate::System::Array>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array_Array_i32_i32_6(
        keys: quest_hook::libil2cpp::Gc<crate::System::Array>,
        items: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items, index, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array_Array_i32_i32_IComparer7(
        keys: quest_hook::libil2cpp::Gc<crate::System::Array>,
        items: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items, index, length, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array_IComparer2(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array_i32_i32_1(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array, index, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Array_i32_i32_IComparer3(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
        length: i32,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array, index, length, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray8<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_Comparison_1_12<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        comparison: quest_hook::libil2cpp::Gc<crate::System::Comparison_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array, comparison))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_IComparer_1_10<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_Il2CppArray13<TKey, TValue>(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        items: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_Il2CppArray_IComparer_1_15<TKey, TValue>(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        items: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_Il2CppArray_i32_i32_14<TKey, TValue>(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        items: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items, index, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_Il2CppArray_i32_i32_IComparer_1_16<TKey, TValue>(
        keys: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        items: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        index: i32,
        length: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (keys, items, index, length, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_i32_i32_9<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array, index, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort_Il2CppArray_i32_i32_IComparer_1_11<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
        length: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (array, index, length, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_Count(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.ICollection.get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_Add(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IList.Add", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_Contains(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IList.Contains", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_IndexOf(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IList.IndexOf", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_Insert(
        &mut self,
        index: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Insert", (index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_Remove(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Remove", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.RemoveAt", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IList.get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IList.get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IList_set_Item(
        &mut self,
        index: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.set_Item", (index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralComparable_CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Collections.IStructuralComparable.CompareTo",
                (other, comparer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralEquatable_Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.IStructuralEquatable.Equals",
                (other, comparer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralEquatable_GetHashCode(
        &mut self,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IStructuralEquatable.GetHashCode", (comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrueForAll<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrueForAll", (array, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeCreateInstance_Il2CppArray2(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lengths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeCreateInstance", (elementType, lengths))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeCreateInstance_Il2CppArray_Il2CppArray0(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        lengths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        lowerBounds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeCreateInstance", (elementType, lengths, lowerBounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeCreateInstance_i32_i32_1(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        length1: i32,
        length2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeCreateInstance", (elementType, length1, length2))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeLoad<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeLoad", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeMov<S, R>(instance: S) -> quest_hook::libil2cpp::Result<R>
    where
        S: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        R: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: R = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeMov", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeStore<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeStore", (array, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFixedSize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFixedSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSynchronized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSynchronized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LongLength(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_LongLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Rank(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Rank", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_SyncRoot", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Array")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Array {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Array")]
impl AsRef<crate::System::Collections::ICollection> for crate::System::Array {
    fn as_ref(&self) -> &crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsMut<crate::System::Collections::ICollection> for crate::System::Array {
    fn as_mut(&mut self) -> &mut crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsRef<crate::System::Collections::IEnumerable> for crate::System::Array {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsMut<crate::System::Collections::IEnumerable> for crate::System::Array {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsRef<crate::System::Collections::IList> for crate::System::Array {
    fn as_ref(&self) -> &crate::System::Collections::IList {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsMut<crate::System::Collections::IList> for crate::System::Array {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IList {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsRef<crate::System::Collections::IStructuralComparable> for crate::System::Array {
    fn as_ref(&self) -> &crate::System::Collections::IStructuralComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsMut<crate::System::Collections::IStructuralComparable> for crate::System::Array {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IStructuralComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsRef<crate::System::Collections::IStructuralEquatable> for crate::System::Array {
    fn as_ref(&self) -> &crate::System::Collections::IStructuralEquatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsMut<crate::System::Collections::IStructuralEquatable> for crate::System::Array {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IStructuralEquatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsRef<crate::System::ICloneable> for crate::System::Array {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array")]
impl AsMut<crate::System::ICloneable> for crate::System::Array {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Array_ArrayEnumerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    pub _index: i32,
    pub _endIndex: i32,
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Array_ArrayEnumerator => "System"
    ."Array/ArrayEnumerator"
);
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl std::ops::Deref for crate::System::Array_ArrayEnumerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl std::ops::DerefMut for crate::System::Array_ArrayEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl crate::System::Array_ArrayEnumerator {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (array))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Array_ArrayEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::System::Array_ArrayEnumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::System::Array_ArrayEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl AsRef<crate::System::ICloneable> for crate::System::Array_ArrayEnumerator {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+ArrayEnumerator")]
impl AsMut<crate::System::ICloneable> for crate::System::Array_ArrayEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Array_EmptyInternalEnumerator_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Array_EmptyInternalEnumerator_1 < T > =>
    "System"."Array/EmptyInternalEnumerator`1" < T >
);
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::Array_EmptyInternalEnumerator_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerator_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+EmptyInternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::System::Array_EmptyInternalEnumerator_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Array_InternalEnumerator_1<T: quest_hook::libil2cpp::Type> {
    pub array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    pub idx: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Array_InternalEnumerator_1 < T > =>
    "System"."Array/InternalEnumerator`1<T>" < T >
);
#[cfg(feature = "System+Array+InternalEnumerator_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Array_InternalEnumerator_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::Array_InternalEnumerator_1<T> {
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
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
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
#[cfg(feature = "System+Array+InternalEnumerator_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::Array_InternalEnumerator_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::Array_InternalEnumerator_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::System::Array_InternalEnumerator_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::System::Array_InternalEnumerator_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::System::Array_InternalEnumerator_1<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Array+InternalEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::System::Array_InternalEnumerator_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Array+RawData")]
#[repr(C)]
#[derive(Debug)]
pub struct Array_RawData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Bounds: crate::System::IntPtr,
    pub Count: crate::System::IntPtr,
    pub Data: u8,
}
#[cfg(feature = "System+Array+RawData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Array_RawData => "System"
    ."Array/RawData"
);
#[cfg(feature = "System+Array+RawData")]
impl std::ops::Deref for crate::System::Array_RawData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array+RawData")]
impl std::ops::DerefMut for crate::System::Array_RawData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Array+RawData")]
impl crate::System::Array_RawData {}
#[cfg(feature = "System+Array+RawData")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Array_RawData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Array+SorterGenericArray")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Array_SorterGenericArray {
    pub keys: quest_hook::libil2cpp::Gc<crate::System::Array>,
    pub items: quest_hook::libil2cpp::Gc<crate::System::Array>,
    pub comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
}
#[cfg(feature = "System+Array+SorterGenericArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Array_SorterGenericArray => "System"
    ."Array/SorterGenericArray"
);
#[cfg(feature = "System+Array+SorterGenericArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Array_SorterGenericArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Array+SorterGenericArray")]
impl crate::System::Array_SorterGenericArray {
    pub fn DownHeap(
        &mut self,
        i: i32,
        n: i32,
        lo: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DownHeap",
            (i, n, lo),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Heapsort(
        &mut self,
        lo: i32,
        hi: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Heapsort",
            (lo, hi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertionSort(
        &mut self,
        lo: i32,
        hi: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertionSort",
            (lo, hi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntroSort(
        &mut self,
        lo: i32,
        hi: i32,
        depthLimit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IntroSort",
            (lo, hi, depthLimit),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntrospectiveSort(
        &mut self,
        left: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IntrospectiveSort",
            (left, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PickPivotAndPartition(
        &mut self,
        lo: i32,
        hi: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PickPivotAndPartition",
            (lo, hi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort(
        &mut self,
        left: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Sort",
            (left, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Swap(
        &mut self,
        i: i32,
        j: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Swap",
            (i, j),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapIfGreaterWithItems(
        &mut self,
        a: i32,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SwapIfGreaterWithItems",
            (a, b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keys: quest_hook::libil2cpp::Gc<crate::System::Array>,
        items: quest_hook::libil2cpp::Gc<crate::System::Array>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keys, items, comparer),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Array+SorterObjectArray")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Array_SorterObjectArray {
    pub keys: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    >,
    pub items: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    >,
    pub comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
}
#[cfg(feature = "System+Array+SorterObjectArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Array_SorterObjectArray => "System"
    ."Array/SorterObjectArray"
);
#[cfg(feature = "System+Array+SorterObjectArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Array_SorterObjectArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Array+SorterObjectArray")]
impl crate::System::Array_SorterObjectArray {
    pub fn DownHeap(
        &mut self,
        i: i32,
        n: i32,
        lo: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DownHeap",
            (i, n, lo),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Heapsort(
        &mut self,
        lo: i32,
        hi: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Heapsort",
            (lo, hi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertionSort(
        &mut self,
        lo: i32,
        hi: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertionSort",
            (lo, hi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntroSort(
        &mut self,
        lo: i32,
        hi: i32,
        depthLimit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IntroSort",
            (lo, hi, depthLimit),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntrospectiveSort(
        &mut self,
        left: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IntrospectiveSort",
            (left, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PickPivotAndPartition(
        &mut self,
        lo: i32,
        hi: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PickPivotAndPartition",
            (lo, hi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort(
        &mut self,
        left: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Sort",
            (left, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Swap(
        &mut self,
        i: i32,
        j: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Swap",
            (i, j),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapIfGreaterWithItems(
        &mut self,
        a: i32,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SwapIfGreaterWithItems",
            (a, b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keys: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        items: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keys, items, comparer),
        )?;
        Ok(__cordl_ret.into())
    }
}
