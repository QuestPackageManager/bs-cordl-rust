#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Inflate")]
#[repr(C)]
#[derive(Debug)]
pub struct Inflate {
    __cordl_parent: crate::System::Object,
    pub mode: i32,
    pub method: i32,
    pub was: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
    pub need: i64,
    pub marker: i32,
    pub nowrap: i32,
    pub wbits: i32,
    pub blocks: *mut crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Inflate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::Inflate =>
    "Org.BouncyCastle.Utilities.Zlib"."Inflate"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Inflate")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::Inflate {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Inflate")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::Inflate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Inflate")]
impl crate::Org::BouncyCastle::Utilities::Zlib::Inflate {
    pub const BAD: i32 = 13i32;
    pub const BLOCKS: i32 = 7i32;
    pub const CHECK1: i32 = 11i32;
    pub const CHECK2: i32 = 10i32;
    pub const CHECK3: i32 = 9i32;
    pub const CHECK4: i32 = 8i32;
    pub const DICT0: i32 = 6i32;
    pub const DICT1: i32 = 5i32;
    pub const DICT2: i32 = 4i32;
    pub const DICT3: i32 = 3i32;
    pub const DICT4: i32 = 2i32;
    pub const DONE: i32 = 12i32;
    pub const FLAG: i32 = 1i32;
    pub const MAX_WBITS: i32 = 15i32;
    pub const METHOD: i32 = 0i32;
    pub const PRESET_DICT: i32 = 32i32;
    pub const Z_BUF_ERROR: i32 = -5i32;
    pub const Z_DATA_ERROR: i32 = -3i32;
    pub const Z_DEFLATED: i32 = 8i32;
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
    pub fn inflate(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        f: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflate", (z, f))?;
        Ok(__cordl_ret)
    }
    pub fn inflateEnd(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateEnd", (z))?;
        Ok(__cordl_ret)
    }
    pub fn inflateInit(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateInit", (z, w))?;
        Ok(__cordl_ret)
    }
    pub fn inflateReset(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateReset", (z))?;
        Ok(__cordl_ret)
    }
    pub fn inflateSetDictionary(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
        dictionary: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        dictLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("inflateSetDictionary", (z, dictionary, dictLength))?;
        Ok(__cordl_ret)
    }
    pub fn inflateSync(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateSync", (z))?;
        Ok(__cordl_ret)
    }
    pub fn inflateSyncPoint(
        &mut self,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("inflateSyncPoint", (z))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+Inflate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::Inflate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
