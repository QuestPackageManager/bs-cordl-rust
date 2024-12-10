#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfCodes")]
#[repr(C)]
#[derive(Debug)]
pub struct InfCodes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mode: i32,
    pub len: i32,
    pub tree: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub tree_index: i32,
    pub need: i32,
    pub lit: i32,
    pub get: i32,
    pub dist: i32,
    pub lbits: u8,
    pub dbits: u8,
    pub ltree: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub ltree_index: i32,
    pub dtree: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub dtree_index: i32,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfCodes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::InfCodes =>
    "Org.BouncyCastle.Utilities.Zlib"."InfCodes"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfCodes")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::InfCodes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfCodes")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::InfCodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfCodes")]
impl crate::Org::BouncyCastle::Utilities::Zlib::InfCodes {
    pub const BADCODE: i32 = 9i32;
    pub const COPY: i32 = 5i32;
    pub const DIST: i32 = 3i32;
    pub const DISTEXT: i32 = 4i32;
    pub const END: i32 = 8i32;
    pub const LEN: i32 = 1i32;
    pub const LENEXT: i32 = 2i32;
    pub const LIT: i32 = 6i32;
    pub const START: i32 = 0i32;
    pub const WASH: i32 = 7i32;
    pub const Z_BUF_ERROR: i32 = -5i32;
    pub const Z_DATA_ERROR: i32 = -3i32;
    pub const Z_ERRNO: i32 = -1i32;
    pub const Z_MEM_ERROR: i32 = -4i32;
    pub const Z_NEED_DICT: i32 = 2i32;
    pub const Z_OK: i32 = 0i32;
    pub const Z_STREAM_END: i32 = 1i32;
    pub const Z_STREAM_ERROR: i32 = -2i32;
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
    pub fn free(
        &mut self,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("free", (z))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflate_fast(
        &mut self,
        bl: i32,
        bd: i32,
        tl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        tl_index: i32,
        td: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        td_index: i32,
        s: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks,
        >,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("inflate_fast", (bl, bd, tl, tl_index, td, td_index, s, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn init(
        &mut self,
        bl: i32,
        bd: i32,
        tl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        tl_index: i32,
        td: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        td_index: i32,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("init", (bl, bd, tl, tl_index, td, td_index, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn proc(
        &mut self,
        s: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Zlib::InfBlocks,
        >,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
        r: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("proc", (s, z, r))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfCodes")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::InfCodes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
