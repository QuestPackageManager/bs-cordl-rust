#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Helper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::X509Helper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X509Helper";
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
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Helper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Helper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl crate::System::Security::Cryptography::X509Certificates::X509Helper {
    pub fn GetInvalidContextException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvalidContextException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Import_Il2CppArray0(
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Import", (rawData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Import_SafePasswordHandle_X509KeyStorageFlags1(
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafePasswordHandle,
        >,
        keyStorageFlags: crate::System::Security::Cryptography::X509Certificates::X509KeyStorageFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Import", (rawData, password, keyStorageFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitFromCertificate_X509Certificate0(
        cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitFromCertificate", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitFromCertificate_X509CertificateImpl1(
        _cordl_impl: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitFromCertificate", (_cordl_impl))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(
        _cordl_impl: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (_cordl_impl))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfContextInvalid(
        _cordl_impl: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowIfContextInvalid", (_cordl_impl))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateProvider() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::ISystemCertificateProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::ISystemCertificateProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CertificateProvider", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Helper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
