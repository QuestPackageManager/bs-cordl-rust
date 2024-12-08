#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberPrimitiveUnTyped {
    __cordl_parent: crate::System::Object,
    pub typeInformation: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub value: *mut crate::System::Object,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped =>
    "System.Runtime.Serialization.Formatters.Binary"."MemberPrimitiveUnTyped"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
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
    pub fn Set_Object0(
        &mut self,
        typeInformation: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (typeInformation, value))?;
        Ok(__cordl_ret)
    }
    pub fn Set_InternalPrimitiveTypeE1(
        &mut self,
        typeInformation: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (typeInformation))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MemberPrimitiveUnTyped")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::MemberPrimitiveUnTyped {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
