#[cfg(feature = "System+Data+Common+DataStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct DataStorage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _column: *mut crate::System::Data::DataColumn,
    pub _table: *mut crate::System::Data::DataTable,
    pub _dataType: *mut crate::System::Type,
    pub _storageTypeCode: crate::System::Data::Common::StorageType,
    pub _dbNullBits: *mut crate::System::Collections::BitArray,
    pub _defaultValue: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _nullValue: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _isCloneable: bool,
    pub _isCustomDefinedType: bool,
    pub _isStringType: bool,
    pub _isValueType: bool,
}
#[cfg(feature = "System+Data+Common+DataStorage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Common::DataStorage =>
    "System.Data.Common"."DataStorage"
);
#[cfg(feature = "System+Data+Common+DataStorage")]
impl std::ops::Deref for crate::System::Data::Common::DataStorage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+DataStorage")]
impl std::ops::DerefMut for crate::System::Data::Common::DataStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+DataStorage")]
impl crate::System::Data::Common::DataStorage {
    pub fn Aggregate(
        &mut self,
        recordNos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        kind: crate::System::Data::AggregateType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Aggregate", (recordNos, kind))?;
        Ok(__cordl_ret.into())
    }
    pub fn AggregateCount(
        &mut self,
        recordNos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("AggregateCount", (recordNos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        &mut self,
        recordNo1: i32,
        recordNo2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (recordNo1, recordNo2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareBits(
        &mut self,
        recordNo1: i32,
        recordNo2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareBits", (recordNo1, recordNo2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareValueTo(
        &mut self,
        recordNo1: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareValueTo", (recordNo1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertObjectToXml_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ConvertObjectToXml", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertObjectToXml_XmlWriter_XmlRootAttribute1(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        xmlWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        xmlAttrib: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertObjectToXml", (value, xmlWriter, xmlAttrib))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertValue(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ConvertValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertXmlToObject_Il2CppString0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ConvertXmlToObject", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertXmlToObject_XmlReader_XmlRootAttribute1(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        xmlAttrib: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ConvertXmlToObject", (xmlReader, xmlAttrib))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy(
        &mut self,
        recordNo1: i32,
        recordNo2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Copy", (recordNo1, recordNo2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyBits(
        &mut self,
        srcRecordNo: i32,
        dstRecordNo: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyBits", (srcRecordNo, dstRecordNo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyValue(
        &mut self,
        record: i32,
        store: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullbits: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
        storeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValue", (record, store, nullbits, storeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyValueInternal(
        &mut self,
        record: i32,
        store: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullbits: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
        storeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValueInternal", (record, store, nullbits, storeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateStorage(
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeCode: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::Common::DataStorage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::Common::DataStorage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateStorage", (column, dataType, typeCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineIfValueType(
        typeCode: crate::System::Data::Common::StorageType,
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetermineIfValueType", (typeCode, dataType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get(
        &mut self,
        recordNo: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Get", (recordNo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBits(
        &mut self,
        recordNo: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetBits", (recordNo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEmptyStorage(
        &mut self,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetEmptyStorage", (recordCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEmptyStorageInternal(
        &mut self,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetEmptyStorageInternal", (recordCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQualifiedName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQualifiedName", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStorageType(
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Common::StorageType> {
        let __cordl_ret: crate::System::Data::Common::StorageType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStorageType", (dataType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringLength(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetStringLength", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetType(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeStorage(
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeStorage", (storageType))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasValue(&mut self, recordNo: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasValue", (recordNo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplementsINullableValue(
        typeCode: crate::System::Data::Common::StorageType,
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplementsINullableValue", (typeCode, dataType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplementsInterfaces(
        typeCode: crate::System::Data::Common::StorageType,
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        sqlType: quest_hook::libil2cpp::ByRefMut<bool>,
        nullable: quest_hook::libil2cpp::ByRefMut<bool>,
        xmlSerializable: quest_hook::libil2cpp::ByRefMut<bool>,
        changeTracking: quest_hook::libil2cpp::ByRefMut<bool>,
        revertibleChangeTracking: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ImplementsInterfaces",
                (
                    typeCode,
                    dataType,
                    sqlType,
                    nullable,
                    xmlSerializable,
                    changeTracking,
                    revertibleChangeTracking,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InspectTypeForInterfaces(
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Tuple_4<bool, bool, bool, bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Tuple_4<bool, bool, bool, bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InspectTypeForInterfaces", (dataType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull(&mut self, recordNo: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", (recordNo))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsObjectNull(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsObjectNull", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsObjectSqlNull(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsObjectSqlNull", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSqlType_StorageType0(
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSqlType", (storageType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSqlType_Type1(
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSqlType", (dataType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTypeCustomType_StorageType1(
        typeCode: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTypeCustomType", (typeCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTypeCustomType_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTypeCustomType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppObject_StorageType1(
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (column, _cordl_type, defaultValue, nullValue, storageType),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject__cordl_bool_StorageType2(
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isICloneable: bool,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (column, _cordl_type, defaultValue, nullValue, isICloneable, storageType),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_StorageType0(
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column, _cordl_type, defaultValue, storageType))?;
        Ok(__cordl_object.into())
    }
    pub fn Set(
        &mut self,
        recordNo: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (recordNo, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCapacity(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCapacity", (capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNullBit(
        &mut self,
        recordNo: i32,
        flag: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNullBit", (recordNo, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNullStorage(
        &mut self,
        nullbits: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNullStorage", (nullbits))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStorage(
        &mut self,
        store: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullbits: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStorage", (store, nullbits))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStorageInternal(
        &mut self,
        store: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullbits: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStorageInternal", (store, nullbits))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_StorageType1(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (column, _cordl_type, defaultValue, nullValue, storageType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject__cordl_bool_StorageType2(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isICloneable: bool,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (column, _cordl_type, defaultValue, nullValue, isICloneable, storageType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StorageType0(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column, _cordl_type, defaultValue, storageType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateTimeMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataSetDateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataSetDateTime = __cordl_object
            .invoke("get_DateTimeMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FormatProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider> = __cordl_object
            .invoke("get_FormatProvider", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Common+DataStorage")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Common::DataStorage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
