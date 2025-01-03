#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertStoreSelector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub authorityKeyIdentifier: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub basicConstraints: i32,
    pub certificate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::X509Certificate,
    >,
    pub certificateValid: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    >,
    pub extendedKeyUsage: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Collections::ISet,
    >,
    pub ignoreX509NameOrdering: bool,
    pub issuer: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Name,
    >,
    pub keyUsage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    pub policy: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Collections::ISet,
    >,
    pub privateKeyValid: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
    >,
    pub serialNumber: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::BigInteger,
    >,
    pub subject: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Name,
    >,
    pub subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub subjectPublicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    >,
    pub subjectPublicKeyAlgID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    >,
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::X509::Store::X509CertStoreSelector =>
    "Org.BouncyCastle.X509.Store"."X509CertStoreSelector"
);
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyBoolArray(
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CopyBoolArray", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopySet(
        s: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CopySet", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubjectPublicKey(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSubjectPublicKey", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IssuersMatch(
        a: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
        b: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IssuersMatch", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Match", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchExtension(
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchExtension", (b, c, oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509CertStoreSelector1(
        o: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509CertStoreSelector1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AuthorityKeyIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_AuthorityKeyIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BasicConstraints(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BasicConstraints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        > = __cordl_object.invoke("get_Certificate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateValid(
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
        > = __cordl_object.invoke("get_CertificateValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtendedKeyUsage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = __cordl_object.invoke("get_ExtendedKeyUsage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreX509NameOrdering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreX509NameOrdering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Issuer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        > = __cordl_object.invoke("get_Issuer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IssuerAsString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_IssuerAsString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyUsage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = __cordl_object.invoke("get_KeyUsage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Policy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = __cordl_object.invoke("get_Policy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrivateKeyValid(
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
        > = __cordl_object.invoke("get_PrivateKeyValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Subject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        > = __cordl_object.invoke("get_Subject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubjectAsString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SubjectAsString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubjectKeyIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_SubjectKeyIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubjectPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        > = __cordl_object.invoke("get_SubjectPublicKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubjectPublicKeyAlgID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("get_SubjectPublicKeyAlgID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AuthorityKeyIdentifier(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AuthorityKeyIdentifier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BasicConstraints(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BasicConstraints", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Certificate(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Certificate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CertificateValid(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertificateValid", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExtendedKeyUsage(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExtendedKeyUsage", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreX509NameOrdering(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreX509NameOrdering", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Issuer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Issuer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_KeyUsage(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyUsage", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Policy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Policy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PrivateKeyValid(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Date::DateTimeObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrivateKeyValid", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SerialNumber(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SerialNumber", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Subject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Subject", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SubjectKeyIdentifier(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SubjectKeyIdentifier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SubjectPublicKey(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SubjectPublicKey", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SubjectPublicKeyAlgID(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SubjectPublicKeyAlgID", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl AsRef<crate::Org::BouncyCastle::X509::Store::IX509Selector>
for crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::X509::Store::IX509Selector {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl AsMut<crate::Org::BouncyCastle::X509::Store::IX509Selector>
for crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::X509::Store::IX509Selector {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl AsRef<crate::System::ICloneable>
for crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertStoreSelector")]
impl AsMut<crate::System::ICloneable>
for crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
