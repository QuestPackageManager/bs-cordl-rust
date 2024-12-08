#[cfg(feature = "Org+BouncyCastle+Ocsp+RevokedStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct RevokedStatus {
    __cordl_parent: crate::Org::BouncyCastle::Ocsp::CertificateStatus,
    pub info: *mut crate::Org::BouncyCastle::Asn1::Ocsp::RevokedInfo,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+RevokedStatus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::RevokedStatus =>
    "Org.BouncyCastle.Ocsp"."RevokedStatus"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+RevokedStatus")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::RevokedStatus {
    type Target = crate::Org::BouncyCastle::Ocsp::CertificateStatus;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+RevokedStatus")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::RevokedStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+RevokedStatus")]
impl crate::Org::BouncyCastle::Ocsp::RevokedStatus {
    pub fn get_RevocationTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_RevocationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasRevocationReason(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasRevocationReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_RevokedInfo0(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Ocsp::RevokedInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DateTime_i32_1(
        &mut self,
        revocationDate: crate::System::DateTime,
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (revocationDate, reason))?;
        Ok(__cordl_ret)
    }
    pub fn get_RevocationReason(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RevocationReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_RevokedInfo0(
        info: *mut crate::Org::BouncyCastle::Asn1::Ocsp::RevokedInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn New_DateTime_i32_1(
        revocationDate: crate::System::DateTime,
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (revocationDate, reason))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+RevokedStatus")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::RevokedStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
