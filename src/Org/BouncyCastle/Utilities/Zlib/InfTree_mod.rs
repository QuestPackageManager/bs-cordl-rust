#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+InfTree")]
#[repr(C)]
#[derive(Debug)]
pub struct InfTree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub c: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
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
    pub fn huft_build(
        &mut self,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bindex: i32,
        n: i32,
        s: i32,
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        e: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        hp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        hn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("huft_build", (b, bindex, n, s, d, e, t, m, hp, hn, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflate_trees_bits(
        &mut self,
        c: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bb: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        tb: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        hp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("inflate_trees_bits", (c, bb, tb, hp, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflate_trees_dynamic(
        &mut self,
        nl: i32,
        nd: i32,
        c: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        tl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        td: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        hp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("inflate_trees_dynamic", (nl, nd, c, bl, bd, tl, td, hp, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn inflate_trees_fixed(
        bl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        tl: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<i32>,
            >,
        >,
        td: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<i32>,
            >,
        >,
        z: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Zlib::ZStream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inflate_trees_fixed", (bl, bd, tl, td, z))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
