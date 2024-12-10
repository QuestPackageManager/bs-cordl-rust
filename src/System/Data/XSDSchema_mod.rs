#[cfg(feature = "System+Data+XSDSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct XSDSchema {
    __cordl_parent: crate::System::Data::XMLSchema,
    pub _schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
    pub _dsElement: *mut crate::System::Xml::Schema::XmlSchemaElement,
    pub _ds: *mut crate::System::Data::DataSet,
    pub _schemaName: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn AddTablesToList(
        &mut self,
        tableList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Data::DataTable,
            >,
        >,
        dt: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTablesToList", (tableList, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildKey(
        &mut self,
        keyNode: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        > = __cordl_object.invoke("BuildKey", (keyNode, table))?;
        Ok(__cordl_ret.into())
    }
    pub fn CollectElementsAnnotations_ArrayList1(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        schemaList: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CollectElementsAnnotations", (schema, schemaList))?;
        Ok(__cordl_ret.into())
    }
    pub fn CollectElementsAnnotations_XmlSchema0(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CollectElementsAnnotations", (schema))?;
        Ok(__cordl_ret.into())
    }
    pub fn DatasetElementCount(
        &mut self,
        elements: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DatasetElementCount", (elements))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindDatasetElement(
        &mut self,
        elements: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = __cordl_object.invoke("FindDatasetElement", (elements))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindField(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        field: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = __cordl_object
            .invoke("FindField", (table, field))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindTypeNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnnotated>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnnotated>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnnotated,
        > = __cordl_object.invoke("FindTypeNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBooleanAttribute(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnnotated,
        >,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defVal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetBooleanAttribute", (element, attrName, defVal))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceName(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnnotated>,
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
    pub fn GetNamespaceFromPrefix(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetNamespaceFromPrefix", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticle(
        &mut self,
        ct: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaParticle,
        > = __cordl_object.invoke("GetParticle", (ct))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParticleItems(
        &mut self,
        pt: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        > = __cordl_object.invoke("GetParticleItems", (pt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefix(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPrefix", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringAttribute(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnnotated,
        >,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defVal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetStringAttribute", (element, attrName, defVal))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTableName(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetTableName", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTableNamespace(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetTableNamespace", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAttributeColumn(
        &mut self,
        attrib: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        >,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAttributeColumn", (attrib, table, isBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAttributeGroup(
        &mut self,
        attributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAttributeGroup", (attributeGroup, table, isBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAttributes(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAttributes", (attributes, table, isBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleColumnExpression(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlAttribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColumnExpression", (instance, attrs))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleComplexType(
        &mut self,
        ct: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        tableChildren: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        isNillable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleComplexType", (ct, table, tableChildren, isNillable))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleConstraint(
        &mut self,
        keyNode: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConstraint", (keyNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDataSet(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
        isNewDataSet: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDataSet", (node, isNewDataSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleElementColumn(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleElementColumn", (elem, table, isBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleKeyref(
        &mut self,
        keyref: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaKeyref>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleKeyref", (keyref))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleParticle(
        &mut self,
        pt: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        tableChildren: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        isBase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleParticle", (pt, table, tableChildren, isBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRefTableProperties(
        &mut self,
        RefTables: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRefTableProperties", (RefTables, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRelation(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        fNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRelation", (node, fNested))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRelations(
        &mut self,
        ann: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnnotation>,
        fNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRelations", (ann, fNested))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSimpleContentColumn(
        &mut self,
        strType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isBase: bool,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlAttribute>,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleSimpleTypeSimpleContentColumn(
        &mut self,
        typeNode: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        >,
        strType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isBase: bool,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlAttribute>,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleTable(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
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
    pub fn HasAttributes(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasAttributes", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateSimpleTable(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("InstantiateSimpleTable", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateTable(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
        typeNode: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        >,
        isRef: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("InstantiateTable", (node, typeNode, isRef))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDatasetParticle(
        &mut self,
        pt: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDatasetParticle", (pt))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTable(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTable", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSchema_DataSet1(
        &mut self,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (schemaSet, ds))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSchema_DataTable0(
        &mut self,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
        dt: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSchema", (schemaSet, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseDataType(
        &mut self,
        dt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("ParseDataType", (dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FromInference(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_FromInference", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Data+XSDSchema+NameType")]
#[repr(C)]
#[derive(Debug)]
pub struct XSDSchema_NameType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _cordl_type: *mut crate::System::Type,
}
#[cfg(feature = "System+Data+XSDSchema+NameType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XSDSchema_NameType =>
    "System.Data"."XSDSchema/NameType"
);
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl std::ops::Deref for crate::System::Data::XSDSchema_NameType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XSDSchema_NameType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl AsRef<crate::System::IComparable> for crate::System::Data::XSDSchema_NameType {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+XSDSchema+NameType")]
impl AsMut<crate::System::IComparable> for crate::System::Data::XSDSchema_NameType {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
