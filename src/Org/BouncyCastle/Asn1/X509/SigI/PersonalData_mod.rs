#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
#[repr(C)]
#[derive(Debug)]
pub struct PersonalData {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub nameOrPseudonym: *mut crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
    pub nameDistinguisher: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub dateOfBirth: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub placeOfBirth: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
    pub gender: *mut quest_hook::libil2cpp::Il2CppString,
    pub postalAddress: *mut crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::SigI::PersonalData =>
    "Org.BouncyCastle.Asn1.X509.SigI"."PersonalData"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    pub fn GetInstance(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Asn1Sequence0(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_NameOrPseudonym_BigInteger_DerGeneralizedTime_DirectoryString_Il2CppString_DirectoryString1(
        nameOrPseudonym: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        >,
        nameDistinguisher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        dateOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        placeOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
        gender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        postalAddress: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    nameOrPseudonym,
                    nameDistinguisher,
                    dateOfBirth,
                    placeOfBirth,
                    gender,
                    postalAddress,
                ),
            )?;
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
    pub fn _ctor_Asn1Sequence0(
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
    pub fn _ctor_NameOrPseudonym_BigInteger_DerGeneralizedTime_DirectoryString_Il2CppString_DirectoryString1(
        &mut self,
        nameOrPseudonym: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        >,
        nameDistinguisher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        dateOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
        placeOfBirth: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
        gender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        postalAddress: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    nameOrPseudonym,
                    nameDistinguisher,
                    dateOfBirth,
                    placeOfBirth,
                    gender,
                    postalAddress,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateOfBirth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerGeneralizedTime>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        > = __cordl_object.invoke("get_DateOfBirth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Gender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Gender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NameDistinguisher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_NameDistinguisher", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NameOrPseudonym(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SigI::NameOrPseudonym,
        > = __cordl_object.invoke("get_NameOrPseudonym", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PlaceOfBirth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X500::DirectoryString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        > = __cordl_object.invoke("get_PlaceOfBirth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PostalAddress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X500::DirectoryString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X500::DirectoryString,
        > = __cordl_object.invoke("get_PostalAddress", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SigI+PersonalData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::SigI::PersonalData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
