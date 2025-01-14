#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
#[repr(C)]
#[derive(Debug)]
pub struct Tree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub dyn_tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub max_code: i32,
    pub stat_desc: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::StaticTree,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Tree")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Utilities::Zlib::Tree {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Utilities.Zlib";
    const CLASS_NAME: &'static str = "Tree";
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
    pub fn bi_reverse(code: i32, len: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("bi_reverse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bi_reverse", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (code, len)) };
        Ok(__cordl_ret.into())
    }
    pub fn build_tree(
        &mut self,
        s: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::Deflate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::Zlib::Deflate,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("build_tree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "build_tree", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (s))
        };
        Ok(__cordl_ret.into())
    }
    pub fn d_code(dist: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("d_code")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "d_code", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (dist)) };
        Ok(__cordl_ret.into())
    }
    pub fn gen_bitlen(
        &mut self,
        s: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::Deflate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::Zlib::Deflate,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("gen_bitlen")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "gen_bitlen", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (s))
        };
        Ok(__cordl_ret.into())
    }
    pub fn gen_codes(
        tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        max_code: i32,
        bl_count: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("gen_codes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "gen_codes", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tree, max_code, bl_count))
        };
        Ok(__cordl_ret.into())
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
