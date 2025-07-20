#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
#[repr(C)]
#[derive(Debug)]
pub struct InfBlocks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mode: i32,
    pub left: i32,
    pub table: i32,
    pub index: i32,
    pub blens: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub bb: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub tb: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub codes: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::InfCodes,
    >,
    pub last: i32,
    pub bitk: i32,
    pub bitb: i32,
    pub hufts: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub window: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub end: i32,
    pub read: i32,
    pub write: i32,
    pub checkfn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub check: i64,
    pub inftree: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Zlib::InfTree,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Utilities.Zlib";
    const CLASS_NAME: &'static str = "InfBlocks";
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
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
impl crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks {
    pub const BAD: i32 = 9i32;
    pub const BTREE: i32 = 4i32;
    pub const CODES: i32 = 6i32;
    pub const DONE: i32 = 8i32;
    pub const DRY: i32 = 7i32;
    pub const DTREE: i32 = 5i32;
    pub const LENS: i32 = 1i32;
    pub const MANY: i32 = 1440i32;
    pub const STORED: i32 = 2i32;
    pub const TABLE: i32 = 3i32;
    pub const TYPE: i32 = 0i32;
    pub const Z_BUF_ERROR: i32 = -5i32;
    pub const Z_DATA_ERROR: i32 = -3i32;
    pub const Z_ERRNO: i32 = -1i32;
    pub const Z_MEM_ERROR: i32 = -4i32;
    pub const Z_NEED_DICT: i32 = 2i32;
    pub const Z_OK: i32 = 0i32;
    pub const Z_STREAM_END: i32 = 1i32;
    pub const Z_STREAM_ERROR: i32 = -2i32;
    pub const Z_VERSION_ERROR: i32 = -6i32;
    pub fn New(
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
        checkfn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (z, checkfn, w))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
        checkfn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (z, checkfn, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn free(
        &mut self,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("free")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "free", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn inflate_flush(
        &mut self,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                            >,
                            i32,
                        ),
                        i32,
                        2usize,
                    >("inflate_flush")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "inflate_flush", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (z, r))? };
        Ok(__cordl_ret.into())
    }
    pub fn proc(
        &mut self,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                            >,
                            i32,
                        ),
                        i32,
                        2usize,
                    >("proc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "proc", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (z, r))? };
        Ok(__cordl_ret.into())
    }
    pub fn reset(
        &mut self,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
        c: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i64>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "reset", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (z, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_dictionary(
        &mut self,
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("set_dictionary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_dictionary", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (d, start, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn sync_point(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("sync_point")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "sync_point", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
