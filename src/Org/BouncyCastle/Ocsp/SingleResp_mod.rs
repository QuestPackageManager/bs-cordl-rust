#[cfg(feature = "Org+BouncyCastle+Ocsp+SingleResp")]
#[repr(C)]
#[derive(Debug)]
pub struct SingleResp {
    __cordl_parent: crate::Org::BouncyCastle::X509::X509ExtensionBase,
    pub resp: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+SingleResp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::SingleResp =>
    "Org.BouncyCastle.Ocsp"."SingleResp"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+SingleResp")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::SingleResp {
    type Target = crate::Org::BouncyCastle::X509::X509ExtensionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+SingleResp")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::SingleResp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+SingleResp")]
impl crate::Org::BouncyCastle::Ocsp::SingleResp {
    pub fn GetCertID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Ocsp::CertificateID>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Ocsp::CertificateID,
        > = __cordl_object.invoke("GetCertID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCertStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetCertStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetX509Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Extensions>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        > = __cordl_object.invoke("GetX509Extensions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        resp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resp))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        resp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resp))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
        > = __cordl_object.invoke("get_NextUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SingleExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Extensions>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        > = __cordl_object.invoke("get_SingleExtensions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ThisUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_ThisUpdate", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+SingleResp")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Ocsp::SingleResp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
