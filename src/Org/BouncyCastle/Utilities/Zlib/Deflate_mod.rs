#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
#[repr(C)]
#[derive(Debug)]
pub struct Deflate_Config {
    __cordl_parent: crate::System::Object,
    pub good_length: i32,
    pub max_lazy: i32,
    pub nice_length: i32,
    pub max_chain: i32,
    pub func: i32,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::Zlib::Deflate_Config =>
    "Org.BouncyCastle.Utilities.Zlib"."Deflate/Config"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate+Config")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::Deflate_Config {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        good_length: i32,
        max_lazy: i32,
        nice_length: i32,
        max_chain: i32,
        func: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (good_length, max_lazy, nice_length, max_chain, func))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        good_length: i32,
        max_lazy: i32,
        nice_length: i32,
        max_chain: i32,
        func: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (good_length, max_lazy, nice_length, max_chain, func),
            )?;
        Ok(__cordl_object)
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
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
#[repr(C)]
#[derive(Debug)]
pub struct Deflate {
    __cordl_parent: crate::System::Object,
    pub strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    pub status: i32,
    pub pending_buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub pending_out: i32,
    pub pending: i32,
    pub noheader: i32,
    pub data_type: u8,
    pub method: u8,
    pub last_flush: i32,
    pub w_size: i32,
    pub w_bits: i32,
    pub w_mask: i32,
    pub window: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub window_size: i32,
    pub prev: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub head: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
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
    pub dyn_ltree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub dyn_dtree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub bl_tree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub l_desc: *mut crate::Org::BouncyCastle::Utilities::Zlib::Tree,
    pub d_desc: *mut crate::Org::BouncyCastle::Utilities::Zlib::Tree,
    pub bl_desc: *mut crate::Org::BouncyCastle::Utilities::Zlib::Tree,
    pub bl_count: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub heap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub heap_len: i32,
    pub heap_max: i32,
    pub depth: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::Deflate =>
    "Org.BouncyCastle.Utilities.Zlib"."Deflate"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Deflate")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::Deflate {
    type Target = crate::System::Object;
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
    pub fn flush_block_only(
        &mut self,
        eof: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("flush_block_only", (eof))?;
        Ok(__cordl_ret)
    }
    pub fn deflateEnd(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflateEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn send_tree(
        &mut self,
        tree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
        max_code: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("send_tree", (tree, max_code))?;
        Ok(__cordl_ret)
    }
    pub fn deflateReset(
        &mut self,
        strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflateReset", (strm))?;
        Ok(__cordl_ret)
    }
    pub fn deflateInit_i32_0(
        &mut self,
        strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        level: i32,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("deflateInit", (strm, level, bits))?;
        Ok(__cordl_ret)
    }
    pub fn deflateInit_ZStream_i32_1(
        &mut self,
        strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        level: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflateInit", (strm, level))?;
        Ok(__cordl_ret)
    }
    pub fn set_data_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_data_type", ())?;
        Ok(__cordl_ret)
    }
    pub fn _tr_align(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_tr_align", ())?;
        Ok(__cordl_ret)
    }
    pub fn copy_block(
        &mut self,
        buf: i32,
        len: i32,
        header: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("copy_block", (buf, len, header))?;
        Ok(__cordl_ret)
    }
    pub fn fill_window(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("fill_window", ())?;
        Ok(__cordl_ret)
    }
    pub fn deflate_fast(&mut self, flush: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflate_fast", (flush))?;
        Ok(__cordl_ret)
    }
    pub fn put_byte_Il2CppArray_i32_i32_0(
        &mut self,
        p: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("put_byte", (p, start, len))?;
        Ok(__cordl_ret)
    }
    pub fn put_byte_u8_1(
        &mut self,
        c: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("put_byte", (c))?;
        Ok(__cordl_ret)
    }
    pub fn _tr_tally(
        &mut self,
        dist: i32,
        lc: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("_tr_tally", (dist, lc))?;
        Ok(__cordl_ret)
    }
    pub fn tr_init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("tr_init", ())?;
        Ok(__cordl_ret)
    }
    pub fn putShortMSB(
        &mut self,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("putShortMSB", (b))?;
        Ok(__cordl_ret)
    }
    pub fn bi_flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("bi_flush", ())?;
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
    pub fn send_bits(
        &mut self,
        val: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("send_bits", (val, length))?;
        Ok(__cordl_ret)
    }
    pub fn deflateParams(
        &mut self,
        strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        _level: i32,
        _strategy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("deflateParams", (strm, _level, _strategy))?;
        Ok(__cordl_ret)
    }
    pub fn scan_tree(
        &mut self,
        tree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
        max_code: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("scan_tree", (tree, max_code))?;
        Ok(__cordl_ret)
    }
    pub fn send_code(
        &mut self,
        c: i32,
        tree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("send_code", (c, tree))?;
        Ok(__cordl_ret)
    }
    pub fn deflateInit2(
        &mut self,
        strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        level: i32,
        method: i32,
        windowBits: i32,
        memLevel: i32,
        strategy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "deflateInit2",
                (strm, level, method, windowBits, memLevel, strategy),
            )?;
        Ok(__cordl_ret)
    }
    pub fn deflate_stored(&mut self, flush: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflate_stored", (flush))?;
        Ok(__cordl_ret)
    }
    pub fn put_short(
        &mut self,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("put_short", (w))?;
        Ok(__cordl_ret)
    }
    pub fn lm_init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("lm_init", ())?;
        Ok(__cordl_ret)
    }
    pub fn build_bl_tree(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("build_bl_tree", ())?;
        Ok(__cordl_ret)
    }
    pub fn deflate_slow(&mut self, flush: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflate_slow", (flush))?;
        Ok(__cordl_ret)
    }
    pub fn send_all_trees(
        &mut self,
        lcodes: i32,
        dcodes: i32,
        blcodes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("send_all_trees", (lcodes, dcodes, blcodes))?;
        Ok(__cordl_ret)
    }
    pub fn _tr_stored_block(
        &mut self,
        buf: i32,
        stored_len: i32,
        eof: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_tr_stored_block", (buf, stored_len, eof))?;
        Ok(__cordl_ret)
    }
    pub fn bi_windup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("bi_windup", ())?;
        Ok(__cordl_ret)
    }
    pub fn _tr_flush_block(
        &mut self,
        buf: i32,
        stored_len: i32,
        eof: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_tr_flush_block", (buf, stored_len, eof))?;
        Ok(__cordl_ret)
    }
    pub fn compress_block(
        &mut self,
        ltree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
        dtree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("compress_block", (ltree, dtree))?;
        Ok(__cordl_ret)
    }
    pub fn deflateSetDictionary(
        &mut self,
        strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        dictionary: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        dictLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("deflateSetDictionary", (strm, dictionary, dictLength))?;
        Ok(__cordl_ret)
    }
    pub fn init_block(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("init_block", ())?;
        Ok(__cordl_ret)
    }
    pub fn deflate(
        &mut self,
        strm: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        flush: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("deflate", (strm, flush))?;
        Ok(__cordl_ret)
    }
    pub fn longest_match(
        &mut self,
        cur_match: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("longest_match", (cur_match))?;
        Ok(__cordl_ret)
    }
    pub fn pqdownheap(
        &mut self,
        tree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
        k: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("pqdownheap", (tree, k))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
