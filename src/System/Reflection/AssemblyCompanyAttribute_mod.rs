#[cfg(feature = "System+Reflection+AssemblyCompanyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyCompanyAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Company_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Reflection+AssemblyCompanyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::AssemblyCompanyAttribute =>
    "System.Reflection"."AssemblyCompanyAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyCompanyAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyCompanyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyCompanyAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyCompanyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyCompanyAttribute")]
impl crate::System::Reflection::AssemblyCompanyAttribute {
    pub fn New(
        company: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (company))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        company: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (company))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+AssemblyCompanyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyCompanyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}