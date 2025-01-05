#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+ZStream")]
#[repr(C)]
#[derive(Debug)]
pub struct ZStream {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub next_in: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub next_in_index: i32,
    pub avail_in: i32,
    pub total_in: i64,
    pub next_out: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub next_out_index: i32,
    pub avail_out: i32,
    pub total_out: i64,
    pub msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub dstate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::Deflate,
    >,
    pub istate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::Inflate,
    >,
    pub data_type: i32,
    pub adler: i64,
    pub _adler: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::Adler32,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+ZStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::ZStream =>
    "Org.BouncyCastle.Utilities.Zlib"."ZStream"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+ZStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::ZStream {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+ZStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::ZStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+ZStream")]
impl crate::Org::BouncyCastle::Utilities::Zlib::ZStream {
    pub const DEF_WBITS: i32 = 15i32;
    pub const MAX_MEM_LEVEL: i32 = 9i32;
    pub const MAX_WBITS: i32 = 15i32;
    pub const Z_BUF_ERROR: i32 = -5i32;
    pub const Z_DATA_ERROR: i32 = -3i32;
    pub const Z_ERRNO: i32 = -1i32;
    pub const Z_FINISH: i32 = 4i32;
    pub const Z_FULL_FLUSH: i32 = 3i32;
    pub const Z_MEM_ERROR: i32 = -4i32;
    pub const Z_NEED_DICT: i32 = 2i32;
    pub const Z_NO_FLUSH: i32 = 0i32;
    pub const Z_OK: i32 = 0i32;
    pub const Z_PARTIAL_FLUSH: i32 = 1i32;
    pub const Z_STREAM_END: i32 = 1i32;
    pub const Z_STREAM_ERROR: i32 = -2i32;
    pub const Z_SYNC_FLUSH: i32 = 2i32;
    pub const Z_VERSION_ERROR: i32 = -6i32;
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
    pub fn deflate(&mut self, flush: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflate", (flush))?;
        Ok(__cordl_ret.into())
    }
    pub fn deflateEnd(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflateEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn deflateInit__cordl_bool1(
        &mut self,
        level: i32,
        nowrap: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflateInit", (level, nowrap))?;
        Ok(__cordl_ret.into())
    }
    pub fn deflateInit_i32_0(
        &mut self,
        level: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflateInit", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn deflateInit_i32_2(
        &mut self,
        level: i32,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflateInit", (level, bits))?;
        Ok(__cordl_ret.into())
    }
    pub fn deflateInit_i32__cordl_bool3(
        &mut self,
        level: i32,
        bits: i32,
        nowrap: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("deflateInit", (level, bits, nowrap))?;
        Ok(__cordl_ret.into())
    }
    pub fn deflateParams(
        &mut self,
        level: i32,
        strategy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("deflateParams", (level, strategy))?;
        Ok(__cordl_ret.into())
    }
    pub fn deflateSetDictionary(
        &mut self,
        dictionary: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dictLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("deflateSetDictionary", (dictionary, dictLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn flush_pending(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("flush_pending", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn free(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("free", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn inflate(&mut self, f: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflate", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflateEnd(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn inflateInit_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn inflateInit__cordl_bool1(
        &mut self,
        nowrap: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateInit", (nowrap))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflateInit_i32_2(&mut self, w: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateInit", (w))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflateInit_i32__cordl_bool3(
        &mut self,
        w: i32,
        nowrap: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateInit", (w, nowrap))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflateSetDictionary(
        &mut self,
        dictionary: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dictLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("inflateSetDictionary", (dictionary, dictLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflateSync(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateSync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn read_buf(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("read_buf", (buf, start, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+ZStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::ZStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
