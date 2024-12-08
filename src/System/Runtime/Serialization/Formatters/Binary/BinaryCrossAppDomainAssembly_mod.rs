#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryCrossAppDomainAssembly"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryCrossAppDomainAssembly {
    __cordl_parent: crate::System::Object,
    pub assemId: i32,
    pub assemblyIndex: i32,
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryCrossAppDomainAssembly"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryCrossAppDomainAssembly =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryCrossAppDomainAssembly"
);
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryCrossAppDomainAssembly"
)]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryCrossAppDomainAssembly {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryCrossAppDomainAssembly"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryCrossAppDomainAssembly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryCrossAppDomainAssembly"
)]
impl crate::System::Runtime::Serialization::Formatters::Binary::BinaryCrossAppDomainAssembly {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Runtime+Serialization+Formatters+Binary+BinaryCrossAppDomainAssembly"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryCrossAppDomainAssembly {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
