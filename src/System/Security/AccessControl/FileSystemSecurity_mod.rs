#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemSecurity {
    __cordl_parent: crate::System::Security::AccessControl::NativeObjectSecurity,
}
#[cfg(feature = "System+Security+AccessControl+FileSystemSecurity")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::FileSystemSecurity {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "FileSystemSecurity";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        identityReference: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::IdentityReference,
        >,
        accessMask: i32,
        isInherited: bool,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        _cordl_type: crate::System::Security::AccessControl::AccessControlType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::AccessControl::AccessRule>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::AccessRule,
        > = __cordl_object
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        isContainer: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer, name, includeSections))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        isContainer: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer, name, includeSections))?;
        Ok(__cordl_ret.into())
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
