#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_objectQueue: *mut crate::System::Collections::Queue,
    pub m_idGenerator: *mut crate::System::Runtime::Serialization::ObjectIDGenerator,
    pub m_currentId: i32,
    pub m_surrogates: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    pub m_context: crate::System::Runtime::Serialization::StreamingContext,
    pub serWriter: *mut crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
    pub m_objectManager: *mut crate::System::Runtime::Serialization::SerializationObjectManager,
    pub topId: i64,
    pub topName: *mut quest_hook::libil2cpp::Il2CppString,
    pub headers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Runtime::Remoting::Messaging::Header,
    >,
    pub formatterEnums: *mut crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
    pub m_binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    pub serObjectInfoInit: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
    pub m_formatterConverter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
    pub crossAppDomainArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub previousObj: *mut quest_hook::libil2cpp::Il2CppObject,
    pub previousId: i64,
    pub previousType: *mut crate::System::Type,
    pub previousCode: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub assemblyToIdTable: *mut crate::System::Collections::Hashtable,
    pub niPool: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ObjectWriter =>
    "System.Runtime.Serialization.Formatters.Binary"."ObjectWriter"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    pub fn CheckForNull(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckForNull", (objectInfo, memberNameInfo, typeNameInfo, data))?;
        Ok(__cordl_ret)
    }
    pub fn CheckTypeFormat(
        &mut self,
        test: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
        want: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckTypeFormat", (test, want))?;
        Ok(__cordl_ret)
    }
    pub fn GetAssemblyId(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetAssemblyId", (objectInfo))?;
        Ok(__cordl_ret)
    }
    pub fn GetNameInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo = __cordl_object
            .invoke("GetNameInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNext(
        &mut self,
        objID: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetNext", (objID))?;
        Ok(__cordl_ret)
    }
    pub fn GetType(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetType", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetId(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        assignUniqueIdToValueType: bool,
        _cordl_type: *mut crate::System::Type,
        isNew: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke(
                "InternalGetId",
                (obj, assignUniqueIdToValueType, _cordl_type, isNew),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MemberToNameInfo(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo = __cordl_object
            .invoke("MemberToNameInfo", (name))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        formatterEnums: *mut crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
        binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (selector, context, formatterEnums, binder))?;
        Ok(__cordl_object)
    }
    pub fn PutNameInfo(
        &mut self,
        nameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PutNameInfo", (nameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn Schedule_Il2CppObject__cordl_bool_Type0(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        assignUniqueIdToValueType: bool,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("Schedule", (obj, assignUniqueIdToValueType, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn Schedule_WriteObjectInfo1(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        assignUniqueIdToValueType: bool,
        _cordl_type: *mut crate::System::Type,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke(
                "Schedule",
                (obj, assignUniqueIdToValueType, _cordl_type, objectInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        graph: *mut quest_hook::libil2cpp::Il2CppObject,
        inHeaders: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::Remoting::Messaging::Header,
        >,
        serWriter: *mut crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
        fCheck: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (graph, inHeaders, serWriter, fCheck))?;
        Ok(__cordl_ret)
    }
    pub fn ToCode(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE = __cordl_object
            .invoke("ToCode", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn TypeToNameInfo_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo = __cordl_object
            .invoke("TypeToNameInfo", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn TypeToNameInfo_Type_NameInfo4(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        nameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TypeToNameInfo", (_cordl_type, nameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn TypeToNameInfo_Type_WriteObjectInfo_InternalPrimitiveTypeE_NameInfo0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        nameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo = __cordl_object
            .invoke("TypeToNameInfo", (_cordl_type, objectInfo, code, nameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn TypeToNameInfo_WriteObjectInfo2(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo = __cordl_object
            .invoke("TypeToNameInfo", (objectInfo))?;
        Ok(__cordl_ret)
    }
    pub fn TypeToNameInfo_WriteObjectInfo_NameInfo3(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        nameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo = __cordl_object
            .invoke("TypeToNameInfo", (objectInfo, nameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        memberObjectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (objectInfo, memberNameInfo, memberObjectInfo))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArrayMember(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        arrayElemTypeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArrayMember", (objectInfo, arrayElemTypeNameInfo, data))?;
        Ok(__cordl_ret)
    }
    pub fn WriteKnownValueClass(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WriteKnownValueClass", (memberNameInfo, typeNameInfo, data))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMemberSetup(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        memberName: *mut quest_hook::libil2cpp::Il2CppString,
        memberType: *mut crate::System::Type,
        memberData: *mut quest_hook::libil2cpp::Il2CppObject,
        memberObjectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteMemberSetup",
                (
                    objectInfo,
                    memberNameInfo,
                    typeNameInfo,
                    memberName,
                    memberType,
                    memberData,
                    memberObjectInfo,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteMembers(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        memberTypeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        memberData: *mut quest_hook::libil2cpp::Il2CppObject,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        memberObjectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteMembers",
                (
                    memberNameInfo,
                    memberTypeNameInfo,
                    memberData,
                    objectInfo,
                    typeNameInfo,
                    memberObjectInfo,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteObjectRef(
        &mut self,
        nameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        objectId: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectRef", (nameInfo, objectId))?;
        Ok(__cordl_ret)
    }
    pub fn WriteRectangle(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        rank: i32,
        maxA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        array: *mut crate::System::Array,
        arrayElemNameTypeInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        lowerBoundA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteRectangle",
                (objectInfo, rank, maxA, array, arrayElemNameTypeInfo, lowerBoundA),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteSerializedStreamHeader(
        &mut self,
        topId: i64,
        headerId: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSerializedStreamHeader", (topId, headerId))?;
        Ok(__cordl_ret)
    }
    pub fn WriteString(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        stringObject: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (memberNameInfo, typeNameInfo, stringObject))?;
        Ok(__cordl_ret)
    }
    pub fn Write_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray1(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        memberData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        memberObjectInfos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Write",
                (
                    objectInfo,
                    memberNameInfo,
                    typeNameInfo,
                    memberNames,
                    memberTypes,
                    memberData,
                    memberObjectInfos,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Write_WriteObjectInfo_NameInfo_NameInfo0(
        &mut self,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (objectInfo, memberNameInfo, typeNameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
        context: crate::System::Runtime::Serialization::StreamingContext,
        formatterEnums: *mut crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
        binder: *mut crate::System::Runtime::Serialization::SerializationBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (selector, context, formatterEnums, binder))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::SerializationObjectManager,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::SerializationObjectManager = __cordl_object
            .invoke("get_ObjectManager", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
