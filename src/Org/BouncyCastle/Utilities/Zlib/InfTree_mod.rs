#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfTree")]
#[repr(C)]
#[derive(Debug)]
pub struct InfTree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hn: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub v: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub c: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub r: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub u: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub x: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfTree")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::InfTree =>
    "Org.BouncyCastle.Utilities.Zlib"."InfTree"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfTree")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::InfTree {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfTree")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::InfTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfTree")]
impl crate::Org::BouncyCastle::Utilities::Zlib::InfTree {
    pub const BMAX: i32 = 15i32;
    pub const MANY: i32 = 1440i32;
    pub const Z_BUF_ERROR: i32 = -5i32;
    pub const Z_DATA_ERROR: i32 = -3i32;
    pub const Z_ERRNO: i32 = -1i32;
    pub const Z_MEM_ERROR: i32 = -4i32;
    pub const Z_NEED_DICT: i32 = 2i32;
    pub const Z_OK: i32 = 0i32;
    pub const Z_STREAM_END: i32 = 1i32;
    pub const Z_STREAM_ERROR: i32 = -2i32;
    pub const Z_VERSION_ERROR: i32 = -6i32;
    pub const fixed_bd: i32 = 5i32;
    pub const fixed_bl: i32 = 9i32;
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
    pub fn huft_build(
        &mut self,
        b: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        bindex: i32,
        n: i32,
        s: i32,
        d: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        e: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        t: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        m: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        hp: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        hn: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        v: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("huft_build", (b, bindex, n, s, d, e, t, m, hp, hn, v))?;
        Ok(__cordl_ret)
    }
    pub fn inflate_trees_bits(
        &mut self,
        c: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        bb: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        tb: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        hp: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("inflate_trees_bits", (c, bb, tb, hp, z))?;
        Ok(__cordl_ret)
    }
    pub fn inflate_trees_dynamic(
        &mut self,
        nl: i32,
        nd: i32,
        c: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        bl: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        bd: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        tl: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        td: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        hp: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        z: *mut crate::Org::BouncyCastle::Utilities::Zlib::ZStream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("inflate_trees_dynamic", (nl, nd, c, bl, bd, tl, td, hp, z))?;
        Ok(__cordl_ret)
    }
    pub fn initWorkArea(
        &mut self,
        vsize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("initWorkArea", (vsize))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfTree")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::InfTree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
