#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
#[repr(C)]
#[derive(Debug)]
pub struct InfBlocks {
    __cordl_parent: crate::System::Object,
    pub mode: i32,
    pub left: i32,
    pub table: i32,
    pub index: i32,
    pub blens: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub bb: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub tb: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub codes: *mut crate::Org::BouncyCastle::Utilities::Zlib::InfCodes,
    pub last: i32,
    pub bitk: i32,
    pub bitb: i32,
    pub hufts: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub window: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub end: i32,
    pub read: i32,
    pub write: i32,
    pub checkfn: *mut crate::System::Object,
    pub check: i64,
    pub inftree: *mut crate::Org::BouncyCastle::Utilities::Zlib::InfTree,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::InfBlocks =>
    "Org.BouncyCastle.Utilities.Zlib"."InfBlocks"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfBlocks")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        checkfn: *mut crate::System::Object,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (z, checkfn, w))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        checkfn: *mut crate::System::Object,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (z, checkfn, w))?;
        Ok(__cordl_ret)
    }
    pub fn free(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("free", (z))?;
        Ok(__cordl_ret)
    }
    pub fn inflate_flush(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflate_flush", (z, r))?;
        Ok(__cordl_ret)
    }
    pub fn proc(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("proc", (z, r))?;
        Ok(__cordl_ret)
    }
    pub fn reset(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        c: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("reset", (z, c))?;
        Ok(__cordl_ret)
    }
    pub fn set_dictionary(
        &mut self,
        d: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dictionary", (d, start, n))?;
        Ok(__cordl_ret)
    }
    pub fn sync_point(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("sync_point", ())?;
        Ok(__cordl_ret)
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
