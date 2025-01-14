#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "BinaryConverter";
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
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        assemId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
                6usize,
            >("GetBinaryTypeInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBinaryTypeInfo", 6usize
                )
            });
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        _cordl_type,
                        objectInfo,
                        typeName,
                        objectWriter,
                        typeInformation,
                        assemId,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetParserBinaryTypeInfo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeInformation: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                ),
                crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
                2usize,
            >("GetParserBinaryTypeInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetParserBinaryTypeInfo", 2usize
                )
            });
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum = unsafe {
            method.invoke_unchecked((), (_cordl_type, typeInformation))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("ReadTypeInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadTypeInfo", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (binaryTypeEnum, input, assemId)) };
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
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
        isVariant: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("TypeFromInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TypeFromInfo", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WriteTypeInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteTypeInfo", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (binaryTypeEnum, typeInformation, assemId, sout))
        };
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
