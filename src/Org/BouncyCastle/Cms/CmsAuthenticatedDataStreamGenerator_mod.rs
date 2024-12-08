#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator+CmsAuthenticatedDataOutputStream"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthenticatedDataStreamGenerator_CmsAuthenticatedDataOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub macStream: *mut crate::System::IO::Stream,
    pub mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    pub cGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    pub authGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    pub eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator+CmsAuthenticatedDataOutputStream"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator_CmsAuthenticatedDataOutputStream
    => "Org.BouncyCastle.Cms"
    ."CmsAuthenticatedDataStreamGenerator/CmsAuthenticatedDataOutputStream"
);
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator+CmsAuthenticatedDataOutputStream"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator_CmsAuthenticatedDataOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator+CmsAuthenticatedDataOutputStream"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator_CmsAuthenticatedDataOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator+CmsAuthenticatedDataOutputStream"
)]
impl crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator_CmsAuthenticatedDataOutputStream {
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
    pub fn _ctor(
        &mut self,
        macStream: *mut crate::System::IO::Stream,
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
        cGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        authGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (macStream, mac, cGen, authGen, eiGen))?;
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
    pub fn New(
        macStream: *mut crate::System::IO::Stream,
        mac: *mut crate::Org::BouncyCastle::Crypto::IMac,
        cGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        authGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (macStream, mac, cGen, authGen, eiGen))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator+CmsAuthenticatedDataOutputStream"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator_CmsAuthenticatedDataOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthenticatedDataStreamGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsAuthenticatedGenerator,
    pub _bufferSize: i32,
    pub _berEncodeRecipientSet: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator =>
    "Org.BouncyCastle.Cms"."CmsAuthenticatedDataStreamGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator {
    type Target = crate::Org::BouncyCastle::Cms::CmsAuthenticatedGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator+CmsAuthenticatedDataOutputStream"
    )]
    pub type CmsAuthenticatedDataOutputStream = crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator_CmsAuthenticatedDataOutputStream;
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
    pub fn Open_String_CipherKeyGenerator0(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
        macOid: *mut crate::System::String,
        keyGen: *mut crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStr, macOid, keyGen))?;
        Ok(__cordl_ret)
    }
    pub fn Open_AlgorithmIdentifier_ICipherParameters_Asn1EncodableVector1(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
        macAlgId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        cipherParameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        recipientInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStr, macAlgId, cipherParameters, recipientInfos))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String2(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
        encryptionOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStr, encryptionOid))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String_i32_3(
        &mut self,
        outStr: *mut crate::System::IO::Stream,
        encryptionOid: *mut crate::System::String,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStr, encryptionOid, keySize))?;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SecureRandom1(
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_rand))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataStreamGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataStreamGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
