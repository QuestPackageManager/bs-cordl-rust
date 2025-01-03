#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthorizationRule {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub identity: quest_hook::libil2cpp::Gc<
        crate::System::Security::Principal::IdentityReference,
    >,
    pub accessMask: i32,
    pub isInherited: bool,
    pub inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
    pub propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::AuthorizationRule =>
    "System.Security.AccessControl"."AuthorizationRule"
);
#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
impl std::ops::Deref for crate::System::Security::AccessControl::AuthorizationRule {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::AuthorizationRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
impl crate::System::Security::AccessControl::AuthorizationRule {
    pub fn New(
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::IdentityReference,
        >,
        accessMask: i32,
        isInherited: bool,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (identity, accessMask, isInherited, inheritanceFlags, propagationFlags),
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (identity, accessMask, isInherited, inheritanceFlags, propagationFlags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AccessMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AccessMask", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::AuthorizationRule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
