#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Helper2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X509Helper2";
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
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    pub fn CreateChainImpl(
        useMachineContext: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainImpl,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainImpl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateChainImpl", (useMachineContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvalidChainContextException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvalidChainContextException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMonoCertificate(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Certificate,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMonoCertificate", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(
        _cordl_impl: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (_cordl_impl))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfContextInvalid(
        _cordl_impl: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowIfContextInvalid", (_cordl_impl))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
