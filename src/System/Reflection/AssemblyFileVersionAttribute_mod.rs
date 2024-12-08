#[cfg(feature = "System+Reflection+AssemblyFileVersionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyFileVersionAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Version_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Reflection+AssemblyFileVersionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::AssemblyFileVersionAttribute
    => "System.Reflection"."AssemblyFileVersionAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyFileVersionAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyFileVersionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyFileVersionAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyFileVersionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyFileVersionAttribute")]
impl crate::System::Reflection::AssemblyFileVersionAttribute {
    pub fn New(
        version: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (version))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        version: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (version))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+AssemblyFileVersionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyFileVersionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
