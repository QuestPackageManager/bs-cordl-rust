#[cfg(feature = "System+Data+Common+SqlUdtStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct SqlUdtStorage {
    __cordl_parent: crate::System::Data::Common::DataStorage,
    pub _values: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
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
    pub fn Aggregate(
        &mut self,
        records: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        kind: crate::System::Data::AggregateType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Aggregate", (records, kind))?;
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
    pub fn GetStaticNullForUdtType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticNullForUdtType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull(&mut self, record: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DataColumn_Type0(
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject1(
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column, _cordl_type, nullValue))?;
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
    pub fn _ctor_DataColumn_Type0(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject1(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column, _cordl_type, nullValue))?;
        Ok(__cordl_ret.into())
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
