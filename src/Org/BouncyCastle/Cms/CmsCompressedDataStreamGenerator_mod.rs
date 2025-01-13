#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsCompressedDataStreamGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bufferSize: i32,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsCompressedDataStreamGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator {
    pub const ZLib: &'static str = "1.2.840.113549.1.9.16.3.8";
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator+CmsCompressedOutputStream"
    )]
    pub type CmsCompressedOutputStream = crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator_CmsCompressedOutputStream;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Open_Il2CppString1(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        contentOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        compressionOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("Open", (outStream, contentOID, compressionOID))?;
        Ok(__cordl_ret.into())
    }
    pub fn Open_Stream_Il2CppString0(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        compressionOID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("Open", (outStream, compressionOID))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator+CmsCompressedOutputStream"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsCompressedDataStreamGenerator_CmsCompressedOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub _out: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::ZOutputStream,
    >,
    pub _sGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    >,
    pub _cGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    >,
    pub _eiGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    >,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator+CmsCompressedOutputStream"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator_CmsCompressedOutputStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsCompressedDataStreamGenerator/CmsCompressedOutputStream";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator+CmsCompressedOutputStream"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator_CmsCompressedOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator+CmsCompressedOutputStream"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator_CmsCompressedOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator+CmsCompressedOutputStream"
)]
impl crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator_CmsCompressedOutputStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        outStream: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZOutputStream,
        >,
        sGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        cGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        eiGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outStream, sGen, cGen, eiGen))?;
        Ok(__cordl_object.into())
    }
    pub fn Write(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (bytes, off, len))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZOutputStream,
        >,
        sGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        cGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
        eiGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outStream, sGen, cGen, eiGen))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsCompressedDataStreamGenerator+CmsCompressedOutputStream"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsCompressedDataStreamGenerator_CmsCompressedOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
