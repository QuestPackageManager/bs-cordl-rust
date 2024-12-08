#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateUrl {
    __cordl_parent: crate::System::Object,
    pub mType: u8,
    pub mUrlAndHashList: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::CertificateUrl
    => "Org.BouncyCastle.Crypto.Tls"."CertificateUrl"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CertificateUrl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::CertificateUrl {
    type Target = crate::System::Object;
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
    pub fn get_UrlAndHashList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_UrlAndHashList", ())?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (output))?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: u8,
        urlAndHashList: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, urlAndHashList))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: u8,
        urlAndHashList: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, urlAndHashList))?;
        Ok(__cordl_object)
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
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncodeTo", (output))?;
        Ok(__cordl_ret)
    }
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
