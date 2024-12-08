#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthorizationRule {
    __cordl_parent: crate::System::Object,
    pub identity: *mut crate::System::Security::Principal::IdentityReference,
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
    type Target = crate::System::Object;
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
    pub fn get_AccessMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AccessMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        identity: *mut crate::System::Security::Principal::IdentityReference,
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
        Ok(__cordl_ret)
    }
    pub fn New(
        identity: *mut crate::System::Security::Principal::IdentityReference,
        accessMask: i32,
        isInherited: bool,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (identity, accessMask, isInherited, inheritanceFlags, propagationFlags),
            )?;
        Ok(__cordl_object)
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
