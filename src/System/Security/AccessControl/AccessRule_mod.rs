#[cfg(feature = "System+Security+AccessControl+AccessRule")]
#[repr(C)]
#[derive(Debug)]
pub struct AccessRule {
    __cordl_parent: crate::System::Security::AccessControl::AuthorizationRule,
    pub _cordl_type: crate::System::Security::AccessControl::AccessControlType,
}
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::AccessRule =>
    "System.Security.AccessControl"."AccessRule"
);
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
impl std::ops::Deref for crate::System::Security::AccessControl::AccessRule {
    type Target = crate::System::Security::AccessControl::AuthorizationRule;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::AccessRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
impl crate::System::Security::AccessControl::AccessRule {
    pub fn New(
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::IdentityReference,
        >,
        accessMask: i32,
        isInherited: bool,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        _cordl_type: crate::System::Security::AccessControl::AccessControlType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    identity,
                    accessMask,
                    isInherited,
                    inheritanceFlags,
                    propagationFlags,
                    _cordl_type,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::IdentityReference,
        >,
        accessMask: i32,
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
                    accessMask,
                    isInherited,
                    inheritanceFlags,
                    propagationFlags,
                    _cordl_type,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AccessControlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::AccessControlType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::AccessControlType = __cordl_object
            .invoke("get_AccessControlType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::AccessRule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
