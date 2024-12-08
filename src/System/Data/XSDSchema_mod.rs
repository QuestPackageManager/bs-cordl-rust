#[cfg(feature = "System+Data+XSDSchema+NameType")]
#[repr(C)]
#[derive(Debug)]
pub struct XSDSchema_NameType {
    __cordl_parent: crate::System::Object,
    pub name: *mut crate::System::String,
    pub _cordl_type: *mut crate::System::Type,
}
#[cfg(feature = "System+Data+XSDSchema+NameType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XSDSchema_NameType =>
    "System.Data"."XSDSchema/NameType"
);
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl std::ops::Deref for crate::System::Data::XSDSchema_NameType {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl std::ops::DerefMut for crate::System::Data::XSDSchema_NameType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl crate::System::Data::XSDSchema_NameType {
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
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XSDSchema_NameType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+XSDSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct XSDSchema {
    __cordl_parent: crate::System::Data::XMLSchema,
    pub _schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
    pub _dsElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
    pub _ds: *mut crate::System::Data::DataSet,
    pub _schemaName: *mut crate::System::String,
    pub _columnExpressions: *mut crate::System::Collections::ArrayList,
    pub _constraintNodes: *mut crate::System::Collections::Hashtable,
    pub _refTables: *mut crate::System::Collections::ArrayList,
    pub _complexTypes: *mut crate::System::Collections::ArrayList,
    pub _annotations: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    pub _elements: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    pub _attributes: *mut crate::System::Collections::Hashtable,
    pub _elementsTable: *mut crate::System::Collections::Hashtable,
    pub _attributeGroups: *mut crate::System::Collections::Hashtable,
    pub _schemaTypes: *mut crate::System::Collections::Hashtable,
    pub _expressions: *mut crate::System::Collections::Hashtable,
    pub _tableDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Data::DataTable,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataTable,
        >,
    >,
    pub _udSimpleTypes: *mut crate::System::Collections::Hashtable,
    pub _existingSimpleTypeMap: *mut crate::System::Collections::Hashtable,
    pub _fromInference: bool,
}
#[cfg(feature = "System+Data+XSDSchema")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XSDSchema => "System.Data"
    ."XSDSchema"
);
#[cfg(feature = "System+Data+XSDSchema")]
impl std::ops::Deref for crate::System::Data::XSDSchema {
    type Target = crate::System::Data::XMLSchema;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XSDSchema")]
impl std::ops::DerefMut for crate::System::Data::XSDSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XSDSchema")]
impl crate::System::Data::XSDSchema {
    #[cfg(feature = "System+Data+XSDSchema+NameType")]
    pub type NameType = crate::System::Data::XSDSchema_NameType;
    pub fn InstantiateTable(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaElement,
        typeNode: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        isRef: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("InstantiateTable", (node, typeNode, isRef))?;
        Ok(__cordl_ret)
    }
    pub fn set_FromInference(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FromInference", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleTable(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("HandleTable", (node))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAttributeColumn(
        &mut self,
        attrib: *mut crate::System::Xml::Schema::XmlSchemaAttribute,
        table: *mut crate::System::Data::DataTable,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAttributeColumn", (attrib, table, isBase))?;
        Ok(__cordl_ret)
    }
    pub fn HandleComplexType(
        &mut self,
        ct: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
        table: *mut crate::System::Data::DataTable,
        tableChildren: *mut crate::System::Collections::ArrayList,
        isNillable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleComplexType", (ct, table, tableChildren, isNillable))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateSimpleTable(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("InstantiateSimpleTable", (node))?;
        Ok(__cordl_ret)
    }
    pub fn get_FromInference(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_FromInference", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetParticle(
        &mut self,
        ct: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaParticle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaParticle = __cordl_object
            .invoke("GetParticle", (ct))?;
        Ok(__cordl_ret)
    }
    pub fn HandleKeyref(
        &mut self,
        keyref: *mut crate::System::Xml::Schema::XmlSchemaKeyref,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleKeyref", (keyref))?;
        Ok(__cordl_ret)
    }
    pub fn AddTablesToList(
        &mut self,
        tableList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataTable,
        >,
        dt: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTablesToList", (tableList, dt))?;
        Ok(__cordl_ret)
    }
    pub fn HandleColumnExpression(
        &mut self,
        instance: *mut crate::System::Object,
        attrs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColumnExpression", (instance, attrs))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringAttribute(
        &mut self,
        element: *mut crate::System::Xml::Schema::XmlSchemaAnnotated,
        attrName: *mut crate::System::String,
        defVal: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStringAttribute", (element, attrName, defVal))?;
        Ok(__cordl_ret)
    }
    pub fn FindField(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        field: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataColumn> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataColumn = __cordl_object
            .invoke("FindField", (table, field))?;
        Ok(__cordl_ret)
    }
    pub fn HandleRelation(
        &mut self,
        node: *mut crate::System::Xml::XmlElement,
        fNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRelation", (node, fNested))?;
        Ok(__cordl_ret)
    }
    pub fn HandleParticle(
        &mut self,
        pt: *mut crate::System::Xml::Schema::XmlSchemaParticle,
        table: *mut crate::System::Data::DataTable,
        tableChildren: *mut crate::System::Collections::ArrayList,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleParticle", (pt, table, tableChildren, isBase))?;
        Ok(__cordl_ret)
    }
    pub fn HasAttributes(
        &mut self,
        attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasAttributes", (attributes))?;
        Ok(__cordl_ret)
    }
    pub fn FindDatasetElement(
        &mut self,
        elements: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("FindDatasetElement", (elements))?;
        Ok(__cordl_ret)
    }
    pub fn GetNamespaceFromPrefix(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetNamespaceFromPrefix", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn BuildKey(
        &mut self,
        keyNode: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("BuildKey", (keyNode, table))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSchema_DataTable0(
        &mut self,
        schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
        dt: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (schemaSet, dt))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSchema_DataSet1(
        &mut self,
        schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
        ds: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (schemaSet, ds))?;
        Ok(__cordl_ret)
    }
    pub fn HandleRelations(
        &mut self,
        ann: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
        fNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRelations", (ann, fNested))?;
        Ok(__cordl_ret)
    }
    pub fn CollectElementsAnnotations_XmlSchema0(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CollectElementsAnnotations", (schema))?;
        Ok(__cordl_ret)
    }
    pub fn CollectElementsAnnotations_ArrayList1(
        &mut self,
        schema: *mut crate::System::Xml::Schema::XmlSchema,
        schemaList: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CollectElementsAnnotations", (schema, schemaList))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSimpleContentColumn(
        &mut self,
        strType: *mut crate::System::String,
        table: *mut crate::System::Data::DataTable,
        isBase: bool,
        attrs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlAttribute,
        >,
        isNillable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSimpleContentColumn",
                (strType, table, isBase, attrs, isNillable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSimpleTypeSimpleContentColumn(
        &mut self,
        typeNode: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
        strType: *mut crate::System::String,
        table: *mut crate::System::Data::DataTable,
        isBase: bool,
        attrs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlAttribute,
        >,
        isNillable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSimpleTypeSimpleContentColumn",
                (typeNode, strType, table, isBase, attrs, isNillable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleAttributeGroup(
        &mut self,
        attributeGroup: *mut crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        table: *mut crate::System::Data::DataTable,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAttributeGroup", (attributeGroup, table, isBase))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAttributes(
        &mut self,
        attributes: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        table: *mut crate::System::Data::DataTable,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAttributes", (attributes, table, isBase))?;
        Ok(__cordl_ret)
    }
    pub fn GetParticleItems(
        &mut self,
        pt: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection = __cordl_object
            .invoke("GetParticleItems", (pt))?;
        Ok(__cordl_ret)
    }
    pub fn GetTableNamespace(
        &mut self,
        key: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTableNamespace", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetBooleanAttribute(
        &mut self,
        element: *mut crate::System::Xml::Schema::XmlSchemaAnnotated,
        attrName: *mut crate::System::String,
        defVal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetBooleanAttribute", (element, attrName, defVal))?;
        Ok(__cordl_ret)
    }
    pub fn HandleRefTableProperties(
        &mut self,
        RefTables: *mut crate::System::Collections::ArrayList,
        element: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRefTableProperties", (RefTables, element))?;
        Ok(__cordl_ret)
    }
    pub fn FindTypeNode(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaAnnotated,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAnnotated,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAnnotated = __cordl_object
            .invoke("FindTypeNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn HandleElementColumn(
        &mut self,
        elem: *mut crate::System::Xml::Schema::XmlSchemaElement,
        table: *mut crate::System::Data::DataTable,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleElementColumn", (elem, table, isBase))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsDatasetParticle(
        &mut self,
        pt: *mut crate::System::Xml::Schema::XmlSchemaParticle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDatasetParticle", (pt))?;
        Ok(__cordl_ret)
    }
    pub fn GetTableName(
        &mut self,
        key: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTableName", (key))?;
        Ok(__cordl_ret)
    }
    pub fn DatasetElementCount(
        &mut self,
        elements: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DatasetElementCount", (elements))?;
        Ok(__cordl_ret)
    }
    pub fn HandleConstraint(
        &mut self,
        keyNode: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConstraint", (keyNode))?;
        Ok(__cordl_ret)
    }
    pub fn HandleDataSet(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaElement,
        isNewDataSet: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDataSet", (node, isNewDataSet))?;
        Ok(__cordl_ret)
    }
    pub fn GetPrefix(
        &mut self,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetPrefix", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn GetInstanceName(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaAnnotated,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetInstanceName", (node))?;
        Ok(__cordl_ret)
    }
    pub fn ParseDataType(
        &mut self,
        dt: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ParseDataType", (dt))?;
        Ok(__cordl_ret)
    }
    pub fn IsTable(
        &mut self,
        node: *mut crate::System::Xml::Schema::XmlSchemaElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTable", (node))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+XSDSchema")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XSDSchema {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
