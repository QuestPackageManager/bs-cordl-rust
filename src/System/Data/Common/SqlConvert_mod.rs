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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeType2", (value, stype, _cordl_type, formatProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTypeForDefaultValue(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeTypeForDefaultValue", (value, _cordl_type, formatProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTypeForXML(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeTypeForXML", (value, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertStringToDateTimeOffset(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertStringToDateTimeOffset", (value, formatProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlBinary(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBinary> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBinary = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlBinary", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlBoolean(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlBoolean> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlBoolean = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlByte(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlByte> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlByte = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlBytes(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SqlTypes::SqlBytes>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::SqlTypes::SqlBytes,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlBytes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlChars(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SqlTypes::SqlChars>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::SqlTypes::SqlChars,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlChars", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlDateTime(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDateTime> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlDateTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlDecimal(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDecimal> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDecimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlDouble(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlDouble> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlDouble = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlGuid(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlGuid> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlGuid = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlGuid", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlInt16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt16> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlInt32(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt32> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlInt64(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlInt64> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlInt64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlMoney(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlMoney> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlMoney = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlMoney", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlSingle(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlSingle> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlSingle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToSqlString(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SqlTypes::SqlString> {
        let __cordl_ret: crate::System::Data::SqlTypes::SqlString = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToSqlString", (value))?;
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
