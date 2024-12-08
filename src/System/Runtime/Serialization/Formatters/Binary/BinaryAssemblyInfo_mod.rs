#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryAssemblyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryAssemblyInfo {
    __cordl_parent: crate::System::Object,
    pub assemblyString: *mut crate::System::String,
    pub assembly: *mut crate::System::Reflection::Assembly,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryAssemblyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryAssemblyInfo"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryAssemblyInfo")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryAssemblyInfo")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryAssemblyInfo")]
impl crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo {
    pub fn _ctor_String0(
        &mut self,
        assemblyString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (assemblyString))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Assembly1(
        &mut self,
        assemblyString: *mut crate::System::String,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (assemblyString, assembly))?;
        Ok(__cordl_ret)
    }
    pub fn GetAssembly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke("GetAssembly", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        assemblyString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assemblyString))?;
        Ok(__cordl_object)
    }
    pub fn New_Assembly1(
        assemblyString: *mut crate::System::String,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assemblyString, assembly))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryAssemblyInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
