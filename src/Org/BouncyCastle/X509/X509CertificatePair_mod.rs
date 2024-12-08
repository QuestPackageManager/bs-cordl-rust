#[cfg(feature = "Org+BouncyCastle+X509+X509CertificatePair")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificatePair {
    __cordl_parent: crate::System::Object,
    pub forward: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    pub reverse: *mut crate::Org::BouncyCastle::X509::X509Certificate,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificatePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509CertificatePair =>
    "Org.BouncyCastle.X509"."X509CertificatePair"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificatePair")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509CertificatePair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificatePair")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509CertificatePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificatePair")]
impl crate::Org::BouncyCastle::X509::X509CertificatePair {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Forward(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("get_Forward", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509Certificate_X509Certificate0(
        &mut self,
        forward: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        reverse: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (forward, reverse))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CertificatePair1(
        &mut self,
        pair: *mut crate::Org::BouncyCastle::Asn1::X509::CertificatePair,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pair))?;
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
    pub fn get_Reverse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Certificate = __cordl_object
            .invoke("get_Reverse", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_X509Certificate_X509Certificate0(
        forward: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        reverse: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (forward, reverse))?;
        Ok(__cordl_object)
    }
    pub fn New_CertificatePair1(
        pair: *mut crate::Org::BouncyCastle::Asn1::X509::CertificatePair,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pair))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509CertificatePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::X509CertificatePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
