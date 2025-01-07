#[cfg(feature = "System+DomainNameHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct DomainNameHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+DomainNameHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DomainNameHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DomainNameHelper";
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
#[cfg(feature = "System+DomainNameHelper")]
impl std::ops::Deref for crate::System::DomainNameHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DomainNameHelper")]
impl std::ops::DerefMut for crate::System::DomainNameHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DomainNameHelper")]
impl crate::System::DomainNameHelper {
    pub fn IdnEquivalent_Il2CppObject_i32_i32_ByRefMut_ByRefMut0(
        hostname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
        allAscii: quest_hook::libil2cpp::ByRefMut<bool>,
        atLeastOneValidIdn: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IdnEquivalent",
                (hostname, start, end, allAscii, atLeastOneValidIdn),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IdnEquivalent_Il2CppObject_i32_i32_ByRefMut_ByRefMut1(
        hostname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
        allAscii: quest_hook::libil2cpp::ByRefMut<bool>,
        bidiStrippedHost: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IdnEquivalent",
                (hostname, start, end, allAscii, bidiStrippedHost),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsASCIILetterOrDigit(
        character: char,
        notCanonical: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsASCIILetterOrDigit", (character, notCanonical))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIdnAce_Il2CppObject1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIdnAce", (input, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIdnAce_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIdnAce", (input, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pos: u16,
        returnedEnd: quest_hook::libil2cpp::ByRefMut<i32>,
        notCanonical: quest_hook::libil2cpp::ByRefMut<bool>,
        notImplicitFile: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (name, pos, returnedEnd, notCanonical, notImplicitFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidByIri(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pos: u16,
        returnedEnd: quest_hook::libil2cpp::ByRefMut<i32>,
        notCanonical: quest_hook::libil2cpp::ByRefMut<bool>,
        notImplicitFile: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IsValidByIri",
                (name, pos, returnedEnd, notCanonical, notImplicitFile),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidDomainLabelCharacter(
        character: char,
        notCanonical: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidDomainLabelCharacter", (character, notCanonical))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCanonicalName(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        loopback: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseCanonicalName", (str, start, end, loopback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnicodeEquivalent_Il2CppObject_i32_ByRefMut_ByRefMut1(
        hostname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
        allAscii: quest_hook::libil2cpp::ByRefMut<bool>,
        atLeastOneValidIdn: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UnicodeEquivalent",
                (hostname, start, end, allAscii, atLeastOneValidIdn),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnicodeEquivalent_Il2CppString_Il2CppObject_i32_0(
        idnHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hostname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnicodeEquivalent", (idnHost, hostname, start, end))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+DomainNameHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DomainNameHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
