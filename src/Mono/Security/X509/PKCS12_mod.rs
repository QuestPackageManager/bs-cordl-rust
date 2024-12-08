#[cfg(feature = "Mono+Security+X509+PKCS12+DeriveBytes")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS12_DeriveBytes {
    __cordl_parent: crate::System::Object,
    pub _hashName: *mut crate::System::String,
    pub _iterations: i32,
    pub _password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Mono+Security+X509+PKCS12+DeriveBytes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::PKCS12_DeriveBytes =>
    "Mono.Security.X509"."PKCS12/DeriveBytes"
);
#[cfg(feature = "Mono+Security+X509+PKCS12+DeriveBytes")]
impl std::ops::Deref for crate::Mono::Security::X509::PKCS12_DeriveBytes {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+PKCS12+DeriveBytes")]
impl std::ops::DerefMut for crate::Mono::Security::X509::PKCS12_DeriveBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+PKCS12+DeriveBytes")]
impl crate::Mono::Security::X509::PKCS12_DeriveBytes {
    pub fn Adjust(
        &mut self,
        a: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        aOff: i32,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Adjust", (a, aOff, b))?;
        Ok(__cordl_ret)
    }
    pub fn Derive(
        &mut self,
        diversifier: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Derive", (diversifier, n))?;
        Ok(__cordl_ret)
    }
    pub fn DeriveIV(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DeriveIV", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn DeriveKey(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DeriveKey", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn DeriveMAC(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DeriveMAC", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn set_HashName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HashName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IterationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IterationCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Password(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Password", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Salt(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Salt", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+X509+PKCS12+DeriveBytes")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::X509::PKCS12_DeriveBytes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+X509+PKCS12")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS12 {
    __cordl_parent: crate::System::Object,
    pub _password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _keyBags: *mut crate::System::Collections::ArrayList,
    pub _secretBags: *mut crate::System::Collections::ArrayList,
    pub _certs: *mut crate::Mono::Security::X509::X509CertificateCollection,
    pub _keyBagsChanged: bool,
    pub _secretBagsChanged: bool,
    pub _certsChanged: bool,
    pub _iterations: i32,
    pub _safeBags: *mut crate::System::Collections::ArrayList,
    pub _rng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
}
#[cfg(feature = "Mono+Security+X509+PKCS12")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::PKCS12 =>
    "Mono.Security.X509"."PKCS12"
);
#[cfg(feature = "Mono+Security+X509+PKCS12")]
impl std::ops::Deref for crate::Mono::Security::X509::PKCS12 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+PKCS12")]
impl std::ops::DerefMut for crate::Mono::Security::X509::PKCS12 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+PKCS12")]
impl crate::Mono::Security::X509::PKCS12 {
    #[cfg(feature = "Mono+Security+X509+PKCS12+DeriveBytes")]
    pub type DeriveBytes = crate::Mono::Security::X509::PKCS12_DeriveBytes;
    pub fn AddCertificate_IDictionary1(
        &mut self,
        cert: *mut crate::Mono::Security::X509::X509Certificate,
        attributes: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCertificate", (cert, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn AddCertificate_X509Certificate0(
        &mut self,
        cert: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCertificate", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn AddPrivateKey(
        &mut self,
        pki: *mut crate::Mono::Security::Cryptography::PKCS8_PrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPrivateKey", (pki))?;
        Ok(__cordl_ret)
    }
    pub fn CertificateSafeBag(
        &mut self,
        x509: *mut crate::Mono::Security::X509::X509Certificate,
        attributes: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("CertificateSafeBag", (x509, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn Compare(
        &mut self,
        expected: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        actual: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Compare", (expected, actual))?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Decode", (data))?;
        Ok(__cordl_ret)
    }
    pub fn Decrypt_PKCS7_EncryptedData1(
        &mut self,
        ed: *mut crate::Mono::Security::PKCS7_EncryptedData,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Decrypt", (ed))?;
        Ok(__cordl_ret)
    }
    pub fn Decrypt_String_Il2CppArray_i32_Il2CppArray0(
        &mut self,
        algorithmOid: *mut crate::System::String,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        encryptedData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Decrypt", (algorithmOid, salt, iterationCount, encryptedData))?;
        Ok(__cordl_ret)
    }
    pub fn Encrypt(
        &mut self,
        algorithmOid: *mut crate::System::String,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Encrypt", (algorithmOid, salt, iterationCount, data))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptedContentInfo(
        &mut self,
        safeBags: *mut crate::Mono::Security::ASN1,
        algorithmOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::PKCS7_ContentInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::PKCS7_ContentInfo = __cordl_object
            .invoke("EncryptedContentInfo", (safeBags, algorithmOid))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExistingParameters(
        &mut self,
        found: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::DSAParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::DSAParameters = __cordl_object
            .invoke("GetExistingParameters", (found))?;
        Ok(__cordl_ret)
    }
    pub fn GetSymmetricAlgorithm(
        &mut self,
        algorithmOid: *mut crate::System::String,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::SymmetricAlgorithm,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::SymmetricAlgorithm = __cordl_object
            .invoke("GetSymmetricAlgorithm", (algorithmOid, salt, iterationCount))?;
        Ok(__cordl_ret)
    }
    pub fn MAC(
        &mut self,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterations: i32,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("MAC", (password, salt, iterations, data))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_String2(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data, password))?;
        Ok(__cordl_object)
    }
    pub fn ReadSafeBag(
        &mut self,
        safeBag: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadSafeBag", (safeBag))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveCertificate_IDictionary1(
        &mut self,
        cert: *mut crate::Mono::Security::X509::X509Certificate,
        attrs: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCertificate", (cert, attrs))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveCertificate_X509Certificate0(
        &mut self,
        cert: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCertificate", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_String2(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data, password))?;
        Ok(__cordl_ret)
    }
    pub fn get_Certificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509CertificateCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509CertificateCollection = __cordl_object
            .invoke("get_Certificates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IterationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_IterationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_Keys", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RNG(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::RandomNumberGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::RandomNumberGenerator = __cordl_object
            .invoke("get_RNG", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IterationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IterationCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Password(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Password", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+X509+PKCS12")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::PKCS12 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
