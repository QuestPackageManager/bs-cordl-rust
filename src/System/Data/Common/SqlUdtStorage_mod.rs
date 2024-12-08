#[cfg(feature = "System+Data+Common+SqlUdtStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct SqlUdtStorage {
    __cordl_parent: crate::System::Data::Common::DataStorage,
    pub _values: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub _implementsIXmlSerializable: bool,
    pub _implementsIComparable: bool,
}
#[cfg(feature = "System+Data+Common+SqlUdtStorage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Common::SqlUdtStorage =>
    "System.Data.Common"."SqlUdtStorage"
);
#[cfg(feature = "System+Data+Common+SqlUdtStorage")]
impl std::ops::Deref for crate::System::Data::Common::SqlUdtStorage {
    type Target = crate::System::Data::Common::DataStorage;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+SqlUdtStorage")]
impl std::ops::DerefMut for crate::System::Data::Common::SqlUdtStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+SqlUdtStorage")]
impl crate::System::Data::Common::SqlUdtStorage {
    #[cfg(feature = "System+Data+Common+SqlUdtStorage+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::System::Data::Common::SqlUdtStorage___c__DisplayClass6_0;
    pub fn Aggregate(
        &mut self,
        records: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        kind: crate::System::Data::AggregateType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Aggregate", (records, kind))?;
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
    pub fn IsNull(&mut self, record: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", (record))?;
        Ok(__cordl_ret)
    }
    pub fn New_DataColumn_Type0(
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column, _cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn New_Object1(
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        nullValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column, _cordl_type, nullValue))?;
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
    pub fn _ctor_DataColumn_Type0(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object1(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
        _cordl_type: *mut crate::System::Type,
        nullValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column, _cordl_type, nullValue))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+Common+SqlUdtStorage")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Common::SqlUdtStorage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
