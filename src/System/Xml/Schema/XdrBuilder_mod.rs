#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder {
    __cordl_parent: crate::System::Xml::Schema::SchemaBuilder,
    pub _SchemaInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
    pub _TargetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub positionInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::PositionInfo>,
    pub _contentValidator: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::ParticleContentValidator,
    >,
    pub _CurState: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_XdrEntry,
    >,
    pub _NextState: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_XdrEntry,
    >,
    pub _StateHistory: quest_hook::libil2cpp::Gc<crate::System::Xml::HWStack>,
    pub _GroupStack: quest_hook::libil2cpp::Gc<crate::System::Xml::HWStack>,
    pub _XdrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _XdrPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _ElementDef: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_ElementContent,
    >,
    pub _GroupDef: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_GroupContent,
    >,
    pub _AttributeDef: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_AttributeContent,
    >,
    pub _UndefinedAttributeTypes: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
    >,
    pub _BaseDecl: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
    >,
    pub _NameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    pub _SchemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
    pub _CurNsMgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    pub _Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub validationEventHandler: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::ValidationEventHandler,
    >,
    pub _UndeclaredElements: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub xmlResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder =>
    "System.Xml.Schema"."XdrBuilder"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder {
    type Target = crate::System::Xml::Schema::SchemaBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl crate::System::Xml::Schema::XdrBuilder {
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
    pub type AttributeContent = crate::System::Xml::Schema::XdrBuilder_AttributeContent;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
    pub type DeclBaseInfo = crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
    pub type ElementContent = crate::System::Xml::Schema::XdrBuilder_ElementContent;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
    pub type GroupContent = crate::System::Xml::Schema::XdrBuilder_GroupContent;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
    pub type XdrAttributeEntry = crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
    pub type XdrBeginChildFunction = crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
    pub type XdrBuildFunction = crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
    pub type XdrEndChildFunction = crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
    pub type XdrEntry = crate::System::Xml::Schema::XdrBuilder_XdrEntry;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
    pub type XdrInitFunction = crate::System::Xml::Schema::XdrBuilder_XdrInitFunction;
    pub fn AddOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOrder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckDatatype(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaDatatype>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaDatatype,
        > = __cordl_object.invoke("CheckDatatype", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckDefaultAttValue(
        &mut self,
        attDef: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckDefaultAttValue", (attDef))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareMinMaxLength(
        cMin: u32,
        cMax: u32,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareMinMaxLength", (cMin, cMax, builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndChildren", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContent(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetContent", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModel(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetModel", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextState(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetNextState", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrder(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOrder", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMinMax(
        pContent: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ParticleContentValidator,
        >,
        cMin: u32,
        cMax: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HandleMinMax", (pContent, cMin, cMax))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsContentParsed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsContentParsed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGlobal(&mut self, flags: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsGlobal", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSkipableAttribute(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSkipableAttribute", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSkipableElement(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSkipableElement", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsXdrSchema(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsXdrSchema", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsYes(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsYes", (obj, builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSchema(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LoadSchema", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        curmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        sinfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
        targetNamspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventhandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    reader,
                    curmgr,
                    sinfo,
                    targetNamspace,
                    nameTable,
                    schemaNames,
                    eventhandler,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ParseDtMaxLength(
        cVal: quest_hook::libil2cpp::ByRefMut<u32>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDtMaxLength", (cVal, obj, builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDtMinLength(
        cVal: quest_hook::libil2cpp::ByRefMut<u32>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDtMinLength", (cVal, obj, builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseInteger(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        n: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseInteger", (str, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseMaxOccurs(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseMaxOccurs", (obj, builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseMinOccurs(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseMinOccurs", (obj, builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PopGroupInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopGroupInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAttribute(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAttribute", (prefix, name, ns, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessElement(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessElement", (prefix, name, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMarkup(
        &mut self,
        markup: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlNode>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMarkup", (markup))?;
        Ok(__cordl_ret.into())
    }
    pub fn Push(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PushGroupInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushGroupInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_Il2CppString1(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppArray_XmlSeverityType0(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, severity))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString2(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_XmlSchemaException_XmlSeverityType3(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e, severity))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributePresence(
        &mut self,
        pAttdef: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
        fRequired: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttributePresence", (pAttdef, fRequired))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartChildren", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BeginAttribute(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BeginAttribute", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BeginAttributeType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BeginAttributeType", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BeginElementType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BeginElementType", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BeginRoot(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BeginRoot", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttributeType_Default(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttributeType_Default", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttributeType_DtMaxLength(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttributeType_DtMaxLength", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttributeType_DtMinLength(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttributeType_DtMinLength", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttributeType_DtType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttributeType_DtType", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttributeType_DtValues(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttributeType_DtValues", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttributeType_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttributeType_Name", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttributeType_Required(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttributeType_Required", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttribute_Default(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttribute_Default", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttribute_Required(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttribute_Required", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildAttribute_Type(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildAttribute_Type", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_Content(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_Content", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_DtMaxLength(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_DtMaxLength", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_DtMinLength(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_DtMinLength", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_DtType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_DtType", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_DtValues(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_DtValues", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_Model(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_Model", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_Name", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElementType_Order(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElementType_Order", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElement_MaxOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElement_MaxOccurs", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElement_MinOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElement_MinOccurs", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildElement_Type(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildElement_Type", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildGroup_MaxOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildGroup_MaxOccurs", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildGroup_MinOccurs(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildGroup_MinOccurs", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildGroup_Order(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildGroup_Order", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildRoot_ID(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildRoot_ID", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_BuildRoot_Name(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_BuildRoot_Name", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_CheckAttributeDefault(
        &mut self,
        decl: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
        >,
        pAttdef: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("XDR_CheckAttributeDefault", (decl, pAttdef))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndAttribute(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndAttribute", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndAttributeDtType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndAttributeDtType", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndAttributeType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndAttributeType", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndElement(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndElement", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndElementDtType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndElementDtType", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndElementType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndElementType", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndGroup(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndGroup", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_EndRoot(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_EndRoot", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitAttribute(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitAttribute", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitAttributeDtType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitAttributeDtType", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitAttributeType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitAttributeType", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitElement(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitElement", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitElementDtType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitElementDtType", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitElementType(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitElementType", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitGroup(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitGroup", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn XDR_InitRoot(
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XDR_InitRoot", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        curmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
        sinfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
        targetNamspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventhandler: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ValidationEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    reader,
                    curmgr,
                    sinfo,
                    targetNamspace,
                    nameTable,
                    schemaNames,
                    eventhandler,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_XmlResolver(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlResolver", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XdrBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_AttributeContent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _AttDef: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
    pub _Name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub _Prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _Required: bool,
    pub _MinVal: u32,
    pub _MaxVal: u32,
    pub _MaxLength: u32,
    pub _MinLength: u32,
    pub _EnumerationRequired: bool,
    pub _HasDataType: bool,
    pub _Global: bool,
    pub _Default: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_AttributeContent
    => "System.Xml.Schema"."XdrBuilder/AttributeContent"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_AttributeContent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_AttributeContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl crate::System::Xml::Schema::XdrBuilder_AttributeContent {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_AttributeContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_DeclBaseInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub _Prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _TypeName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub _TypePrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _Default: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _Revises: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _MaxOccurs: u32,
    pub _MinOccurs: u32,
    pub _Checking: bool,
    pub _ElementDecl: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub _Attdef: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
    pub _Next: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
    >,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_DeclBaseInfo =>
    "System.Xml.Schema"."XdrBuilder/DeclBaseInfo"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
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
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_ElementContent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ElementDecl: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SchemaElementDecl,
    >,
    pub _ContentAttr: i32,
    pub _OrderAttr: i32,
    pub _MasterGroupRequired: bool,
    pub _ExistTerminal: bool,
    pub _AllowDataType: bool,
    pub _HasDataType: bool,
    pub _HasType: bool,
    pub _EnumerationRequired: bool,
    pub _MinVal: u32,
    pub _MaxVal: u32,
    pub _MaxLength: u32,
    pub _MinLength: u32,
    pub _AttDefList: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_ElementContent
    => "System.Xml.Schema"."XdrBuilder/ElementContent"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_ElementContent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_ElementContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl crate::System::Xml::Schema::XdrBuilder_ElementContent {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_ElementContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_GroupContent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _MinVal: u32,
    pub _MaxVal: u32,
    pub _HasMaxAttr: bool,
    pub _HasMinAttr: bool,
    pub _Order: i32,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_GroupContent =>
    "System.Xml.Schema"."XdrBuilder/GroupContent"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_GroupContent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_GroupContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl crate::System::Xml::Schema::XdrBuilder_GroupContent {
    pub fn Copy_XdrBuilder_GroupContent0(
        from: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_GroupContent,
        >,
        to: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_GroupContent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_XdrBuilder_GroupContent1(
        other: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_GroupContent,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder_GroupContent>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_GroupContent,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Copy", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_GroupContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrAttributeEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Attribute: crate::System::Xml::Schema::SchemaNames_Token,
    pub _SchemaFlags: i32,
    pub _Datatype: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaDatatype,
    >,
    pub _BuildFunc: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
    >,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XdrBuilder_XdrAttributeEntry => "System.Xml.Schema"
    ."XdrBuilder/XdrAttributeEntry"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    pub fn New_XdrBuilder_XdrBuildFunction0(
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        build: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a, ttype, build))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_XdrBuilder_XdrBuildFunction1(
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        schemaFlags: i32,
        build: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a, ttype, schemaFlags, build))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_XdrBuilder_XdrBuildFunction0(
        &mut self,
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        build: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a, ttype, build))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_XdrBuilder_XdrBuildFunction1(
        &mut self,
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        schemaFlags: i32,
        build: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a, ttype, schemaFlags, build))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrBeginChildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction => "System.Xml.Schema"
    ."XdrBuilder/XdrBeginChildFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    pub fn Invoke(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrBuildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_XdrBuildFunction
    => "System.Xml.Schema"."XdrBuilder/XdrBuildFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    pub fn Invoke(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, obj, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrEndChildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XdrBuilder_XdrEndChildFunction => "System.Xml.Schema"
    ."XdrBuilder/XdrEndChildFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    pub fn Invoke(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Name: crate::System::Xml::Schema::SchemaNames_Token,
    pub _NextStates: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _Attributes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry,
        >,
    >,
    pub _InitFunc: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_XdrInitFunction,
    >,
    pub _BeginChildFunc: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction,
    >,
    pub _EndChildFunc: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction,
    >,
    pub _AllowText: bool,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_XdrEntry =>
    "System.Xml.Schema"."XdrBuilder/XdrEntry"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    pub fn New(
        n: crate::System::Xml::Schema::SchemaNames_Token,
        states: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry,
            >,
        >,
        init: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrInitFunction,
        >,
        begin: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction,
        >,
        end: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction,
        >,
        fText: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (n, states, attributes, init, begin, end, fText))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        n: crate::System::Xml::Schema::SchemaNames_Token,
        states: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry,
            >,
        >,
        init: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrInitFunction,
        >,
        begin: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction,
        >,
        end: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction,
        >,
        fText: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (n, states, attributes, init, begin, end, fText))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrInitFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_XdrInitFunction
    => "System.Xml.Schema"."XdrBuilder/XdrInitFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    pub fn Invoke(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XdrBuilder>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
