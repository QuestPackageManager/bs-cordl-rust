#[cfg(feature = "System+Reflection+AssemblyInformationalVersionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyInformationalVersionAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _InformationalVersion_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Reflection+AssemblyInformationalVersionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::AssemblyInformationalVersionAttribute => "System.Reflection"
    ."AssemblyInformationalVersionAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyInformationalVersionAttribute")]
impl std::ops::Deref
for crate::System::Reflection::AssemblyInformationalVersionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyInformationalVersionAttribute")]
impl std::ops::DerefMut
for crate::System::Reflection::AssemblyInformationalVersionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyInformationalVersionAttribute")]
impl crate::System::Reflection::AssemblyInformationalVersionAttribute {
    pub fn New(
        informationalVersion: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (informationalVersion))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        informationalVersion: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (informationalVersion))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+AssemblyInformationalVersionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyInformationalVersionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}