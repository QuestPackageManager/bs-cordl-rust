#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestedCertificate {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub cert: *mut crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
    pub publicKeyCert: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub attributeCert: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate =>
    "Org.BouncyCastle.Asn1.IsisMtt.Ocsp"."RequestedCertificate"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
impl crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate {
    #[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate+Choice")]
    pub type Choice = crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate_Choice;
    pub fn GetCertificateBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetCertificateBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1TaggedObject0(
        tagged: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagged))?;
        Ok(__cordl_object.into())
    }
    pub fn New_RequestedCertificate_Choice_Il2CppArray2(
        _cordl_type: crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate_Choice,
        certificateOctets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, certificateOctets))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509CertificateStructure1(
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certificate))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1TaggedObject0(
        &mut self,
        tagged: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagged))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RequestedCertificate_Choice_Il2CppArray2(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate_Choice,
        certificateOctets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, certificateOctets))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509CertificateStructure1(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate_Choice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate_Choice = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate+Choice")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestedCertificate_Choice {
    AttributeCertificate = 1i32,
    Certificate = -1i32,
    PublicKeyCertificate = 0i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IsisMtt+Ocsp+RequestedCertificate+Choice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IsisMtt::Ocsp::RequestedCertificate_Choice =>
    "Org.BouncyCastle.Asn1.IsisMtt.Ocsp"."RequestedCertificate/Choice"
);
