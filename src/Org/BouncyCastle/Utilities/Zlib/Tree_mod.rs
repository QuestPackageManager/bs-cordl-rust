#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
#[repr(C)]
#[derive(Debug)]
pub struct Tree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub dyn_tree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub max_code: i32,
    pub stat_desc: *mut crate::Org::BouncyCastle::Utilities::Zlib::StaticTree,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::Tree =>
    "Org.BouncyCastle.Utilities.Zlib"."Tree"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::Tree {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::Tree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
impl crate::Org::BouncyCastle::Utilities::Zlib::Tree {
    pub const BL_CODES: i32 = 19i32;
    pub const Buf_size: i32 = 16i32;
    pub const DIST_CODE_LEN: i32 = 512i32;
    pub const D_CODES: i32 = 30i32;
    pub const END_BLOCK: i32 = 256i32;
    pub const HEAP_SIZE: i32 = 573i32;
    pub const LENGTH_CODES: i32 = 29i32;
    pub const LITERALS: i32 = 256i32;
    pub const L_CODES: i32 = 286i32;
    pub const MAX_BITS: i32 = 15i32;
    pub const MAX_BL_BITS: i32 = 7i32;
    pub const REPZ_11_138: i32 = 18i32;
    pub const REPZ_3_10: i32 = 17i32;
    pub const REP_3_6: i32 = 16i32;
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
    pub fn build_tree(
        &mut self,
        s: *mut crate::Org::BouncyCastle::Utilities::Zlib::Deflate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("build_tree", (s))?;
        Ok(__cordl_ret)
    }
    pub fn gen_bitlen(
        &mut self,
        s: *mut crate::Org::BouncyCastle::Utilities::Zlib::Deflate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("gen_bitlen", (s))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::Tree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
