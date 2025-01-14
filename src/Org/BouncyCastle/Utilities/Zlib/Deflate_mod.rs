#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
#[repr(C)]
#[derive(Debug)]
pub struct Deflate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub strm: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    >,
    pub status: i32,
    pub pending_buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub pending_out: i32,
    pub pending: i32,
    pub noheader: i32,
    pub data_type: u8,
    pub method: u8,
    pub last_flush: i32,
    pub w_size: i32,
    pub w_bits: i32,
    pub w_mask: i32,
    pub window: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub window_size: i32,
    pub prev: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub head: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub ins_h: i32,
    pub hash_size: i32,
    pub hash_bits: i32,
    pub hash_mask: i32,
    pub hash_shift: i32,
    pub block_start: i32,
    pub match_length: i32,
    pub prev_match: i32,
    pub match_available: i32,
    pub strstart: i32,
    pub match_start: i32,
    pub lookahead: i32,
    pub prev_length: i32,
    pub max_chain_length: i32,
    pub max_lazy_match: i32,
    pub level: i32,
    pub strategy: i32,
    pub good_match: i32,
    pub nice_match: i32,
    pub dyn_ltree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub dyn_dtree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub bl_tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub l_desc: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::Tree,
    >,
    pub d_desc: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::Tree,
    >,
    pub bl_desc: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::Tree,
    >,
    pub bl_count: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub heap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub heap_len: i32,
    pub heap_max: i32,
    pub depth: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub l_buf: i32,
    pub lit_bufsize: i32,
    pub last_lit: i32,
    pub d_buf: i32,
    pub opt_len: i32,
    pub static_len: i32,
    pub matches: i32,
    pub last_eob_len: i32,
    pub bi_buf: u32,
    pub bi_valid: i32,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Utilities::Zlib::Deflate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Utilities.Zlib";
    const CLASS_NAME: &'static str = "Deflate";
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
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::Deflate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::Deflate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
impl crate::Org::BouncyCastle::Utilities::Zlib::Deflate {
    pub const BL_CODES: i32 = 19i32;
    pub const BUSY_STATE: i32 = 113i32;
    pub const BlockDone: i32 = 1i32;
    pub const Buf_size: i32 = 16i32;
    pub const DEF_MEM_LEVEL: i32 = 8i32;
    pub const DYN_TREES: i32 = 2i32;
    pub const D_CODES: i32 = 30i32;
    pub const END_BLOCK: i32 = 256i32;
    pub const FAST: i32 = 1i32;
    pub const FINISH_STATE: i32 = 666i32;
    pub const FinishDone: i32 = 3i32;
    pub const FinishStarted: i32 = 2i32;
    pub const HEAP_SIZE: i32 = 573i32;
    pub const INIT_STATE: i32 = 42i32;
    pub const LENGTH_CODES: i32 = 29i32;
    pub const LITERALS: i32 = 256i32;
    pub const L_CODES: i32 = 286i32;
    pub const MAX_BITS: i32 = 15i32;
    pub const MAX_MATCH: i32 = 258i32;
    pub const MAX_MEM_LEVEL: i32 = 9i32;
    pub const MAX_WBITS: i32 = 15i32;
    pub const MIN_LOOKAHEAD: i32 = 262i32;
    pub const MIN_MATCH: i32 = 3i32;
    pub const NeedMore: i32 = 0i32;
    pub const PRESET_DICT: i32 = 32i32;
    pub const REPZ_11_138: i32 = 18i32;
    pub const REPZ_3_10: i32 = 17i32;
    pub const REP_3_6: i32 = 16i32;
    pub const SLOW: i32 = 2i32;
    pub const STATIC_TREES: i32 = 1i32;
    pub const STORED: i32 = 0i32;
    pub const STORED_BLOCK: i32 = 0i32;
    pub const Z_ASCII: i32 = 1i32;
    pub const Z_BINARY: i32 = 0i32;
    pub const Z_BUF_ERROR: i32 = -5i32;
    pub const Z_DATA_ERROR: i32 = -3i32;
    pub const Z_DEFAULT_COMPRESSION: i32 = -1i32;
    pub const Z_DEFAULT_STRATEGY: i32 = 0i32;
    pub const Z_DEFLATED: i32 = 8i32;
    pub const Z_ERRNO: i32 = -1i32;
    pub const Z_FILTERED: i32 = 1i32;
    pub const Z_FINISH: i32 = 4i32;
    pub const Z_FULL_FLUSH: i32 = 3i32;
    pub const Z_HUFFMAN_ONLY: i32 = 2i32;
    pub const Z_MEM_ERROR: i32 = -4i32;
    pub const Z_NEED_DICT: i32 = 2i32;
    pub const Z_NO_FLUSH: i32 = 0i32;
    pub const Z_OK: i32 = 0i32;
    pub const Z_PARTIAL_FLUSH: i32 = 1i32;
    pub const Z_STREAM_END: i32 = 1i32;
    pub const Z_STREAM_ERROR: i32 = -2i32;
    pub const Z_SYNC_FLUSH: i32 = 2i32;
    pub const Z_UNKNOWN: i32 = 2i32;
    pub const Z_VERSION_ERROR: i32 = -6i32;
    #[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
    pub type Config = crate::Org::BouncyCastle::Utilities::Zlib::Deflate_Config;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _tr_align(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("_tr_align")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "_tr_align", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _tr_flush_block(
        &mut self,
        buf: i32,
        stored_len: i32,
        eof: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("_tr_flush_block")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "_tr_flush_block", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buf, stored_len, eof))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _tr_stored_block(
        &mut self,
        buf: i32,
        stored_len: i32,
        eof: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("_tr_stored_block")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "_tr_stored_block", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buf, stored_len, eof))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _tr_tally(
        &mut self,
        dist: i32,
        lc: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("_tr_tally")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "_tr_tally", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (dist, lc)) };
        Ok(__cordl_ret.into())
    }
    pub fn bi_flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("bi_flush")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bi_flush", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn bi_windup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("bi_windup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bi_windup", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn build_bl_tree(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("build_bl_tree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "build_bl_tree", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn compress_block(
        &mut self,
        ltree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        dtree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("compress_block")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "compress_block", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ltree, dtree))
        };
        Ok(__cordl_ret.into())
    }
    pub fn copy_block(
        &mut self,
        buf: i32,
        len: i32,
        header: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("copy_block")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "copy_block", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buf, len, header))
        };
        Ok(__cordl_ret.into())
    }
    pub fn deflate(
        &mut self,
        strm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        >,
        flush: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                    >,
                    i32,
                ),
                i32,
                2usize,
            >("deflate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflate", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (strm, flush)) };
        Ok(__cordl_ret.into())
    }
    pub fn deflateEnd(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("deflateEnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflateEnd", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn deflateInit2(
        &mut self,
        strm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        >,
        level: i32,
        method: i32,
        windowBits: i32,
        memLevel: i32,
        strategy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                    >,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                ),
                i32,
                6usize,
            >("deflateInit2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflateInit2", 6usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (strm, level, method, windowBits, memLevel, strategy),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn deflateInit_ZStream_i32_1(
        &mut self,
        strm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        >,
        level: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                    >,
                    i32,
                ),
                i32,
                2usize,
            >("deflateInit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflateInit", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (strm, level)) };
        Ok(__cordl_ret.into())
    }
    pub fn deflateInit_i32_0(
        &mut self,
        strm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        >,
        level: i32,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                    >,
                    i32,
                    i32,
                ),
                i32,
                3usize,
            >("deflateInit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflateInit", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (strm, level, bits))
        };
        Ok(__cordl_ret.into())
    }
    pub fn deflateParams(
        &mut self,
        strm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        >,
        _level: i32,
        _strategy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                    >,
                    i32,
                    i32,
                ),
                i32,
                3usize,
            >("deflateParams")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflateParams", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (strm, _level, _strategy))
        };
        Ok(__cordl_ret.into())
    }
    pub fn deflateReset(
        &mut self,
        strm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                >),
                i32,
                1usize,
            >("deflateReset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflateReset", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (strm)) };
        Ok(__cordl_ret.into())
    }
    pub fn deflateSetDictionary(
        &mut self,
        strm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        >,
        dictionary: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dictLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                ),
                i32,
                3usize,
            >("deflateSetDictionary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflateSetDictionary", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (strm, dictionary, dictLength))
        };
        Ok(__cordl_ret.into())
    }
    pub fn deflate_fast(&mut self, flush: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("deflate_fast")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflate_fast", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (flush)) };
        Ok(__cordl_ret.into())
    }
    pub fn deflate_slow(&mut self, flush: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("deflate_slow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflate_slow", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (flush)) };
        Ok(__cordl_ret.into())
    }
    pub fn deflate_stored(&mut self, flush: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("deflate_stored")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "deflate_stored", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (flush)) };
        Ok(__cordl_ret.into())
    }
    pub fn fill_window(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("fill_window")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fill_window", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn flush_block_only(
        &mut self,
        eof: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("flush_block_only")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "flush_block_only", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eof))
        };
        Ok(__cordl_ret.into())
    }
    pub fn init_block(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("init_block")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "init_block", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn lm_init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("lm_init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "lm_init", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn longest_match(
        &mut self,
        cur_match: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("longest_match")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "longest_match", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (cur_match)) };
        Ok(__cordl_ret.into())
    }
    pub fn pqdownheap(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        k: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("pqdownheap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "pqdownheap", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tree, k))
        };
        Ok(__cordl_ret.into())
    }
    pub fn putShortMSB(
        &mut self,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("putShortMSB")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "putShortMSB", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn put_byte_Il2CppArray_i32_i32_0(
        &mut self,
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("put_byte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "put_byte", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (p, start, len))
        };
        Ok(__cordl_ret.into())
    }
    pub fn put_byte_u8_1(
        &mut self,
        c: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u8), quest_hook::libil2cpp::Void, 1usize>("put_byte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "put_byte", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn put_short(
        &mut self,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("put_short")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "put_short", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (w))
        };
        Ok(__cordl_ret.into())
    }
    pub fn scan_tree(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        max_code: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("scan_tree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "scan_tree", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tree, max_code))
        };
        Ok(__cordl_ret.into())
    }
    pub fn send_all_trees(
        &mut self,
        lcodes: i32,
        dcodes: i32,
        blcodes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("send_all_trees")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "send_all_trees", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (lcodes, dcodes, blcodes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn send_bits(
        &mut self,
        val: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("send_bits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "send_bits", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (val, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn send_code(
        &mut self,
        c: i32,
        tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("send_code")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "send_code", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (c, tree))
        };
        Ok(__cordl_ret.into())
    }
    pub fn send_tree(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        max_code: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("send_tree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "send_tree", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tree, max_code))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_data_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("set_data_type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_data_type", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn smaller(
        tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        n: i32,
        m: i32,
        depth: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                bool,
                4usize,
            >("smaller")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "smaller", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (tree, n, m, depth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn tr_init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("tr_init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "tr_init", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::Deflate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
#[repr(C)]
#[derive(Debug)]
pub struct Deflate_Config {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub good_length: i32,
    pub max_lazy: i32,
    pub nice_length: i32,
    pub max_chain: i32,
    pub func: i32,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Utilities::Zlib::Deflate_Config {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Utilities.Zlib";
    const CLASS_NAME: &'static str = "Deflate/Config";
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
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::Deflate_Config {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::Deflate_Config {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
impl crate::Org::BouncyCastle::Utilities::Zlib::Deflate_Config {
    pub fn New(
        good_length: i32,
        max_lazy: i32,
        nice_length: i32,
        max_chain: i32,
        func: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (good_length, max_lazy, nice_length, max_chain, func),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        good_length: i32,
        max_lazy: i32,
        nice_length: i32,
        max_chain: i32,
        func: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (good_length, max_lazy, nice_length, max_chain, func),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::Deflate_Config {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
