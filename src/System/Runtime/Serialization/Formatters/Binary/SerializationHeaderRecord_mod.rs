#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+SerializationHeaderRecord"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationHeaderRecord {
    __cordl_parent: crate::System::Object,
    pub binaryFormatterMajorVersion: i32,
    pub binaryFormatterMinorVersion: i32,
    pub binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    pub topId: i32,
    pub headerId: i32,
    pub majorVersion: i32,
    pub minorVersion: i32,
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+SerializationHeaderRecord"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::SerializationHeaderRecord =>
    "System.Runtime.Serialization.Formatters.Binary"."SerializationHeaderRecord"
);
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+SerializationHeaderRecord"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::SerializationHeaderRecord {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+SerializationHeaderRecord"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::SerializationHeaderRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+SerializationHeaderRecord"
)]
impl crate::System::Runtime::Serialization::Formatters::Binary::SerializationHeaderRecord {
    pub fn Dump(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dump", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_BinaryHeaderEnum_i32_i32_i32_i32_1(
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
        topId: i32,
        headerId: i32,
        majorVersion: i32,
        minorVersion: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (binaryHeaderEnum, topId, headerId, majorVersion, minorVersion),
            )?;
        Ok(__cordl_object)
    }
    pub fn Read(
        &mut self,
        input: *mut crate::System::Runtime::Serialization::Formatters::Binary::__BinaryParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (input))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        sout: *mut crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (sout))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BinaryHeaderEnum_i32_i32_i32_i32_1(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
        topId: i32,
        headerId: i32,
        majorVersion: i32,
        minorVersion: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (binaryHeaderEnum, topId, headerId, majorVersion, minorVersion),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+SerializationHeaderRecord"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::SerializationHeaderRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}