#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryObjectWithMapTyped"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryObjectWithMapTyped {
    __cordl_parent: crate::System::Object,
    pub binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    pub objectId: i32,
    pub name: *mut crate::System::String,
    pub numMembers: i32,
    pub memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub binaryTypeEnumA: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    >,
    pub typeInformationA: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Object,
    >,
    pub memberAssemIds: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub assemId: i32,
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryObjectWithMapTyped"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryObjectWithMapTyped"
);
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryObjectWithMapTyped"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryObjectWithMapTyped"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryObjectWithMapTyped"
)]
impl crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_BinaryHeaderEnum1(
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binaryHeaderEnum))?;
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
    pub fn Set(
        &mut self,
        objectId: i32,
        name: *mut crate::System::String,
        numMembers: i32,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        binaryTypeEnumA: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
        >,
        typeInformationA: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        >,
        memberAssemIds: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        assemId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Set",
                (
                    objectId,
                    name,
                    numMembers,
                    memberNames,
                    binaryTypeEnumA,
                    typeInformationA,
                    memberAssemIds,
                    assemId,
                ),
            )?;
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
    pub fn _ctor_BinaryHeaderEnum1(
        &mut self,
        binaryHeaderEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (binaryHeaderEnum))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryObjectWithMapTyped"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryObjectWithMapTyped {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}