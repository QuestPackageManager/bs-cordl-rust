#[cfg(feature = "System+Security+AccessControl+ObjectSecurity")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectSecurity {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub descriptor: quest_hook::libil2cpp::Gc<
        crate::System::Security::AccessControl::CommonSecurityDescriptor,
    >,
    pub sections_modified: crate::System::Security::AccessControl::AccessControlSections,
    pub rw_lock: quest_hook::libil2cpp::Gc<crate::System::Threading::ReaderWriterLock>,
}
#[cfg(feature = "System+Security+AccessControl+ObjectSecurity")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::ObjectSecurity {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "ObjectSecurity";
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
#[cfg(feature = "System+Security+AccessControl+ObjectSecurity")]
impl std::ops::Deref for crate::System::Security::AccessControl::ObjectSecurity {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+ObjectSecurity")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::ObjectSecurity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+ObjectSecurity")]
impl crate::System::Security::AccessControl::ObjectSecurity {
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
    pub fn InternalAccessRuleFactory(
        &mut self,
        ace: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::QualifiedAce,
        >,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
            .invoke("InternalAccessRuleFactory", (ace, targetType, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetAccessRules(
        &mut self,
        includeExplicit: bool,
        includeInherited: bool,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::AuthorizationRuleCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::AuthorizationRuleCollection,
        > = __cordl_object
            .invoke(
                "InternalGetAccessRules",
                (includeExplicit, includeInherited, targetType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_CommonSecurityDescriptor0(
        securityDescriptor: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::CommonSecurityDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (securityDescriptor))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool__cordl_bool1(
        isContainer: bool,
        isDS: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer, isDS))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnlock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadUnlock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUnlock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteUnlock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Writing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Writing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CommonSecurityDescriptor0(
        &mut self,
        securityDescriptor: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::CommonSecurityDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (securityDescriptor))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool1(
        &mut self,
        isContainer: bool,
        isDS: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer, isDS))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AccessControlSectionsModified(
        &mut self,
        value: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AccessControlSectionsModified", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+ObjectSecurity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::ObjectSecurity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
