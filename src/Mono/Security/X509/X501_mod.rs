#[cfg(feature = "Mono+Security+X509+X501")]
#[repr(C)]
#[derive(Debug)]
pub struct X501 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+X509+X501")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Security::X509::X501 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Security.X509";
    const CLASS_NAME: &'static str = "X501";
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
#[cfg(feature = "Mono+Security+X509+X501")]
impl std::ops::Deref for crate::Mono::Security::X509::X501 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X501")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X501 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X501")]
impl crate::Mono::Security::X509::X501 {
    pub fn AppendEntry(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        entry: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
        quotes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendEntry", (sb, entry, quotes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_ASN1_0(
        seq: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_bool_Il2CppString__cordl_bool1(
        seq: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
        reversed: bool,
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quotes: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (seq, reversed, separator, quotes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+X509+X501")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::X501 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
