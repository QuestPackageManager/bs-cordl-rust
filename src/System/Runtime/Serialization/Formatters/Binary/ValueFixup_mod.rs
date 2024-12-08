#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixup")]
#[repr(C)]
#[derive(Debug)]
pub struct ValueFixup {
    __cordl_parent: crate::System::Object,
    pub valueFixupEnum: crate::System::Runtime::Serialization::Formatters::Binary::ValueFixupEnum,
    pub arrayObj: *mut crate::System::Array,
    pub indexMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub header: *mut crate::System::Object,
    pub memberObject: *mut crate::System::Object,
    pub objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    pub memberName: *mut crate::System::String,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ValueFixup =>
    "System.Runtime.Serialization.Formatters.Binary"."ValueFixup"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixup")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ValueFixup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixup")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ValueFixup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixup")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ValueFixup {
    pub fn Fixup(
        &mut self,
        record: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        parent: *mut crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fixup", (record, parent))?;
        Ok(__cordl_ret)
    }
    pub fn New_Array_Il2CppArray0(
        arrayObj: *mut crate::System::Array,
        indexMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (arrayObj, indexMap))?;
        Ok(__cordl_object)
    }
    pub fn New_Object_String_ReadObjectInfo1(
        memberObject: *mut crate::System::Object,
        memberName: *mut crate::System::String,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (memberObject, memberName, objectInfo))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Array_Il2CppArray0(
        &mut self,
        arrayObj: *mut crate::System::Array,
        indexMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (arrayObj, indexMap))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_String_ReadObjectInfo1(
        &mut self,
        memberObject: *mut crate::System::Object,
        memberName: *mut crate::System::String,
        objectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (memberObject, memberName, objectInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixup")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ValueFixup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
