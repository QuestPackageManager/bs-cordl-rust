#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::EnumUtils =>
    "Newtonsoft.Json.Utilities"."EnumUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::EnumUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::EnumUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl crate::Newtonsoft::Json::Utilities::EnumUtils {
    pub const EnumSeparatorChar: char = ',';
    pub const EnumSeparatorString: &'static str = ", ";
    pub fn FindIndexByName(
        enumNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueIndex: i32,
        valueSubstringLength: i32,
        comparison: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_ret: crate::System::Nullable_1<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FindIndexByName",
                (enumNames, value, valueIndex, valueSubstringLength, comparison),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumValuesAndNames(
        enumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::EnumInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::EnumInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnumValuesAndNames", (enumType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFlagsValues<T>(
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFlagsValues", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeValuesAndNames(
        key: crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::NamingStrategy,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::EnumInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::EnumInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeValuesAndNames", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalFlagsFormat(
        entry: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::EnumInfo>,
        result: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalFlagsFormat", (entry, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enumNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        resolvedNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        valueIndex: i32,
        valueSubstringLength: i32,
        comparison: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_ret: crate::System::Nullable_1<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchName",
                (
                    value,
                    enumNames,
                    resolvedNames,
                    valueIndex,
                    valueSubstringLength,
                    comparison,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseEnum(
        enumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        namingStrategy: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        disallowNumber: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseEnum", (enumType, namingStrategy, value, disallowNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryToString_NamingStrategy1(
        enumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        namingStrategy: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        >,
        name: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryToString", (enumType, value, namingStrategy, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryToString__cordl_bool0(
        enumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        camelCase: bool,
        name: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryToString", (enumType, value, camelCase, name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::EnumUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
