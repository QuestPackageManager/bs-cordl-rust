#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryParser")]
#[repr(C)]
#[derive(Debug)]
pub struct __BinaryParser {
    __cordl_parent: crate::System::Object,
    pub objectReader: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
    pub input: *mut crate::System::IO::Stream,
    pub topId: i64,
    pub headerId: i64,
    pub objectMapIdTable: *mut crate::System::Runtime::Serialization::Formatters::Binary::SizedArray,
    pub assemIdToAssemblyTable: *mut crate::System::Runtime::Serialization::Formatters::Binary::SizedArray,
    pub stack: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
    pub expectedType: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    pub expectedTypeInformation: *mut crate::System::Object,
    pub PRS: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    pub systemAssemblyInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
    pub dataReader: *mut crate::System::IO::BinaryReader,
    pub opPool: *mut crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
    pub binaryObject: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObject,
    pub bowm: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMap,
    pub bowmt: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped,
    pub objectString: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectString,
    pub crossAppDomainString: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryCrossAppDomainString,
    pub memberPrimitiveTyped: *mut crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveTyped,
    pub byteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub memberPrimitiveUnTyped: *mut crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped,
    pub memberReference: *mut crate::System::Runtime::Serialization::Formatters::Binary::MemberReference,
    pub objectNull: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectNull,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::__BinaryParser =>
    "System.Runtime.Serialization.Formatters.Binary"."__BinaryParser"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryParser")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryParser")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryParser")]
impl crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser {
    pub fn ReadBytes_i32_0(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadBytes", (length))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBytes_Il2CppArray_i32_i32_1(
        &mut self,
        byteA: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadBytes", (byteA, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn GetOp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectProgress,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectProgress = __cordl_object
            .invoke("GetOp", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadMemberPrimitiveTyped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadMemberPrimitiveTyped", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadMemberPrimitiveUnTyped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadMemberPrimitiveUnTyped", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadMemberReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadMemberReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadObjectString(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadObjectString", (binaryHeaderEnum))?;
        Ok(__cordl_ret)
    }
    pub fn ReadUInt32(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("ReadUInt32", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadUInt64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("ReadUInt64", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadChars(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("ReadChars", (length))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (binaryHeaderEnum))?;
        Ok(__cordl_ret)
    }
    pub fn get_SystemAssemblyInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo = __cordl_object
            .invoke("get_SystemAssemblyInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssemIdToAssemblyTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::SizedArray,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::SizedArray = __cordl_object
            .invoke("get_AssemIdToAssemblyTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ReadChar", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadObjectWithMapTyped_BinaryHeaderEnum0(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadObjectWithMapTyped", (binaryHeaderEnum))?;
        Ok(__cordl_ret)
    }
    pub fn ReadObjectWithMapTyped_BinaryObjectWithMapTyped1(
        &mut self,
        record: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadObjectWithMapTyped", (record))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArrayAsBytes(
        &mut self,
        pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArrayAsBytes", (pr))?;
        Ok(__cordl_ret)
    }
    pub fn ReadString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadTimeSpan(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TimeSpan = __cordl_object
            .invoke("ReadTimeSpan", ())?;
        Ok(__cordl_ret)
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadInt16(&mut self) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("ReadInt16", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ReadDouble", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadObjectNull(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadObjectNull", (binaryHeaderEnum))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectMapIdTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::SizedArray,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::SizedArray = __cordl_object
            .invoke("get_ObjectMapIdTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadBegin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadBegin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_prs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord = __cordl_object
            .invoke("get_prs", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadInt64(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ReadInt64", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadSerializationHeaderRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadSerializationHeaderRecord", ())?;
        Ok(__cordl_ret)
    }
    pub fn PutOp(
        &mut self,
        op: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectProgress,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PutOp", (op))?;
        Ok(__cordl_ret)
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadUInt16(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("ReadUInt16", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadCrossAppDomainMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadCrossAppDomainMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadObjectWithMap_BinaryHeaderEnum0(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadObjectWithMap", (binaryHeaderEnum))?;
        Ok(__cordl_ret)
    }
    pub fn ReadObjectWithMap_BinaryObjectWithMap1(
        &mut self,
        record: *mut crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadObjectWithMap", (record))?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue(
        &mut self,
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadValue", (code))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("ReadDecimal", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        objectReader: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, objectReader))?;
        Ok(__cordl_ret)
    }
    pub fn ReadSingle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ReadSingle", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadMessageEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadMessageEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadBoolean(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadBoolean", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadSByte(&mut self) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i8 = __cordl_object.invoke("ReadSByte", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAssembly(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAssembly", (binaryHeaderEnum))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ReadDateTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadInt32(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadInt32", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        stream: *mut crate::System::IO::Stream,
        objectReader: *mut crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, objectReader))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+__BinaryParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
