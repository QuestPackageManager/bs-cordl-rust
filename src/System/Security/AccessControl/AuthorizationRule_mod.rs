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
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::AuthorizationRule {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "AuthorizationRule";
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
#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
impl std::ops::Deref for crate::System::Security::AccessControl::AuthorizationRule {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AuthorizationRule")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::AuthorizationRule {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Principal::IdentityReference,
                            >,
                            i32,
                            bool,
                            crate::System::Security::AccessControl::InheritanceFlags,
                            crate::System::Security::AccessControl::PropagationFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        identity,
                        accessMask,
                        isInherited,
                        inheritanceFlags,
                        propagationFlags,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AccessMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_AccessMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AccessMask", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
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
