#[cfg(feature = "System+Security+AccessControl+AccessRule")]
#[repr(C)]
#[derive(Debug)]
pub struct AccessRule {
    __cordl_parent: crate::System::Security::AccessControl::AuthorizationRule,
    pub _cordl_type: crate::System::Security::AccessControl::AccessControlType,
}
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::AccessRule {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "AccessRule";
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
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
impl std::ops::Deref for crate::System::Security::AccessControl::AccessRule {
    type Target = crate::System::Security::AccessControl::AuthorizationRule;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+AccessRule")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::AccessRule {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
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
                            crate::System::Security::AccessControl::AccessControlType,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        identity,
                        accessMask,
                        isInherited,
                        inheritanceFlags,
                        propagationFlags,
                        _cordl_type,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AccessControlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::AccessControlType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Security::AccessControl::AccessControlType,
                        0usize,
                    >("get_AccessControlType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AccessControlType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Security::AccessControl::AccessControlType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
