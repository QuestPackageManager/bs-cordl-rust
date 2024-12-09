#[cfg(feature = "Org+BouncyCastle+Pkix+TrustAnchor")]
#[repr(C)]
#[derive(Debug)]
pub struct TrustAnchor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub caName: *mut quest_hook::libil2cpp::Il2CppString,
    pub caPrincipal: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub trustedCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    pub ncBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub nc: *mut crate::Org::BouncyCastle::Asn1::X509::NameConstraints,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+TrustAnchor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::TrustAnchor =>
    "Org.BouncyCastle.Pkix"."TrustAnchor"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+TrustAnchor")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::TrustAnchor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+TrustAnchor")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::TrustAnchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+TrustAnchor")]
impl crate::Org::BouncyCastle::Pkix::TrustAnchor {
    pub fn New_Il2CppString_AsymmetricKeyParameter_Il2CppArray2(
        caName: *mut quest_hook::libil2cpp::Il2CppString,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        nameConstraints: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (caName, pubKey, nameConstraints))?;
        Ok(__cordl_object)
    }
    pub fn New_X509Certificate_Il2CppArray0(
        trustedCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        nameConstraints: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trustedCert, nameConstraints))?;
        Ok(__cordl_object)
    }
    pub fn New_X509Name_AsymmetricKeyParameter_Il2CppArray1(
        caPrincipal: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        nameConstraints: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (caPrincipal, pubKey, nameConstraints))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_AsymmetricKeyParameter_Il2CppArray2(
        &mut self,
        caName: *mut quest_hook::libil2cpp::Il2CppString,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        nameConstraints: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (caName, pubKey, nameConstraints))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509Certificate_Il2CppArray0(
        &mut self,
        trustedCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        nameConstraints: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trustedCert, nameConstraints))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509Name_AsymmetricKeyParameter_Il2CppArray1(
        &mut self,
        caPrincipal: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        nameConstraints: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (caPrincipal, pubKey, nameConstraints))?;
        Ok(__cordl_ret)
    }
    pub fn get_CA(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name = __cordl_object
            .invoke("get_CA", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CAName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_CAName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CAPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("get_CAPublicKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetNameConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_GetNameConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TrustedCert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("get_TrustedCert", ())?;
        Ok(__cordl_ret)
    }
    pub fn setNameConstraints(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("setNameConstraints", (bytes))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+TrustAnchor")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Pkix::TrustAnchor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
