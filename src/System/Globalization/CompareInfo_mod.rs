#[cfg(feature = "System+Globalization+CompareInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct CompareInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _sortName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_SortVersion: *mut crate::System::Globalization::SortVersion,
    pub culture: i32,
    pub collator: *mut crate::System::Globalization::ISimpleCollator,
}
#[cfg(feature = "System+Globalization+CompareInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CompareInfo =>
    "System.Globalization"."CompareInfo"
);
#[cfg(feature = "System+Globalization+CompareInfo")]
impl std::ops::Deref for crate::System::Globalization::CompareInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CompareInfo")]
impl std::ops::DerefMut for crate::System::Globalization::CompareInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CompareInfo")]
impl crate::System::Globalization::CompareInfo {
    pub fn CompareOptionIgnoreCase(
        &mut self,
        string1: crate::System::ReadOnlySpan_1<char>,
        string2: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareOptionIgnoreCase", (string1, string2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareString_Il2CppString0(
        &mut self,
        string1: crate::System::ReadOnlySpan_1<char>,
        string2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareString", (string1, string2, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareString_ReadOnlySpan_1_1(
        &mut self,
        string1: crate::System::ReadOnlySpan_1<char>,
        string2: crate::System::ReadOnlySpan_1<char>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareString", (string1, string2, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppString_Il2CppString0(
        &mut self,
        string1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        string2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (string1, string2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppString_Il2CppString_CompareOptions1(
        &mut self,
        string1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        string2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (string1, string2, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppString_i32_i32_Il2CppString_i32_i32_CompareOptions3(
        &mut self,
        string1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset1: i32,
        length1: i32,
        string2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset2: i32,
        length2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "Compare",
                (string1, offset1, length1, string2, offset2, length2, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_ReadOnlySpan_1_Il2CppString_CompareOptions2(
        &mut self,
        string1: crate::System::ReadOnlySpan_1<char>,
        string2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (string1, string2, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSortKey(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::SortKey,
        > = __cordl_object.invoke("CreateSortKey", (source, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSortKeyCore(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::SortKey,
        > = __cordl_object.invoke("CreateSortKeyCore", (source, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_Il2CppString_Il2CppString0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        suffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndsWith", (source, suffix, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_ReadOnlySpan_1_ReadOnlySpan_1_1(
        &mut self,
        source: crate::System::ReadOnlySpan_1<char>,
        suffix: crate::System::ReadOnlySpan_1<char>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndsWith", (source, suffix, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCollator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::ISimpleCollator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::ISimpleCollator,
        > = __cordl_object.invoke("GetCollator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCodeOfString(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetHashCodeOfString", (source, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCodeOfStringCore(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetHashCodeOfStringCore", (source, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_Il2CppString_CompareOptions1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (source, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSortKey(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::SortKey,
        > = __cordl_object.invoke("GetSortKey", (source, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfCore(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
        matchLengthPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "IndexOfCore",
                (source, target, startIndex, count, options, matchLengthPtr),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfOrdinal(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startIndex: i32,
        count: i32,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOfOrdinal", (source, value, startIndex, count, ignoreCase))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_CompareOptions0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (source, value, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_i32_i32_CompareOptions1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (source, value, startIndex, count, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSort(
        &mut self,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitSort", (culture))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvariantCreateSortKey(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::SortKey,
        > = __cordl_object.invoke("InvariantCreateSortKey", (source, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrefix(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPrefix", (source, prefix, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSuffix_Il2CppString_Il2CppString0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        suffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSuffix", (source, suffix, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSuffix_ReadOnlySpan_1_ReadOnlySpan_1_1(
        &mut self,
        source: crate::System::ReadOnlySpan_1<char>,
        suffix: crate::System::ReadOnlySpan_1<char>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSuffix", (source, suffix, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOfCore(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOfCore", (source, target, startIndex, count, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOfOrdinal(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startIndex: i32,
        count: i32,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "LastIndexOfOrdinal",
                (source, value, startIndex, count, ignoreCase),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_CompareOptions0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (source, value, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_i32_i32_CompareOptions1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (source, value, startIndex, count, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_CultureInfo0(
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object.into())
    }
    pub fn OnDeserialized_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDeserialized_StreamingContext0(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDeserializing(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserializing", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSerializing(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerializing", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartsWith", (source, prefix, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
                (sender),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CultureInfo0(
        &mut self,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internal_compare_managed(
        &mut self,
        str1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset1: i32,
        length1: i32,
        str2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset2: i32,
        length2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "internal_compare_managed",
                (str1, offset1, length1, str2, offset2, length2, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn internal_compare_switch(
        &mut self,
        str1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset1: i32,
        length1: i32,
        str2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset2: i32,
        length2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "internal_compare_switch",
                (str1, offset1, length1, str2, offset2, length2, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn internal_index_managed(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sindex: i32,
        count: i32,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        opt: crate::System::Globalization::CompareOptions,
        first: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("internal_index_managed", (s1, sindex, count, s2, opt, first))?;
        Ok(__cordl_ret.into())
    }
    pub fn internal_index_switch(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sindex: i32,
        count: i32,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        opt: crate::System::Globalization::CompareOptions,
        first: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("internal_index_switch", (s1, sindex, count, s2, opt, first))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+CompareInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::CompareInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+CompareInfo")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Globalization::CompareInfo {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+CompareInfo")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Globalization::CompareInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
