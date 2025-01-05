#[cfg(feature = "System+Net+Mime+MailBnfHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MailBnfHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mime+MailBnfHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mime::MailBnfHelper =>
    "System.Net.Mime"."MailBnfHelper"
);
#[cfg(feature = "System+Net+Mime+MailBnfHelper")]
impl std::ops::Deref for crate::System::Net::Mime::MailBnfHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mime+MailBnfHelper")]
impl std::ops::DerefMut for crate::System::Net::Mime::MailBnfHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mime+MailBnfHelper")]
impl crate::System::Net::Mime::MailBnfHelper {
    pub fn CreateCharactersAllowedInAtoms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCharactersAllowedInAtoms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCharactersAllowedInComments() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCharactersAllowedInComments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCharactersAllowedInDomainLiterals() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCharactersAllowedInDomainLiterals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCharactersAllowedInHeaderNames() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCharactersAllowedInHeaderNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCharactersAllowedInQuotedStrings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCharactersAllowedInQuotedStrings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCharactersAllowedInTokens() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCharactersAllowedInTokens", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAllowedWhiteSpace(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAllowedWhiteSpace", (c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Mime+MailBnfHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mime::MailBnfHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
