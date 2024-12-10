#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsEnvelopedHelper =>
    "Org.BouncyCastle.Cms"."CmsEnvelopedHelper"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
    )]
    pub type CmsAuthenticatedSecureReadable = crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable;
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable"
    )]
    pub type CmsEnvelopedSecureReadable = crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable;
    pub fn CreateAsymmetricCipher(
        &mut self,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBufferedCipher>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        > = __cordl_object.invoke("CreateAsymmetricCipher", (encryptionOid))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWrapper(
        &mut self,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IWrapper>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IWrapper,
        > = __cordl_object.invoke("CreateWrapper", (encryptionOid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAsymmetricEncryptionAlgName(
        &mut self,
        encryptionAlgOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAsymmetricEncryptionAlgName", (encryptionAlgOid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeySize(
        &mut self,
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetKeySize", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRfc3211WrapperName(
        &mut self,
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetRfc3211WrapperName", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    pub readable: *mut crate::Org::BouncyCastle::Cms::CmsReadable,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable =>
    "Org.BouncyCastle.Cms"."CmsEnvelopedHelper/CmsAuthenticatedSecureReadable"
);
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    pub fn GetReadable(
        &mut self,
        sKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsReadable,
        > = __cordl_object.invoke("GetReadable", (sKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, readable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, readable))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = __cordl_object.invoke("get_Algorithm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CryptoObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_CryptoObject", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsAuthenticatedSecureReadable"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsAuthenticatedSecureReadable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub cipher: *mut crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    pub readable: *mut crate::Org::BouncyCastle::Cms::CmsReadable,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable =>
    "Org.BouncyCastle.Cms"."CmsEnvelopedHelper/CmsEnvelopedSecureReadable"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    pub fn GetReadable(
        &mut self,
        sKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsReadable,
        > = __cordl_object.invoke("GetReadable", (sKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, readable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        readable: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsReadable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, readable))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = __cordl_object.invoke("get_Algorithm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CryptoObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_CryptoObject", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedHelper+CmsEnvelopedSecureReadable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedHelper_CmsEnvelopedSecureReadable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
