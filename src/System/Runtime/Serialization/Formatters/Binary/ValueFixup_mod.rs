#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ValueFixup")]
#[repr(C)]
#[derive(Debug)]
pub struct ValueFixup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub valueFixupEnum: crate::System::Runtime::Serialization::Formatters::Binary::ValueFixupEnum,
    pub arrayObj: quest_hook::libil2cpp::Gc<crate::System::Array>,
    pub indexMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub memberObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub objectInfo: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    >,
    pub memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        record: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        >,
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fixup", (record, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Array_Il2CppArray0(
        arrayObj: quest_hook::libil2cpp::Gc<crate::System::Array>,
        indexMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (arrayObj, indexMap))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_Il2CppString_ReadObjectInfo1(
        memberObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (memberObject, memberName, objectInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Array_Il2CppArray0(
        &mut self,
        arrayObj: quest_hook::libil2cpp::Gc<crate::System::Array>,
        indexMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (arrayObj, indexMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_Il2CppString_ReadObjectInfo1(
        &mut self,
        memberObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (memberObject, memberName, objectInfo))?;
        Ok(__cordl_ret.into())
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
