#[cfg(feature = "System+Data+Common+SqlConvert")]
#[repr(C)]
#[derive(Debug)]
pub struct SqlConvert {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::Common::SqlConvert {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data.Common";
    const CLASS_NAME: &'static str = "SqlConvert";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl std::ops::Deref for crate::System::Data::Common::SqlConvert {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl std::ops::DerefMut for crate::System::Data::Common::SqlConvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl crate::System::Data::Common::SqlConvert {
    pub fn ChangeType2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stype: crate::System::Data::Common::StorageType,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::Data::Common::StorageType,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                4usize,
            >("ChangeType2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ChangeType2", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method.invoke_unchecked((), (value, stype, _cordl_type, formatProvider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTypeForDefaultValue(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("ChangeTypeForDefaultValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ChangeTypeForDefaultValue",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method.invoke_unchecked((), (value, _cordl_type, formatProvider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTypeForXML(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("ChangeTypeForXML")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ChangeTypeForXML", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertStringToDateTimeOffset(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                crate::System::DateTimeOffset,
                2usize,
            >("ConvertStringToDateTimeOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ConvertStringToDateTimeOffset", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTimeOffset = unsafe {
            method.invoke_unchecked((), (value, formatProvider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlBinary(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBinary> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlBinary,
                1usize,
            >("ConvertToSqlBinary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlBinary", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBinary = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlBoolean(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlBoolean,
                1usize,
            >("ConvertToSqlBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlBoolean",
                    1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlByte(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlByte> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlByte,
                1usize,
            >("ConvertToSqlByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlByte", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlByte = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlBytes(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SqlTypes::SqlBytes>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::System::Data::SqlTypes::SqlBytes>,
                1usize,
            >("ConvertToSqlBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlBytes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::SqlTypes::SqlBytes,
        > = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlChars(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SqlTypes::SqlChars>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::System::Data::SqlTypes::SqlChars>,
                1usize,
            >("ConvertToSqlChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlChars", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::SqlTypes::SqlChars,
        > = unsafe { method.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlDateTime(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlDateTime,
                1usize,
            >("ConvertToSqlDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlDateTime",
                    1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDateTime = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlDecimal(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlDecimal,
                1usize,
            >("ConvertToSqlDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlDecimal",
                    1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlDouble(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlDouble,
                1usize,
            >("ConvertToSqlDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlDouble", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlGuid(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlGuid> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlGuid,
                1usize,
            >("ConvertToSqlGuid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlGuid", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlGuid = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlInt16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlInt16,
                1usize,
            >("ConvertToSqlInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlInt16", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt16 = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlInt32(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlInt32,
                1usize,
            >("ConvertToSqlInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlInt32", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt32 = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlInt64(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlInt64,
                1usize,
            >("ConvertToSqlInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlInt64", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlMoney(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlMoney,
                1usize,
            >("ConvertToSqlMoney")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlMoney", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlSingle(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlSingle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlSingle,
                1usize,
            >("ConvertToSqlSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlSingle", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlSingle = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlString(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlString> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::Common::SqlConvert as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::Data::SqlTypes::SqlString,
                1usize,
            >("ConvertToSqlString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::Common::SqlConvert as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertToSqlString", 1usize
                )
            });
        let __cordl_ret: crate::System::Data::SqlTypes::SqlString = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Common::SqlConvert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
