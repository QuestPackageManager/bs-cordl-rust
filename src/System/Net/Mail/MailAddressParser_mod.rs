#[cfg(feature = "System+Net+Mail+MailAddressParser")]
#[repr(C)]
#[derive(Debug)]
pub struct MailAddressParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Mail::MailAddressParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Mail";
    const CLASS_NAME: &'static str = "MailAddressParser";
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
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl std::ops::Deref for crate::System::Net::Mail::MailAddressParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl std::ops::DerefMut for crate::System::Net::Mail::MailAddressParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl crate::System::Net::Mail::MailAddressParser {
    pub fn NormalizeOrThrow(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NormalizeOrThrow", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseAddress_Il2CppString0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Mail::MailAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Mail::MailAddress,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseAddress", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseAddress__cordl_bool_ByRefMut1(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expectMultipleAddresses: bool,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Mail::MailAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Mail::MailAddress,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseAddress", (data, expectMultipleAddresses, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDisplayName(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        expectMultipleAddresses: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDisplayName", (data, index, expectMultipleAddresses))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDomain(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDomain", (data, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseLocalPart(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        expectAngleBracket: bool,
        expectMultipleAddresses: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseLocalPart",
                (data, index, expectAngleBracket, expectMultipleAddresses),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCfwsAndThrowIfIncomplete(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadCfwsAndThrowIfIncomplete", (data, index))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mail::MailAddressParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
