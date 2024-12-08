#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExporterLabel")]
#[repr(C)]
#[derive(Debug)]
pub struct ExporterLabel {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExporterLabel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::ExporterLabel =>
    "Org.BouncyCastle.Crypto.Tls"."ExporterLabel"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExporterLabel")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::ExporterLabel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExporterLabel")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::ExporterLabel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExporterLabel")]
impl crate::Org::BouncyCastle::Crypto::Tls::ExporterLabel {
    pub const client_EAP_encryption: &'static str = "client EAP encryption";
    pub const client_finished: &'static str = "client finished";
    pub const dtls_srtp: &'static str = "EXTRACTOR-dtls_srtp";
    pub const key_expansion: &'static str = "key expansion";
    pub const master_secret: &'static str = "master secret";
    pub const server_finished: &'static str = "server finished";
    pub const ttls_challenge: &'static str = "ttls challenge";
    pub const ttls_keying_material: &'static str = "ttls keying material";
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExporterLabel")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::ExporterLabel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
