#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspResp")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicOcspResp {
    __cordl_parent: crate::Org::BouncyCastle::X509::X509ExtensionBase,
    pub resp: *mut crate::Org::BouncyCastle::Asn1::Ocsp::BasicOcspResponse,
    pub data: *mut crate::Org::BouncyCastle::Asn1::Ocsp::ResponseData,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspResp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::BasicOcspResp =>
    "Org.BouncyCastle.Ocsp"."BasicOcspResp"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspResp")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::BasicOcspResp {
    type Target = crate::Org::BouncyCastle::X509::X509ExtensionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspResp")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::BasicOcspResp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspResp")]
impl crate::Org::BouncyCastle::Ocsp::BasicOcspResp {
    pub fn GetSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncoded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProducedAt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_ProducedAt", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTbsResponseData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetTbsResponseData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetResponseData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Ocsp::RespData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::RespData = __cordl_object
            .invoke("GetResponseData", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        resp: *mut crate::Org::BouncyCastle::Asn1::Ocsp::BasicOcspResponse,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resp))?;
        Ok(__cordl_ret)
    }
    pub fn get_Responses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Ocsp::SingleResp,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Ocsp::SingleResp,
        > = __cordl_object.invoke("get_Responses", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SignatureAlgOid", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCertificates(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Store = __cordl_object
            .invoke("GetCertificates", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn Verify(
        &mut self,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_ResponseExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_ResponseExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::X509::X509Certificate,
        > = __cordl_object.invoke("GetCerts", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetX509Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("GetX509Extensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCertList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetCertList", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResponderId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Ocsp::RespID> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::RespID = __cordl_object
            .invoke("get_ResponderId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureAlgName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SignatureAlgName", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        resp: *mut crate::Org::BouncyCastle::Asn1::Ocsp::BasicOcspResponse,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resp))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+BasicOcspResp")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::BasicOcspResp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
