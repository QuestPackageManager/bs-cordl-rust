#[cfg(feature = "System+Security+AccessControl+FileSystemAccessRule")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemAccessRule {
    __cordl_parent: crate::System::Security::AccessControl::AccessRule,
}
#[cfg(feature = "System+Security+AccessControl+FileSystemAccessRule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::FileSystemAccessRule =>
    "System.Security.AccessControl"."FileSystemAccessRule"
);
#[cfg(feature = "System+Security+AccessControl+FileSystemAccessRule")]
impl std::ops::Deref for crate::System::Security::AccessControl::FileSystemAccessRule {
    type Target = crate::System::Security::AccessControl::AccessRule;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+FileSystemAccessRule")]
impl std::ops::DerefMut
for crate::System::Security::AccessControl::FileSystemAccessRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+FileSystemAccessRule")]
impl crate::System::Security::AccessControl::FileSystemAccessRule {
    pub fn New(
        identity: *mut crate::System::Security::Principal::IdentityReference,
        fileSystemRights: crate::System::Security::AccessControl::FileSystemRights,
        isInherited: bool,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        _cordl_type: crate::System::Security::AccessControl::AccessControlType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    identity,
                    fileSystemRights,
                    isInherited,
                    inheritanceFlags,
                    propagationFlags,
                    _cordl_type,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        identity: *mut crate::System::Security::Principal::IdentityReference,
        fileSystemRights: crate::System::Security::AccessControl::FileSystemRights,
        isInherited: bool,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        _cordl_type: crate::System::Security::AccessControl::AccessControlType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    identity,
                    fileSystemRights,
                    isInherited,
                    inheritanceFlags,
                    propagationFlags,
                    _cordl_type,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_FileSystemRights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::FileSystemRights,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::FileSystemRights = __cordl_object
            .invoke("get_FileSystemRights", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+FileSystemAccessRule")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::FileSystemAccessRule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
