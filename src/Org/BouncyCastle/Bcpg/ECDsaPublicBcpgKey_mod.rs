#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDsaPublicBcpgKey")]
#[repr(C)]
#[derive(Debug)]
pub struct ECDsaPublicBcpgKey {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::ECPublicBcpgKey,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDsaPublicBcpgKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::ECDsaPublicBcpgKey =>
    "Org.BouncyCastle.Bcpg"."ECDsaPublicBcpgKey"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDsaPublicBcpgKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::ECDsaPublicBcpgKey {
    type Target = crate::Org::BouncyCastle::Bcpg::ECPublicBcpgKey;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDsaPublicBcpgKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::ECDsaPublicBcpgKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDsaPublicBcpgKey")]
impl crate::Org::BouncyCastle::Bcpg::ECDsaPublicBcpgKey {
    pub fn New_BcpgInputStream0(
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_BigInteger2(
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        encodedPoint: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, encodedPoint))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_ECPoint1(
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, point))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgIn))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_BigInteger2(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        encodedPoint: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, encodedPoint))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_ECPoint1(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, point))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDsaPublicBcpgKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::ECDsaPublicBcpgKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}