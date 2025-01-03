#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECDomainParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ECDomainParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    pub seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    pub n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub h: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub hInv: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECDomainParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."ECDomainParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECDomainParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECDomainParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECDomainParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters {
    pub fn Equals_ECDomainParameters1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetSeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ECCurve_ECPoint_BigInteger1(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, g, n))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ECCurve_ECPoint_BigInteger_BigInteger2(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        h: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, g, n, h))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ECCurve_ECPoint_BigInteger_BigInteger_Il2CppArray3(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        h: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, g, n, h, seed))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X9ECParameters0(
        x9: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (x9))?;
        Ok(__cordl_object.into())
    }
    pub fn ValidatePrivateScalar(
        &mut self,
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("ValidatePrivateScalar", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidatePublicPoint_ECCurve_ECPoint1(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidatePublicPoint", (c, q))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidatePublicPoint_ECPoint0(
        &mut self,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = __cordl_object.invoke("ValidatePublicPoint", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ECCurve_ECPoint_BigInteger1(
        &mut self,
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, g, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ECCurve_ECPoint_BigInteger_BigInteger2(
        &mut self,
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        h: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, g, n, h))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ECCurve_ECPoint_BigInteger_BigInteger_Il2CppArray3(
        &mut self,
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        h: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, g, n, h, seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X9ECParameters0(
        &mut self,
        x9: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (x9))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Curve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve,
        > = __cordl_object.invoke("get_Curve", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_G(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = __cordl_object.invoke("get_G", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_H(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_H", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HInv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_HInv", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_N(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_N", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECDomainParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
