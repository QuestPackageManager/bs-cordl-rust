#[cfg(feature = "System+Security+AccessControl+DirectorySecurity")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectorySecurity {
    __cordl_parent: crate::System::Security::AccessControl::FileSystemSecurity,
}
#[cfg(feature = "System+Security+AccessControl+DirectorySecurity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::DirectorySecurity =>
    "System.Security.AccessControl"."DirectorySecurity"
);
#[cfg(feature = "System+Security+AccessControl+DirectorySecurity")]
impl std::ops::Deref for crate::System::Security::AccessControl::DirectorySecurity {
    type Target = crate::System::Security::AccessControl::FileSystemSecurity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+DirectorySecurity")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::DirectorySecurity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+DirectorySecurity")]
impl crate::System::Security::AccessControl::DirectorySecurity {
    pub fn New(
        name: *mut quest_hook::libil2cpp::Il2CppString,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, includeSections))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, includeSections))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+DirectorySecurity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::DirectorySecurity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
