#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryMethodReturn")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryMethodReturn {
    __cordl_parent: crate::System::Object,
    pub returnValue: *mut crate::System::Object,
    pub args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub callContext: *mut crate::System::Object,
    pub argTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    pub bArgsPrimitive: bool,
    pub messageEnum: crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum,
    pub returnType: *mut crate::System::Type,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryMethodReturn")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryMethodReturn =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryMethodReturn"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryMethodReturn")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryMethodReturn {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryMethodReturn")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryMethodReturn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryMethodReturn")]
impl crate::System::Runtime::Serialization::Formatters::Binary::BinaryMethodReturn {
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryMethodReturn")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryMethodReturn {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}