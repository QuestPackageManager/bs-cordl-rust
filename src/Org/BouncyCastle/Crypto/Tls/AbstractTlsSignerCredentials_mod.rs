#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractTlsSignerCredentials {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsCredentials,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials =>
    "Org.BouncyCastle.Crypto.Tls"."AbstractTlsSignerCredentials"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsCredentials;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    pub fn GenerateCertificateSignature(
        &mut self,
        hash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateCertificateSignature", (hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignatureAndHashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        > = __cordl_object.invoke("get_SignatureAndHashAlgorithm", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSignerCredentials")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
