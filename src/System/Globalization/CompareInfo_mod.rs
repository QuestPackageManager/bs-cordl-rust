#[cfg(feature = "System+Globalization+CompareInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct CompareInfo {
    __cordl_parent: crate::System::Object,
    pub m_name: *mut crate::System::String,
    pub _sortName: *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub fn CreateSortKey(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("CreateSortKey", (source, options))?;
        Ok(__cordl_ret)
    }
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
        Ok(__cordl_ret)
    }
    pub fn IsSuffix_String_String0(
        &mut self,
        source: *mut crate::System::String,
        suffix: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSuffix", (source, suffix, options))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn OnDeserialized_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf_CompareOptions0(
        &mut self,
        source: *mut crate::System::String,
        value: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (source, value, options))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf_i32_i32_CompareOptions1(
        &mut self,
        source: *mut crate::System::String,
        value: *mut crate::System::String,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (source, value, startIndex, count, options))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOf_CompareOptions0(
        &mut self,
        source: *mut crate::System::String,
        value: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (source, value, options))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOf_i32_i32_CompareOptions1(
        &mut self,
        source: *mut crate::System::String,
        value: *mut crate::System::String,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (source, value, startIndex, count, options))?;
        Ok(__cordl_ret)
    }
    pub fn internal_compare_managed(
        &mut self,
        str1: *mut crate::System::String,
        offset1: i32,
        length1: i32,
        str2: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn LastIndexOfCore(
        &mut self,
        source: *mut crate::System::String,
        target: *mut crate::System::String,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOfCore", (source, target, startIndex, count, options))?;
        Ok(__cordl_ret)
    }
    pub fn internal_compare_switch(
        &mut self,
        str1: *mut crate::System::String,
        offset1: i32,
        length1: i32,
        str2: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn IndexOfCore(
        &mut self,
        source: *mut crate::System::String,
        target: *mut crate::System::String,
        startIndex: i32,
        count: i32,
        options: crate::System::Globalization::CompareOptions,
        matchLengthPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "IndexOfCore",
                (source, target, startIndex, count, options, matchLengthPtr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CultureInfo0(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSortKey(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("GetSortKey", (source, options))?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
                (sender),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn Compare_String_String0(
        &mut self,
        string1: *mut crate::System::String,
        string2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (string1, string2))?;
        Ok(__cordl_ret)
    }
    pub fn Compare_String_String_CompareOptions1(
        &mut self,
        string1: *mut crate::System::String,
        string2: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (string1, string2, options))?;
        Ok(__cordl_ret)
    }
    pub fn Compare_ReadOnlySpan_1_String_CompareOptions2(
        &mut self,
        string1: crate::System::ReadOnlySpan_1<char>,
        string2: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (string1, string2, options))?;
        Ok(__cordl_ret)
    }
    pub fn Compare_String_i32_i32_String_i32_i32_CompareOptions3(
        &mut self,
        string1: *mut crate::System::String,
        offset1: i32,
        length1: i32,
        string2: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn internal_index_managed(
        &mut self,
        s1: *mut crate::System::String,
        sindex: i32,
        count: i32,
        s2: *mut crate::System::String,
        opt: crate::System::Globalization::CompareOptions,
        first: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("internal_index_managed", (s1, sindex, count, s2, opt, first))?;
        Ok(__cordl_ret)
    }
    pub fn GetCollator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Globalization::ISimpleCollator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::ISimpleCollator = __cordl_object
            .invoke("GetCollator", ())?;
        Ok(__cordl_ret)
    }
    pub fn EndsWith_String_String0(
        &mut self,
        source: *mut crate::System::String,
        suffix: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndsWith", (source, suffix, options))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn internal_index_switch(
        &mut self,
        s1: *mut crate::System::String,
        sindex: i32,
        count: i32,
        s2: *mut crate::System::String,
        opt: crate::System::Globalization::CompareOptions,
        first: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("internal_index_switch", (s1, sindex, count, s2, opt, first))?;
        Ok(__cordl_ret)
    }
    pub fn InitSort(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitSort", (culture))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn StartsWith(
        &mut self,
        source: *mut crate::System::String,
        prefix: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartsWith", (source, prefix, options))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCodeOfStringCore(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetHashCodeOfStringCore", (source, options))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOfOrdinal(
        &mut self,
        source: *mut crate::System::String,
        value: *mut crate::System::String,
        startIndex: i32,
        count: i32,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOfOrdinal", (source, value, startIndex, count, ignoreCase))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCodeOfString(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetHashCodeOfString", (source, options))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOfOrdinal(
        &mut self,
        source: *mut crate::System::String,
        value: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn GetHashCode_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode_String_CompareOptions1(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (source, options))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrefix(
        &mut self,
        source: *mut crate::System::String,
        prefix: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPrefix", (source, prefix, options))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvariantCreateSortKey(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("InvariantCreateSortKey", (source, options))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSortKeyCore(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("CreateSortKeyCore", (source, options))?;
        Ok(__cordl_ret)
    }
    pub fn CompareString_String0(
        &mut self,
        string1: crate::System::ReadOnlySpan_1<char>,
        string2: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareString", (string1, string2, options))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New_CultureInfo0(
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
