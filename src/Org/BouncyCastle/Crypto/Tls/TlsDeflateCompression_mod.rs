#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression+DeflateOutputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsDeflateCompression_DeflateOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::Zlib::ZOutputStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression+DeflateOutputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression_DeflateOutputStream =>
    "Org.BouncyCastle.Crypto.Tls"."TlsDeflateCompression/DeflateOutputStream"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression+DeflateOutputStream")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression_DeflateOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::Zlib::ZOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression+DeflateOutputStream")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression_DeflateOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression+DeflateOutputStream")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression_DeflateOutputStream {
    pub fn _ctor(
        &mut self,
        output: *mut crate::System::IO::Stream,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        compress: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (output, z, compress))?;
        Ok(__cordl_ret)
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        output: *mut crate::System::IO::Stream,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        compress: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (output, z, compress))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression+DeflateOutputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression_DeflateOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsDeflateCompression {
    __cordl_parent: crate::System::Object,
    pub zIn: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    pub zOut: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression =>
    "Org.BouncyCastle.Crypto.Tls"."TlsDeflateCompression"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression {
    pub const LEVEL_DEFAULT: i32 = -1i32;
    pub const LEVEL_FASTEST: i32 = 1i32;
    pub const LEVEL_NONE: i32 = 0i32;
    pub const LEVEL_SMALLEST: i32 = 9i32;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression+DeflateOutputStream"
    )]
    pub type DeflateOutputStream = crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression_DeflateOutputStream;
    pub fn Compress(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Compress", (output))?;
        Ok(__cordl_ret)
    }
    pub fn Decompress(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Decompress", (output))?;
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
    pub fn _ctor_i32_1(
        &mut self,
        level: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (level))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(level: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (level))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDeflateCompression")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsDeflateCompression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
