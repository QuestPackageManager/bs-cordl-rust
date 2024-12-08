#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetails")]
#[repr(C)]
#[derive(Debug)]
pub struct RevocationDetails {
    __cordl_parent: crate::System::Object,
    pub revDetails: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevDetails,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetails")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cmp::RevocationDetails =>
    "Org.BouncyCastle.Cmp"."RevocationDetails"
);
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetails")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::RevocationDetails {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetails")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cmp::RevocationDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetails")]
impl crate::Org::BouncyCastle::Cmp::RevocationDetails {
    pub fn New(
        revDetails: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevDetails,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (revDetails))?;
        Ok(__cordl_object)
    }
    pub fn ToASN1Structure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::RevDetails,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevDetails = __cordl_object
            .invoke("ToASN1Structure", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        revDetails: *mut crate::Org::BouncyCastle::Asn1::Cmp::RevDetails,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (revDetails))?;
        Ok(__cordl_ret)
    }
    pub fn get_Issuer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name = __cordl_object
            .invoke("get_Issuer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Subject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name = __cordl_object
            .invoke("get_Subject", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+RevocationDetails")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::RevocationDetails {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
