#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateUrl {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mType: u8,
    pub mUrlAndHashList: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::CertificateUrl
    => "Org.BouncyCastle.Crypto.Tls"."CertificateUrl"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
impl crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl {
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl+ListBuffer16")]
    pub type ListBuffer16 = crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl_ListBuffer16;
    pub fn Encode(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_type: u8,
        urlAndHashList: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, urlAndHashList))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: u8,
        urlAndHashList: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, urlAndHashList))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UrlAndHashList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = __cordl_object
            .invoke("get_UrlAndHashList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn parse(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("parse", (context, input))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl+ListBuffer16")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateUrl_ListBuffer16 {
    __cordl_parent: crate::System::IO::MemoryStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl+ListBuffer16")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::CertificateUrl_ListBuffer16 =>
    "Org.BouncyCastle.Crypto.Tls"."CertificateUrl/ListBuffer16"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl+ListBuffer16")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl_ListBuffer16 {
    type Target = crate::System::IO::MemoryStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl+ListBuffer16")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl_ListBuffer16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl+ListBuffer16")]
impl crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl_ListBuffer16 {
    pub fn EncodeTo(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncodeTo", (output))?;
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl+ListBuffer16")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl_ListBuffer16 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
