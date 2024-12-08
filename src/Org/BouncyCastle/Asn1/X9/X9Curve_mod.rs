#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9Curve")]
#[repr(C)]
#[derive(Debug)]
pub struct X9Curve {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
    pub seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub fieldIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9Curve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X9::X9Curve =>
    "Org.BouncyCastle.Asn1.X9"."X9Curve"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9Curve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X9::X9Curve {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9Curve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X9::X9Curve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9Curve")]
impl crate::Org::BouncyCastle::Asn1::X9::X9Curve {
    pub fn GetSeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_ECCurve0(
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve))?;
        Ok(__cordl_object)
    }
    pub fn New_ECCurve_Il2CppArray1(
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, seed))?;
        Ok(__cordl_object)
    }
    pub fn New_X9FieldID_Asn1Sequence2(
        fieldID: *mut crate::Org::BouncyCastle::Asn1::X9::X9FieldID,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fieldID, seq))?;
        Ok(__cordl_object)
    }
    pub fn New_X9FieldID_BigInteger_BigInteger_Asn1Sequence3(
        fieldID: *mut crate::Org::BouncyCastle::Asn1::X9::X9FieldID,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fieldID, order, cofactor, seq))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECCurve0(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECCurve_Il2CppArray1(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, seed))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X9FieldID_Asn1Sequence2(
        &mut self,
        fieldID: *mut crate::Org::BouncyCastle::Asn1::X9::X9FieldID,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fieldID, seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X9FieldID_BigInteger_BigInteger_Asn1Sequence3(
        &mut self,
        fieldID: *mut crate::Org::BouncyCastle::Asn1::X9::X9FieldID,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fieldID, order, cofactor, seq))?;
        Ok(__cordl_ret)
    }
    pub fn get_Curve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECCurve = __cordl_object
            .invoke("get_Curve", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X9+X9Curve")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::X9::X9Curve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
