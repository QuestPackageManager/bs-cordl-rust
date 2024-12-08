#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsTypedStream {
    __cordl_parent: crate::System::Object,
    pub _oid: *mut crate::System::String,
    pub _in: *mut crate::System::IO::Stream,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsTypedStream =>
    "Org.BouncyCastle.Cms"."CmsTypedStream"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsTypedStream {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsTypedStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream")]
impl crate::Org::BouncyCastle::Cms::CmsTypedStream {
    pub const BufferSize: i32 = 32768i32;
    #[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream+FullReaderStream")]
    pub type FullReaderStream = crate::Org::BouncyCastle::Cms::CmsTypedStream_FullReaderStream;
    pub fn Drain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Drain", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Stream0(
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inStream))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Stream1(
        oid: *mut crate::System::String,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, inStream))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Stream_i32_2(
        oid: *mut crate::System::String,
        inStream: *mut crate::System::IO::Stream,
        bufSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, inStream, bufSize))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Stream0(
        &mut self,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inStream))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Stream1(
        &mut self,
        oid: *mut crate::System::String,
        inStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, inStream))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Stream_i32_2(
        &mut self,
        oid: *mut crate::System::String,
        inStream: *mut crate::System::IO::Stream,
        bufSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, inStream, bufSize))?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_ContentStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ContentType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsTypedStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream+FullReaderStream")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsTypedStream_FullReaderStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::FilterStream,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream+FullReaderStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsTypedStream_FullReaderStream => "Org.BouncyCastle.Cms"
    ."CmsTypedStream/FullReaderStream"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream+FullReaderStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsTypedStream_FullReaderStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::FilterStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream+FullReaderStream")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsTypedStream_FullReaderStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream+FullReaderStream")]
impl crate::Org::BouncyCastle::Cms::CmsTypedStream_FullReaderStream {
    pub fn New(
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object)
    }
    pub fn Read(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsTypedStream+FullReaderStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsTypedStream_FullReaderStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
