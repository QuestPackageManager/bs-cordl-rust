#[cfg(feature = "System+Data+XDRSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct XDRSchema {
    __cordl_parent: crate::System::Data::XMLSchema,
    pub _schemaName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _schemaUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _schemaRoot: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    pub _ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
}
#[cfg(feature = "System+Data+XDRSchema")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::XDRSchema {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "XDRSchema";
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
    pub fn FindNameType(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::XDRSchema_NameType>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::XDRSchema_NameType,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindNameType", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindTypeNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement> = __cordl_object
            .invoke("FindTypeNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceName(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetInstanceName", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinMax_ByRefMut0(
        &mut self,
        elNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        minOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
        maxOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMinMax", (elNode, minOccurs, maxOccurs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinMax__cordl_bool_ByRefMut1(
        &mut self,
        elNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        isAttribute: bool,
        minOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
        maxOccurs: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMinMax", (elNode, isAttribute, minOccurs, maxOccurs))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleColumn(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColumn", (node, table))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleTable(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("HandleTable", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleTypeNode(
        &mut self,
        typeNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        tableChildren: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTypeNode", (typeNode, table, tableChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateSimpleTable(
        &mut self,
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("InstantiateSimpleTable", (dataSet, node))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateTable(
        &mut self,
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        typeNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("InstantiateTable", (dataSet, node, typeNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTextOnlyContent(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTextOnlyContent", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsXDRField(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        typeNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsXDRField", (node, typeNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSchema(
        &mut self,
        schemaRoot: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (schemaRoot, ds))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        fInline: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ds, fInline))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseDataType(
        &mut self,
        dt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dtValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("ParseDataType", (dt, dtValues))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        fInline: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ds, fInline))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Data+XDRSchema+NameType")]
#[repr(C)]
#[derive(Debug)]
pub struct XDRSchema_NameType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Data+XDRSchema+NameType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::XDRSchema_NameType {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "XDRSchema/NameType";
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
#[cfg(feature = "System+Data+XDRSchema+NameType")]
impl std::ops::Deref for crate::System::Data::XDRSchema_NameType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (n, t))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (n, t))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Data+XDRSchema+NameType")]
impl AsRef<crate::System::IComparable> for crate::System::Data::XDRSchema_NameType {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+XDRSchema+NameType")]
impl AsMut<crate::System::IComparable> for crate::System::Data::XDRSchema_NameType {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
