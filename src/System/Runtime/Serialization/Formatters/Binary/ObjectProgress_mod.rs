#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectProgress")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectProgress {
    __cordl_parent: crate::System::Object,
    pub isInitial: bool,
    pub count: i32,
    pub expectedType: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    pub expectedTypeInformation: *mut crate::System::Object,
    pub name: *mut crate::System::String,
    pub objectTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalObjectTypeE,
    pub memberTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalMemberTypeE,
    pub memberValueEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalMemberValueE,
    pub dtType: *mut crate::System::Type,
    pub numItems: i32,
    pub binaryTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    pub typeInformation: *mut crate::System::Object,
    pub nullCount: i32,
    pub memberLength: i32,
    pub binaryTypeEnumA: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
    >,
    pub typeInformationA: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Object,
    >,
    pub memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    pub pr: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectProgress")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ObjectProgress =>
    "System.Runtime.Serialization.Formatters.Binary"."ObjectProgress"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectProgress")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectProgress {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectProgress")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectProgress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectProgress")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectProgress {
    pub fn ArrayCountIncrement(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ArrayCountIncrement", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetNext(
        &mut self,
        outBinaryTypeEnum: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
        >,
        outTypeInformation: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetNext", (outBinaryTypeEnum, outTypeInformation))?;
        Ok(__cordl_ret)
    }
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
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectProgress")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectProgress {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
