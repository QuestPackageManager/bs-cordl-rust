#[cfg(feature = "System+Data+Common+DataStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct DataStorage {
    __cordl_parent: crate::System::Object,
    pub _column: *mut crate::System::Data::DataColumn,
    pub _table: *mut crate::System::Data::DataTable,
    pub _dataType: *mut crate::System::Type,
    pub _storageTypeCode: crate::System::Data::Common::StorageType,
    pub _dbNullBits: *mut crate::System::Collections::BitArray,
    pub _defaultValue: *mut crate::System::Object,
    pub _nullValue: *mut crate::System::Object,
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
    type Target = crate::System::Object;
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
        recordNos: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        kind: crate::System::Data::AggregateType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Aggregate", (recordNos, kind))?;
        Ok(__cordl_ret)
    }
    pub fn AggregateCount(
        &mut self,
        recordNos: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("AggregateCount", (recordNos))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn CompareValueTo(
        &mut self,
        recordNo1: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareValueTo", (recordNo1, value))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertObjectToXml_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ConvertObjectToXml", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertObjectToXml_XmlWriter_XmlRootAttribute1(
        &mut self,
        value: *mut crate::System::Object,
        xmlWriter: *mut crate::System::Xml::XmlWriter,
        xmlAttrib: *mut crate::System::Xml::Serialization::XmlRootAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertObjectToXml", (value, xmlWriter, xmlAttrib))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertValue(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertXmlToObject_String0(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertXmlToObject", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertXmlToObject_XmlReader_XmlRootAttribute1(
        &mut self,
        xmlReader: *mut crate::System::Xml::XmlReader,
        xmlAttrib: *mut crate::System::Xml::Serialization::XmlRootAttribute,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ConvertXmlToObject", (xmlReader, xmlAttrib))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn CopyValue(
        &mut self,
        record: i32,
        store: *mut crate::System::Object,
        nullbits: *mut crate::System::Collections::BitArray,
        storeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValue", (record, store, nullbits, storeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn CopyValueInternal(
        &mut self,
        record: i32,
        store: *mut crate::System::Object,
        nullbits: *mut crate::System::Collections::BitArray,
        storeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValueInternal", (record, store, nullbits, storeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn Get(
        &mut self,
        recordNo: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Get", (recordNo))?;
        Ok(__cordl_ret)
    }
    pub fn GetBits(
        &mut self,
        recordNo: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetBits", (recordNo))?;
        Ok(__cordl_ret)
    }
    pub fn GetEmptyStorage(
        &mut self,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetEmptyStorage", (recordCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetEmptyStorageInternal(
        &mut self,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetEmptyStorageInternal", (recordCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringLength(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetStringLength", (record))?;
        Ok(__cordl_ret)
    }
    pub fn HasValue(&mut self, recordNo: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasValue", (recordNo))?;
        Ok(__cordl_ret)
    }
    pub fn IsNull(&mut self, recordNo: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", (recordNo))?;
        Ok(__cordl_ret)
    }
    pub fn New_Object_StorageType1(
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        defaultValue: *mut crate::System::Object,
        nullValue: *mut crate::System::Object,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (column, _cordl_type, defaultValue, nullValue, storageType),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Object__cordl_bool_StorageType2(
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        defaultValue: *mut crate::System::Object,
        nullValue: *mut crate::System::Object,
        isICloneable: bool,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (column, _cordl_type, defaultValue, nullValue, isICloneable, storageType),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_StorageType0(
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        defaultValue: *mut crate::System::Object,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column, _cordl_type, defaultValue, storageType))?;
        Ok(__cordl_object)
    }
    pub fn Set(
        &mut self,
        recordNo: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (recordNo, value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetNullStorage(
        &mut self,
        nullbits: *mut crate::System::Collections::BitArray,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNullStorage", (nullbits))?;
        Ok(__cordl_ret)
    }
    pub fn SetStorage(
        &mut self,
        store: *mut crate::System::Object,
        nullbits: *mut crate::System::Collections::BitArray,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStorage", (store, nullbits))?;
        Ok(__cordl_ret)
    }
    pub fn SetStorageInternal(
        &mut self,
        store: *mut crate::System::Object,
        nullbits: *mut crate::System::Collections::BitArray,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStorageInternal", (store, nullbits))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_StorageType1(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        defaultValue: *mut crate::System::Object,
        nullValue: *mut crate::System::Object,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object__cordl_bool_StorageType2(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        defaultValue: *mut crate::System::Object,
        nullValue: *mut crate::System::Object,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_StorageType0(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        defaultValue: *mut crate::System::Object,
        storageType: crate::System::Data::Common::StorageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column, _cordl_type, defaultValue, storageType))?;
        Ok(__cordl_ret)
    }
    pub fn get_DateTimeMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataSetDateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataSetDateTime = __cordl_object
            .invoke("get_DateTimeMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FormatProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IFormatProvider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IFormatProvider = __cordl_object
            .invoke("get_FormatProvider", ())?;
        Ok(__cordl_ret)
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