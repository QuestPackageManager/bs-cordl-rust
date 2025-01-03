#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryConverter =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryConverter"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    pub fn GetBinaryTypeInfo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter,
        >,
        typeInformation: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        assemId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    > {
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBinaryTypeInfo",
                (
                    _cordl_type,
                    objectInfo,
                    typeName,
                    objectWriter,
                    typeInformation,
                    assemId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParserBinaryTypeInfo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeInformation: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    > {
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParserBinaryTypeInfo", (_cordl_type, typeInformation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadTypeInfo(
        binaryTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
        input: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser,
        >,
        assemId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadTypeInfo", (binaryTypeEnum, input, assemId))?;
        Ok(__cordl_ret.into())
    }
    pub fn TypeFromInfo(
        binaryTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
        typeInformation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objectReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
        >,
        assemblyInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        >,
        primitiveTypeEnum: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        >,
        typeString: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
        isVariant: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TypeFromInfo",
                (
                    binaryTypeEnum,
                    typeInformation,
                    objectReader,
                    assemblyInfo,
                    primitiveTypeEnum,
                    typeString,
                    _cordl_type,
                    isVariant,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTypeInfo(
        binaryTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
        typeInformation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        assemId: i32,
        sout: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteTypeInfo", (binaryTypeEnum, typeInformation, assemId, sout))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
