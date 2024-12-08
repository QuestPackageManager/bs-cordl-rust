#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub _outer: *mut crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
    pub _out: *mut crate::Org::BouncyCastle::Crypto::IO::CipherStream,
    pub _cGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    pub _envGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    pub _eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream
    => "Org.BouncyCastle.Cms"
    ."CmsEnvelopedDataStreamGenerator/CmsEnvelopedDataOutputStream"
);
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        outer: *mut crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
        outStream: *mut crate::Org::BouncyCastle::Crypto::IO::CipherStream,
        cGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        envGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer, outStream, cGen, envGen, eiGen))?;
        Ok(__cordl_object)
    }
    pub fn Write(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (bytes, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn WriteByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteByte", (b))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
        outStream: *mut crate::Org::BouncyCastle::Crypto::IO::CipherStream,
        cGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        envGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outer, outStream, cGen, envGen, eiGen))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedDataStreamGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator,
    pub _originatorInfo: *mut crate::System::Object,
    pub _unprotectedAttributes: *mut crate::System::Object,
    pub _bufferSize: i32,
    pub _berEncodeRecipientSet: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator => "Org.BouncyCastle.Cms"
    ."CmsEnvelopedDataStreamGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    type Target = crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator+CmsEnvelopedDataOutputStream"
    )]
    pub type CmsEnvelopedDataOutputStream = crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator_CmsEnvelopedDataOutputStream;
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SecureRandom1(
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_rand))?;
        Ok(__cordl_object)
    }
    pub fn Open_AlgorithmIdentifier_ICipherParameters_Asn1EncodableVector1(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        encAlgID: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        cipherParameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        recipientInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream, encAlgID, cipherParameters, recipientInfos))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String2(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        encryptionOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream, encryptionOid))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String_CipherKeyGenerator0(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        encryptionOid: *mut crate::System::String,
        keyGen: *mut crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream, encryptionOid, keyGen))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String_i32_3(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        encryptionOid: *mut crate::System::String,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream, encryptionOid, keySize))?;
        Ok(__cordl_ret)
    }
    pub fn SetBerEncodeRecipients(
        &mut self,
        berEncodeRecipientSet: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBerEncodeRecipients", (berEncodeRecipientSet))?;
        Ok(__cordl_ret)
    }
    pub fn SetBufferSize(
        &mut self,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBufferSize", (bufferSize))?;
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
    pub fn _ctor_SecureRandom1(
        &mut self,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_rand))?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataStreamGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataStreamGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
