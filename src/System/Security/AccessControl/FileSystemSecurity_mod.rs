#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemSecurity {
    __cordl_parent: crate::System::Security::AccessControl::NativeObjectSecurity,
}
#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::FileSystemSecurity =>
    "System.Security.AccessControl"."FileSystemSecurity"
);
#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
impl std::ops::Deref for crate::System::Security::AccessControl::FileSystemSecurity {
    type Target = crate::System::Security::AccessControl::NativeObjectSecurity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::FileSystemSecurity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
impl crate::System::Security::AccessControl::FileSystemSecurity {
    pub fn AccessRuleFactory(
        &mut self,
        identityReference: *mut crate::System::Security::Principal::IdentityReference,
        accessMask: i32,
        isInherited: bool,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        _cordl_type: crate::System::Security::AccessControl::AccessControlType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::AccessControl::AccessRule,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::AccessControl::AccessRule = __cordl_object
            .invoke(
                "AccessRuleFactory",
                (
                    identityReference,
                    accessMask,
                    isInherited,
                    inheritanceFlags,
                    propagationFlags,
                    _cordl_type,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        isContainer: bool,
        name: *mut crate::System::String,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer, name, includeSections))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        isContainer: bool,
        name: *mut crate::System::String,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer, name, includeSections))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::FileSystemSecurity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
