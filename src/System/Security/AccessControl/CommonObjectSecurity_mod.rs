#[cfg(feature = "System+Security+AccessControl+CommonObjectSecurity")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonObjectSecurity {
    __cordl_parent: crate::System::Security::AccessControl::ObjectSecurity,
}
#[cfg(feature = "System+Security+AccessControl+CommonObjectSecurity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::CommonObjectSecurity =>
    "System.Security.AccessControl"."CommonObjectSecurity"
);
#[cfg(feature = "System+Security+AccessControl+CommonObjectSecurity")]
impl std::ops::Deref for crate::System::Security::AccessControl::CommonObjectSecurity {
    type Target = crate::System::Security::AccessControl::ObjectSecurity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonObjectSecurity")]
impl std::ops::DerefMut
for crate::System::Security::AccessControl::CommonObjectSecurity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonObjectSecurity")]
impl crate::System::Security::AccessControl::CommonObjectSecurity {
    pub fn _ctor(
        &mut self,
        isContainer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer))?;
        Ok(__cordl_ret)
    }
    pub fn GetAccessRules(
        &mut self,
        includeExplicit: bool,
        includeInherited: bool,
        targetType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::AccessControl::AuthorizationRuleCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::AccessControl::AuthorizationRuleCollection = __cordl_object
            .invoke("GetAccessRules", (includeExplicit, includeInherited, targetType))?;
        Ok(__cordl_ret)
    }
    pub fn New(isContainer: bool) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonObjectSecurity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::CommonObjectSecurity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
