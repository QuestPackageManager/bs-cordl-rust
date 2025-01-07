#[cfg(feature = "System+Net+CredentialCache")]
#[repr(C)]
#[derive(Debug)]
pub struct CredentialCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+CredentialCache")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::CredentialCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CredentialCache";
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
#[cfg(feature = "System+Net+CredentialCache")]
impl std::ops::Deref for crate::System::Net::CredentialCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CredentialCache")]
impl std::ops::DerefMut for crate::System::Net::CredentialCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CredentialCache")]
impl crate::System::Net::CredentialCache {
    pub fn get_DefaultCredentials() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultCredentials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultNetworkCredentials() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::NetworkCredential>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkCredential,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultNetworkCredentials", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+CredentialCache")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::CredentialCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
