#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECNamedDomainParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ECNamedDomainParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    pub name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECNamedDomainParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."ECNamedDomainParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECNamedDomainParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECNamedDomainParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECNamedDomainParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters {
    pub fn New_ECCurve_ECPoint_BigInteger2(
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        g: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, curve, g, n))?;
        Ok(__cordl_object)
    }
    pub fn New_ECCurve_ECPoint_BigInteger_BigInteger3(
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        g: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        h: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, curve, g, n, h))?;
        Ok(__cordl_object)
    }
    pub fn New_ECCurve_ECPoint_BigInteger_BigInteger_Il2CppArray4(
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        g: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        h: *mut crate::Org::BouncyCastle::Math::BigInteger,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, curve, g, n, h, seed))?;
        Ok(__cordl_object)
    }
    pub fn New_ECDomainParameters0(
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        dp: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, dp))?;
        Ok(__cordl_object)
    }
    pub fn New_X9ECParameters1(
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        x9: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, x9))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ECCurve_ECPoint_BigInteger2(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        g: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, curve, g, n))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECCurve_ECPoint_BigInteger_BigInteger3(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        g: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        h: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, curve, g, n, h))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECCurve_ECPoint_BigInteger_BigInteger_Il2CppArray4(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        g: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        h: *mut crate::Org::BouncyCastle::Math::BigInteger,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, curve, g, n, h, seed))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECDomainParameters0(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        dp: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, dp))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X9ECParameters1(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        x9: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, x9))?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECNamedDomainParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
