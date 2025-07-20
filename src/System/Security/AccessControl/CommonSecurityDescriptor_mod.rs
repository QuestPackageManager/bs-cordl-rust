#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonSecurityDescriptor {
    __cordl_parent: crate::System::Security::AccessControl::GenericSecurityDescriptor,
    pub is_container: bool,
    pub is_ds: bool,
    pub flags: crate::System::Security::AccessControl::ControlFlags,
    pub owner: quest_hook::libil2cpp::Gc<
        crate::System::Security::Principal::SecurityIdentifier,
    >,
    pub group: quest_hook::libil2cpp::Gc<
        crate::System::Security::Principal::SecurityIdentifier,
    >,
    pub system_acl: quest_hook::libil2cpp::Gc<
        crate::System::Security::AccessControl::SystemAcl,
    >,
    pub discretionary_acl: quest_hook::libil2cpp::Gc<
        crate::System::Security::AccessControl::DiscretionaryAcl,
    >,
}
#[cfg(feature = "System+Security+AccessControl+CommonSecurityDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::CommonSecurityDescriptor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "CommonSecurityDescriptor";
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
        acl: quest_hook::libil2cpp::Gc<crate::System::Security::AccessControl::CommonAcl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Security::AccessControl::CommonAcl,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckAclConsistency")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckAclConsistency", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (acl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        isContainer: bool,
        isDS: bool,
        flags: crate::System::Security::AccessControl::ControlFlags,
        owner: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        group: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        systemAcl: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::SystemAcl,
        >,
        discretionaryAcl: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DiscretionaryAcl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
                            bool,
                            crate::System::Security::AccessControl::ControlFlags,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Principal::SecurityIdentifier,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Principal::SecurityIdentifier,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::AccessControl::SystemAcl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::AccessControl::DiscretionaryAcl,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Init", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (isContainer, isDS, flags, owner, group, systemAcl, discretionaryAcl),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        isContainer: bool,
        isDS: bool,
        flags: crate::System::Security::AccessControl::ControlFlags,
        owner: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        group: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        systemAcl: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::SystemAcl,
        >,
        discretionaryAcl: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DiscretionaryAcl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (isContainer, isDS, flags, owner, group, systemAcl, discretionaryAcl),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        isContainer: bool,
        isDS: bool,
        flags: crate::System::Security::AccessControl::ControlFlags,
        owner: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        group: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
        systemAcl: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::SystemAcl,
        >,
        discretionaryAcl: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DiscretionaryAcl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
                            bool,
                            crate::System::Security::AccessControl::ControlFlags,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Principal::SecurityIdentifier,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Principal::SecurityIdentifier,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::AccessControl::SystemAcl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::AccessControl::DiscretionaryAcl,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (isContainer, isDS, flags, owner, group, systemAcl, discretionaryAcl),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DiscretionaryAcl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DiscretionaryAcl,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Security::AccessControl::DiscretionaryAcl,
                        >,
                        0usize,
                    >("get_DiscretionaryAcl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_DiscretionaryAcl", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DiscretionaryAcl,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsContainer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("get_IsContainer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_IsContainer", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDS(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("get_IsDS")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_IsDS", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_DiscretionaryAcl(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DiscretionaryAcl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Security::AccessControl::DiscretionaryAcl,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_DiscretionaryAcl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_DiscretionaryAcl", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Group(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Security::Principal::SecurityIdentifier,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Group")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_Group", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Owner(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::SecurityIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Security::Principal::SecurityIdentifier,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Owner")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_Owner", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_SystemAcl(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::SystemAcl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Security::AccessControl::SystemAcl,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_SystemAcl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_SystemAcl", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
