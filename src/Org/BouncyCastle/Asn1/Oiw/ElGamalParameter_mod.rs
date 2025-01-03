#[cfg(feature = "Org+BouncyCastle+Asn1+Oiw+ElGamalParameter")]
#[repr(C)]
#[derive(Debug)]
pub struct ElGamalParameter {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
    pub g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Oiw+ElGamalParameter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Oiw::ElGamalParameter
    => "Org.BouncyCastle.Asn1.Oiw"."ElGamalParameter"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Oiw+ElGamalParameter")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Oiw::ElGamalParameter {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Oiw+ElGamalParameter")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Oiw::ElGamalParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Oiw+ElGamalParameter")]
impl crate::Org::BouncyCastle::Asn1::Oiw::ElGamalParameter {
    pub fn New_Asn1Sequence1(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_BigInteger_BigInteger0(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, g))?;
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
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger_BigInteger0(
        &mut self,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, g))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_G(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_G", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_P(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_P", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Oiw+ElGamalParameter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Oiw::ElGamalParameter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
