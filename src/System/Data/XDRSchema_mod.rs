#[cfg(feature = "System+Data+XDRSchema+NameType")]
#[repr(C)]
#[derive(Debug)]
pub struct XDRSchema_NameType {
    __cordl_parent: crate::System::Object,
    pub name: *mut crate::System::String,
    pub _cordl_type: *mut crate::System::Type,
}
#[cfg(feature = "System+Data+XDRSchema+NameType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XDRSchema_NameType =>
    "System.Data"."XDRSchema/NameType"
);
#[cfg(feature = "System+Data+XDRSchema+NameType")]
impl std::ops::Deref for crate::System::Data::XDRSchema_NameType {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XDRSchema+NameType")]
impl std::ops::DerefMut for crate::System::Data::XDRSchema_NameType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XDRSchema+NameType")]
impl crate::System::Data::XDRSchema_NameType {
    pub fn CompareTo(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        n: *mut crate::System::String,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (n, t))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        n: *mut crate::System::String,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (n, t))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+XDRSchema+NameType")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XDRSchema_NameType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+XDRSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct XDRSchema {
    __cordl_parent: crate::System::Data::XMLSchema,
    pub _schemaName: *mut crate::System::String,
    pub _schemaUri: *mut crate::System::String,
    pub _schemaRoot: *mut crate::System::Xml::XmlElement,
    pub _ds: *mut crate::System::Data::DataSet,
}
#[cfg(feature = "System+Data+XDRSchema")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XDRSchema => "System.Data"
    ."XDRSchema"
);
#[cfg(feature = "System+Data+XDRSchema")]
impl std::ops::Deref for crate::System::Data::XDRSchema {
    type Target = crate::System::Data::XMLSchema;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XDRSchema")]
impl std::ops::DerefMut for crate::System::Data::XDRSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XDRSchema")]
impl crate::System::Data::XDRSchema {
    #[cfg(feature = "System+Data+XDRSchema+NameType")]
    pub type NameType = crate::System::Data::XDRSchema_NameType;
    pub fn FindTypeNode(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlElement = __cordl_object
            .invoke("FindTypeNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateSimpleTable(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
        node: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("InstantiateSimpleTable", (dataSet, node))?;
        Ok(__cordl_ret)
    }
    pub fn HandleColumn(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColumn", (node, table))?;
        Ok(__cordl_ret)
    }
    pub fn HandleTypeNode(
        &mut self,
        typeNode: *mut crate::System::Xml::XmlElement,
        table: *mut crate::System::Data::DataTable,
        tableChildren: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTypeNode", (typeNode, table, tableChildren))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSchema(
        &mut self,
        schemaRoot: *mut crate::System::Xml::XmlElement,
        ds: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (schemaRoot, ds))?;
        Ok(__cordl_ret)
    }
    pub fn IsTextOnlyContent(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTextOnlyContent", (node))?;
        Ok(__cordl_ret)
    }
    pub fn HandleTable(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("HandleTable", (node))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateTable(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
        node: *mut crate::System::Xml::XmlElement,
        typeNode: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("InstantiateTable", (dataSet, node, typeNode))?;
        Ok(__cordl_ret)
    }
    pub fn IsXDRField(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
        typeNode: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXDRField", (node, typeNode))?;
        Ok(__cordl_ret)
    }
    pub fn GetInstanceName(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetInstanceName", (node))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ds: *mut crate::System::Data::DataSet,
        fInline: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ds, fInline))?;
        Ok(__cordl_ret)
    }
    pub fn GetMinMax_ByRefMut0(
        &mut self,
        elNode: *mut crate::System::Xml::XmlElement,
        minOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
        maxOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMinMax", (elNode, minOccurs, maxOccurs))?;
        Ok(__cordl_ret)
    }
    pub fn GetMinMax__cordl_bool_ByRefMut1(
        &mut self,
        elNode: *mut crate::System::Xml::XmlElement,
        isAttribute: bool,
        minOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
        maxOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMinMax", (elNode, isAttribute, minOccurs, maxOccurs))?;
        Ok(__cordl_ret)
    }
    pub fn ParseDataType(
        &mut self,
        dt: *mut crate::System::String,
        dtValues: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ParseDataType", (dt, dtValues))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        ds: *mut crate::System::Data::DataSet,
        fInline: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ds, fInline))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+XDRSchema")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XDRSchema {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
