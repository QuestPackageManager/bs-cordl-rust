#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct __BinaryWriter {
    __cordl_parent: crate::System::Object,
    pub sout: *mut crate::System::IO::Stream,
    pub formatterTypeStyle: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
    pub objectMapTable: *mut crate::System::Collections::Hashtable,
    pub objectWriter: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter,
    pub dataWriter: *mut crate::System::IO::BinaryWriter,
    pub m_nestedObjectCount: i32,
    pub nullCount: i32,
    pub binaryMethodCall: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryMethodCall,
    pub binaryMethodReturn: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryMethodReturn,
    pub binaryObject: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObject,
    pub binaryObjectWithMap: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMap,
    pub binaryObjectWithMapTyped: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped,
    pub binaryObjectString: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectString,
    pub binaryArray: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryArray,
    pub byteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub chunkSize: i32,
    pub memberPrimitiveUnTyped: *mut crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped,
    pub memberPrimitiveTyped: *mut crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveTyped,
    pub objectNull: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectNull,
    pub memberReference: *mut crate::System::Runtime::Serialization::Formatters::Binary::MemberReference,
    pub binaryAssembly: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssembly,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter =>
    "System.Runtime.Serialization.Formatters.Binary"."__BinaryWriter"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryWriter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryWriter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryWriter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter {
    pub fn WriteChar(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InternalWriteItemNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteItemNull", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullItem(
        &mut self,
        itemNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullItem", (itemNameInfo, typeNameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMethodCall(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMethodCall", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteObjectString(
        &mut self,
        objectId: i32,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectString", (objectId, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteString(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMemberObjectRef(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        idRef: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMemberObjectRef", (memberNameInfo, idRef))?;
        Ok(__cordl_ret)
    }
    pub fn WriteValue(
        &mut self,
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (code, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAssembly(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        assemblyString: *mut crate::System::String,
        assemId: i32,
        isNew: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteAssembly", (_cordl_type, assemblyString, assemId, isNew))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSingleArray(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        arrayNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        arrayElemTypeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        length: i32,
        lowerBound: i32,
        array: *mut crate::System::Array,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteSingleArray",
                (
                    memberNameInfo,
                    arrayNameInfo,
                    objectInfo,
                    arrayElemTypeNameInfo,
                    length,
                    lowerBound,
                    array,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteDelayedNullItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDelayedNullItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteSerializationHeaderEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSerializationHeaderEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteInt16(
        &mut self,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteInt16", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteTimeSpan(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTimeSpan", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteJaggedArray(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        arrayNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        arrayElemTypeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        length: i32,
        lowerBound: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteJaggedArray",
                (
                    memberNameInfo,
                    arrayNameInfo,
                    objectInfo,
                    arrayElemTypeNameInfo,
                    length,
                    lowerBound,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteItemObjectRef(
        &mut self,
        nameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        idRef: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteItemObjectRef", (nameInfo, idRef))?;
        Ok(__cordl_ret)
    }
    pub fn WriteInt32(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteInt32", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObject(
        &mut self,
        nameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        numMembers: i32,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        memberObjectInfos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteObject",
                (
                    nameInfo,
                    typeNameInfo,
                    numMembers,
                    memberNames,
                    memberTypes,
                    memberObjectInfos,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        sout: *mut crate::System::IO::Stream,
        objectWriter: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter,
        formatterTypeStyle: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sout, objectWriter, formatterTypeStyle))?;
        Ok(__cordl_ret)
    }
    pub fn WriteBytes_Il2CppArray0(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBytes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteBytes_i32_i32_1(
        &mut self,
        byteA: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBytes", (byteA, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn WriteBegin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBegin", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteBoolean(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBoolean", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSerializationHeader(
        &mut self,
        topId: i32,
        headerId: i32,
        minorVersion: i32,
        majorVersion: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteSerializationHeader",
                (topId, headerId, minorVersion, majorVersion),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteChars(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteDouble(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDouble", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteInt64(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteInt64", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteDecimal(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDecimal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSByte(
        &mut self,
        value: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSByte", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArrayAsBytes(
        &mut self,
        array: *mut crate::System::Array,
        typeLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArrayAsBytes", (array, typeLength))?;
        Ok(__cordl_ret)
    }
    pub fn WriteRectangleArray(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        arrayNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        arrayElemTypeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        rank: i32,
        lengthA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        lowerBoundA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteRectangleArray",
                (
                    memberNameInfo,
                    arrayNameInfo,
                    objectInfo,
                    arrayElemTypeNameInfo,
                    rank,
                    lengthA,
                    lowerBoundA,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteNullMember(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNullMember", (memberNameInfo, typeNameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMemberString(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMemberString", (memberNameInfo, typeNameInfo, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMember(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMember", (memberNameInfo, typeNameInfo, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteByte(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteByte", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteItem(
        &mut self,
        itemNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteItem", (itemNameInfo, typeNameInfo, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteItemEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteItemEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteDateTime(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDateTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteUInt32(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteUInt32", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMethodReturn(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMethodReturn", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteSingle(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSingle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObjectByteArray(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        arrayNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        arrayElemTypeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        length: i32,
        lowerBound: i32,
        byteA: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteObjectByteArray",
                (
                    memberNameInfo,
                    arrayNameInfo,
                    objectInfo,
                    arrayElemTypeNameInfo,
                    length,
                    lowerBound,
                    byteA,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteMemberNested(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMemberNested", (memberNameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn WriteUInt16(
        &mut self,
        value: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteUInt16", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteUInt64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteUInt64", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteObjectEnd(
        &mut self,
        memberNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        typeNameInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObjectEnd", (memberNameInfo, typeNameInfo))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sout: *mut crate::System::IO::Stream,
        objectWriter: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter,
        formatterTypeStyle: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sout, objectWriter, formatterTypeStyle))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
