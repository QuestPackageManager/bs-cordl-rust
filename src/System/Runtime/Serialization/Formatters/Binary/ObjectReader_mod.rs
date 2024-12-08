#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectReader {
    __cordl_parent: crate::System::Object,
    pub m_stream: *mut crate::System::IO::Stream,
    pub m_surrogates: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    pub m_context: crate::System::Runtime::Serialization::StreamingContext,
    pub m_objectManager: *mut crate::System::Runtime::Serialization::ObjectManager,
    pub formatterEnums: *mut crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
    pub m_binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    pub topId: i64,
    pub bSimpleAssembly: bool,
    pub handlerObject: *mut crate::System::Object,
    pub m_topObject: *mut crate::System::Object,
    pub headers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Runtime::Remoting::Messaging::Header,
    >,
    pub handler: *mut crate::System::Runtime::Remoting::Messaging::HeaderHandler,
    pub serObjectInfoInit: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
    pub m_formatterConverter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
    pub stack: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
    pub valueFixupStack: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
    pub crossAppDomainArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Object,
    >,
    pub bFullDeserialization: bool,
    pub bOldFormatDetected: bool,
    pub valTypeObjectIdTable: *mut crate::System::Runtime::Serialization::Formatters::Binary::IntSizedArray,
    pub typeCache: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameCache,
    pub previousAssemblyString: *mut crate::System::String,
    pub previousName: *mut crate::System::String,
    pub previousType: *mut crate::System::Type,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ObjectReader =>
    "System.Runtime.Serialization.Formatters.Binary"."ObjectReader"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader {
    #[cfg(
        feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TypeNAssembly"
    )]
    pub type TypeNAssembly = crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TypeNAssembly;
    #[cfg(
        feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TopLevelAssemblyTypeResolver"
    )]
    pub type TopLevelAssemblyTypeResolver = crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TopLevelAssemblyTypeResolver;
    pub fn CreateReadObjectInfo_Type0(
        &mut self,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo = __cordl_object
            .invoke("CreateReadObjectInfo", (objectType))?;
        Ok(__cordl_ret)
    }
    pub fn CreateReadObjectInfo_Il2CppArray_Il2CppArray1(
        &mut self,
        objectType: *mut crate::System::Type,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo = __cordl_object
            .invoke("CreateReadObjectInfo", (objectType, memberNames, memberTypes))?;
        Ok(__cordl_ret)
    }
    pub fn ParseObject(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseObject", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn ParseObjectEnd(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseObjectEnd", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn NextRectangleMap(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NextRectangleMap", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn ParseArrayMemberEnd(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseArrayMemberEnd", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn GetId(&mut self, objectId: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetId", (objectId))?;
        Ok(__cordl_ret)
    }
    pub fn ParseString(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        parentPr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseString", (pr, parentPr))?;
        Ok(__cordl_ret)
    }
    pub fn ParseSerializedStreamHeaderEnd(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseSerializedStreamHeaderEnd", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn Bind(
        &mut self,
        assemblyString: *mut crate::System::String,
        typeString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("Bind", (assemblyString, typeString))?;
        Ok(__cordl_ret)
    }
    pub fn ParseArrayMember(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseArrayMember", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn ParseError(
        &mut self,
        processing: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        onStack: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseError", (processing, onStack))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        handler: *mut crate::System::Runtime::Remoting::Messaging::HeaderHandler,
        serParser: *mut crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser,
        fCheck: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (handler, serParser, fCheck))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        formatterEnums: *mut crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
        binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, selector, context, formatterEnums, binder))?;
        Ok(__cordl_ret)
    }
    pub fn ParseSerializedStreamHeader(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseSerializedStreamHeader", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMember(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseMember", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueFixupStack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerStack = __cordl_object
            .invoke("get_ValueFixupStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasSurrogate(
        &mut self,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasSurrogate", (t))?;
        Ok(__cordl_ret)
    }
    pub fn InitFullDeserialization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitFullDeserialization", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TopObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_TopObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_TopObject(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TopObject", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ParseArray(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseArray", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn Parse(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Parse", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn FastBindToType(
        &mut self,
        assemblyName: *mut crate::System::String,
        typeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("FastBindToType", (assemblyName, typeName))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMemberEnd(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseMemberEnd", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn CheckSerializable(
        &mut self,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckSerializable", (t))?;
        Ok(__cordl_ret)
    }
    pub fn CrossAppDomainArray(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CrossAppDomainArray", (index))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterObject_Object_ParseRecord_ParseRecord0(
        &mut self,
        obj: *mut crate::System::Object,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        objectPr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterObject", (obj, pr, objectPr))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterObject__cordl_bool1(
        &mut self,
        obj: *mut crate::System::Object,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        objectPr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        bIsString: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterObject", (obj, pr, objectPr, bIsString))?;
        Ok(__cordl_ret)
    }
    pub fn GetType(
        &mut self,
        assemblyInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetType", (assemblyInfo, name))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        stream: *mut crate::System::IO::Stream,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        formatterEnums: *mut crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
        binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, selector, context, formatterEnums, binder))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TopLevelAssemblyTypeResolver"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectReader_TopLevelAssemblyTypeResolver {
    __cordl_parent: crate::System::Object,
    pub m_topLevelAssembly: *mut crate::System::Reflection::Assembly,
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TopLevelAssemblyTypeResolver"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TopLevelAssemblyTypeResolver
    => "System.Runtime.Serialization.Formatters.Binary"
    ."ObjectReader/TopLevelAssemblyTypeResolver"
);
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TopLevelAssemblyTypeResolver"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TopLevelAssemblyTypeResolver {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TopLevelAssemblyTypeResolver"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TopLevelAssemblyTypeResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TopLevelAssemblyTypeResolver"
)]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TopLevelAssemblyTypeResolver {
    pub fn ResolveType(
        &mut self,
        assembly: *mut crate::System::Reflection::Assembly,
        simpleTypeName: *mut crate::System::String,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ResolveType", (assembly, simpleTypeName, ignoreCase))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        topLevelAssembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (topLevelAssembly))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        topLevelAssembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (topLevelAssembly))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TopLevelAssemblyTypeResolver"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TopLevelAssemblyTypeResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TypeNAssembly"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectReader_TypeNAssembly {
    __cordl_parent: crate::System::Object,
    pub _cordl_type: *mut crate::System::Type,
    pub assemblyName: *mut crate::System::String,
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TypeNAssembly"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TypeNAssembly =>
    "System.Runtime.Serialization.Formatters.Binary"."ObjectReader/TypeNAssembly"
);
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TypeNAssembly"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TypeNAssembly {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TypeNAssembly"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TypeNAssembly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TypeNAssembly"
)]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TypeNAssembly {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+ObjectReader+TypeNAssembly"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader_TypeNAssembly {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
