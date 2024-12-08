#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+CertId")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs12Store_CertId {
    __cordl_parent: crate::System::Object,
    pub id: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+CertId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkcs::Pkcs12Store_CertId =>
    "Org.BouncyCastle.Pkcs"."Pkcs12Store/CertId"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+CertId")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::Pkcs12Store_CertId {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+CertId")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkcs::Pkcs12Store_CertId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+CertId")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs12Store_CertId {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_AsymmetricKeyParameter0(
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pubKey))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        id: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_AsymmetricKeyParameter0(
        &mut self,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pubKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        id: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id))?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+CertId")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::Pkcs12Store_CertId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+IgnoresCaseHashtable")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs12Store_IgnoresCaseHashtable {
    __cordl_parent: crate::System::Object,
    pub orig: *mut crate::System::Collections::IDictionary,
    pub keys: *mut crate::System::Collections::IDictionary,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+IgnoresCaseHashtable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable =>
    "Org.BouncyCastle.Pkcs"."Pkcs12Store/IgnoresCaseHashtable"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+IgnoresCaseHashtable")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+IgnoresCaseHashtable")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+IgnoresCaseHashtable")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Remove", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Item", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("get_Keys", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("get_Values", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        alias: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (alias, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+IgnoresCaseHashtable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs12Store {
    __cordl_parent: crate::System::Object,
    pub keys: *mut crate::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable,
    pub localIds: *mut crate::System::Collections::IDictionary,
    pub certs: *mut crate::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable,
    pub chainCerts: *mut crate::System::Collections::IDictionary,
    pub keyCerts: *mut crate::System::Collections::IDictionary,
    pub keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub keyPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub certAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub certPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub useDerEncoding: bool,
    pub unmarkedKeyEntry: *mut crate::Org::BouncyCastle::Pkcs::AsymmetricKeyEntry,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkcs::Pkcs12Store =>
    "Org.BouncyCastle.Pkcs"."Pkcs12Store"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::Pkcs12Store {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkcs::Pkcs12Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs12Store {
    pub const IgnoreUselessPasswordProperty: &'static str = "Org.BouncyCastle.Pkcs12.IgnoreUselessPassword";
    pub const MinIterations: i32 = 1024i32;
    pub const SaltSize: i32 = 20i32;
    #[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+CertId")]
    pub type CertId = crate::Org::BouncyCastle::Pkcs::Pkcs12Store_CertId;
    #[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store+IgnoresCaseHashtable")]
    pub type IgnoresCaseHashtable = crate::Org::BouncyCastle::Pkcs::Pkcs12Store_IgnoresCaseHashtable;
    pub fn ContainsAlias(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsAlias", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteEntry(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteEntry", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn GetAliasesTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("GetAliasesTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCertificate(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::X509CertificateEntry,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::X509CertificateEntry = __cordl_object
            .invoke("GetCertificate", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn GetCertificateAlias(
        &mut self,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCertificateAlias", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn GetCertificateChain(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Pkcs::X509CertificateEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Pkcs::X509CertificateEntry,
        > = __cordl_object.invoke("GetCertificateChain", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn GetKey(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::AsymmetricKeyEntry,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::AsymmetricKeyEntry = __cordl_object
            .invoke("GetKey", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn IsCertificateEntry(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCertificateEntry", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn IsEntryOfType(
        &mut self,
        alias: *mut crate::System::String,
        entryType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEntryOfType", (alias, entryType))?;
        Ok(__cordl_ret)
    }
    pub fn IsKeyEntry(
        &mut self,
        alias: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeyEntry", (alias))?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
        input: *mut crate::System::IO::Stream,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Load", (input, password))?;
        Ok(__cordl_ret)
    }
    pub fn LoadKeyBag(
        &mut self,
        privKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        bagAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadKeyBag", (privKeyInfo, bagAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn LoadPkcs8ShroudedKeyBag(
        &mut self,
        encPrivKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        bagAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        wrongPkcs12Zero: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadPkcs8ShroudedKeyBag",
                (encPrivKeyInfo, bagAttributes, password, wrongPkcs12Zero),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_2() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_DerObjectIdentifier_DerObjectIdentifier_DerObjectIdentifier__cordl_bool1(
        keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        keyPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        certAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        certPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        useDerEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    keyAlgorithm,
                    keyPrfAlgorithm,
                    certAlgorithm,
                    certPrfAlgorithm,
                    useDerEncoding,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_DerObjectIdentifier__cordl_bool0(
        keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        certAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        useDerEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyAlgorithm, certAlgorithm, useDerEncoding))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Il2CppArray3(
        input: *mut crate::System::IO::Stream,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, password))?;
        Ok(__cordl_object)
    }
    pub fn Save(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (stream, password, random))?;
        Ok(__cordl_ret)
    }
    pub fn SetCertificateEntry(
        &mut self,
        alias: *mut crate::System::String,
        certEntry: *mut crate::Org::BouncyCastle::Pkcs::X509CertificateEntry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCertificateEntry", (alias, certEntry))?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyEntry(
        &mut self,
        alias: *mut crate::System::String,
        keyEntry: *mut crate::Org::BouncyCastle::Pkcs::AsymmetricKeyEntry,
        chain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Pkcs::X509CertificateEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyEntry", (alias, keyEntry, chain))?;
        Ok(__cordl_ret)
    }
    pub fn Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Size", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_DerObjectIdentifier_DerObjectIdentifier_DerObjectIdentifier__cordl_bool1(
        &mut self,
        keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        keyPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        certAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        certPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        useDerEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    keyAlgorithm,
                    keyPrfAlgorithm,
                    certAlgorithm,
                    certPrfAlgorithm,
                    useDerEncoding,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_DerObjectIdentifier__cordl_bool0(
        &mut self,
        keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        certAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        useDerEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyAlgorithm, certAlgorithm, useDerEncoding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Il2CppArray3(
        &mut self,
        input: *mut crate::System::IO::Stream,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, password))?;
        Ok(__cordl_ret)
    }
    pub fn get_Aliases(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("get_Aliases", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12Store")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Pkcs::Pkcs12Store {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}