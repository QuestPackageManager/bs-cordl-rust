#[cfg(feature = "Internal+Cryptography+OidLookup")]
#[repr(C)]
#[derive(Debug)]
pub struct OidLookup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
unsafe impl quest_hook::libil2cpp::Type for crate::Internal::Cryptography::OidLookup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Internal.Cryptography";
    const CLASS_NAME: &'static str = "OidLookup";
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
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl std::ops::Deref for crate::Internal::Cryptography::OidLookup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl std::ops::DerefMut for crate::Internal::Cryptography::OidLookup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl crate::Internal::Cryptography::OidLookup {
    pub fn NativeFriendlyNameToOid(
        friendlyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oidGroup: crate::System::Security::Cryptography::OidGroup,
        fallBackToAllGroups: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "NativeFriendlyNameToOid",
                (friendlyName, oidGroup, fallBackToAllGroups),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn NativeOidToFriendlyName(
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oidGroup: crate::System::Security::Cryptography::OidGroup,
        fallBackToAllGroups: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NativeOidToFriendlyName", (oid, oidGroup, fallBackToAllGroups))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldUseCache(
        oidGroup: crate::System::Security::Cryptography::OidGroup,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldUseCache", (oidGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFriendlyName(
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oidGroup: crate::System::Security::Cryptography::OidGroup,
        fallBackToAllGroups: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFriendlyName", (oid, oidGroup, fallBackToAllGroups))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToOid(
        friendlyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oidGroup: crate::System::Security::Cryptography::OidGroup,
        fallBackToAllGroups: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToOid", (friendlyName, oidGroup, fallBackToAllGroups))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Cryptography+OidLookup")]
impl quest_hook::libil2cpp::ObjectType for crate::Internal::Cryptography::OidLookup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
