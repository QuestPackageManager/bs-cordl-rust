#[cfg(feature = "Org+BouncyCastle+Ocsp+Req")]
#[repr(C)]
#[derive(Debug)]
pub struct Req {
    __cordl_parent: crate::Org::BouncyCastle::X509::X509ExtensionBase,
    pub req: *mut crate::Org::BouncyCastle::Asn1::Ocsp::Request,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+Req")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::Req =>
    "Org.BouncyCastle.Ocsp"."Req"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+Req")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::Req {
    type Target = crate::Org::BouncyCastle::X509::X509ExtensionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+Req")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::Req {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+Req")]
impl crate::Org::BouncyCastle::Ocsp::Req {
    pub fn get_SingleRequestExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_SingleRequestExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCertID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Ocsp::CertificateID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Ocsp::CertificateID = __cordl_object
            .invoke("GetCertID", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        req: *mut crate::Org::BouncyCastle::Asn1::Ocsp::Request,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (req))?;
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
    pub fn New(
        req: *mut crate::Org::BouncyCastle::Asn1::Ocsp::Request,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (req))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+Req")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Ocsp::Req {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
