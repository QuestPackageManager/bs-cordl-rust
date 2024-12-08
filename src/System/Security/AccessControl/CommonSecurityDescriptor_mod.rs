#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonSecurityDescriptor {
    __cordl_parent: crate::System::Security::AccessControl::GenericSecurityDescriptor,
    pub is_container: bool,
    pub is_ds: bool,
    pub flags: crate::System::Security::AccessControl::ControlFlags,
    pub owner: *mut crate::System::Security::Principal::SecurityIdentifier,
    pub group: *mut crate::System::Security::Principal::SecurityIdentifier,
    pub system_acl: *mut crate::System::Security::AccessControl::SystemAcl,
    pub discretionary_acl: *mut crate::System::Security::AccessControl::DiscretionaryAcl,
}
#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::CommonSecurityDescriptor =>
    "System.Security.AccessControl"."CommonSecurityDescriptor"
);
#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
impl std::ops::Deref
for crate::System::Security::AccessControl::CommonSecurityDescriptor {
    type Target = crate::System::Security::AccessControl::GenericSecurityDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
impl std::ops::DerefMut
for crate::System::Security::AccessControl::CommonSecurityDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
impl crate::System::Security::AccessControl::CommonSecurityDescriptor {
    pub fn CheckAclConsistency(
        &mut self,
        acl: *mut crate::System::Security::AccessControl::CommonAcl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckAclConsistency", (acl))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        isContainer: bool,
        isDS: bool,
        flags: crate::System::Security::AccessControl::ControlFlags,
        owner: *mut crate::System::Security::Principal::SecurityIdentifier,
        group: *mut crate::System::Security::Principal::SecurityIdentifier,
        systemAcl: *mut crate::System::Security::AccessControl::SystemAcl,
        discretionaryAcl: *mut crate::System::Security::AccessControl::DiscretionaryAcl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (isContainer, isDS, flags, owner, group, systemAcl, discretionaryAcl),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        isContainer: bool,
        isDS: bool,
        flags: crate::System::Security::AccessControl::ControlFlags,
        owner: *mut crate::System::Security::Principal::SecurityIdentifier,
        group: *mut crate::System::Security::Principal::SecurityIdentifier,
        systemAcl: *mut crate::System::Security::AccessControl::SystemAcl,
        discretionaryAcl: *mut crate::System::Security::AccessControl::DiscretionaryAcl,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (isContainer, isDS, flags, owner, group, systemAcl, discretionaryAcl),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        isContainer: bool,
        isDS: bool,
        flags: crate::System::Security::AccessControl::ControlFlags,
        owner: *mut crate::System::Security::Principal::SecurityIdentifier,
        group: *mut crate::System::Security::Principal::SecurityIdentifier,
        systemAcl: *mut crate::System::Security::AccessControl::SystemAcl,
        discretionaryAcl: *mut crate::System::Security::AccessControl::DiscretionaryAcl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (isContainer, isDS, flags, owner, group, systemAcl, discretionaryAcl),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_DiscretionaryAcl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::AccessControl::DiscretionaryAcl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::AccessControl::DiscretionaryAcl = __cordl_object
            .invoke("get_DiscretionaryAcl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsContainer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDS(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDS", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DiscretionaryAcl(
        &mut self,
        value: *mut crate::System::Security::AccessControl::DiscretionaryAcl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DiscretionaryAcl", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Group(
        &mut self,
        value: *mut crate::System::Security::Principal::SecurityIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Group", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Owner(
        &mut self,
        value: *mut crate::System::Security::Principal::SecurityIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Owner", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SystemAcl(
        &mut self,
        value: *mut crate::System::Security::AccessControl::SystemAcl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SystemAcl", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::CommonSecurityDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
