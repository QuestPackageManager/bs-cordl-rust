#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+StaticTree")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticTree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub static_tree: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub extra_bits: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub extra_base: i32,
    pub elems: i32,
    pub max_length: i32,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+StaticTree")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Utilities::Zlib::StaticTree
    => "Org.BouncyCastle.Utilities.Zlib"."StaticTree"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+StaticTree")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Zlib::StaticTree {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+StaticTree")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Zlib::StaticTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+StaticTree")]
impl crate::Org::BouncyCastle::Utilities::Zlib::StaticTree {
    pub const BL_CODES: i32 = 19i32;
    pub const D_CODES: i32 = 30i32;
    pub const LENGTH_CODES: i32 = 29i32;
    pub const LITERALS: i32 = 256i32;
    pub const L_CODES: i32 = 286i32;
    pub const MAX_BITS: i32 = 15i32;
    pub const MAX_BL_BITS: i32 = 7i32;
    pub fn New(
        static_tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        extra_bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        extra_base: i32,
        elems: i32,
        max_length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (static_tree, extra_bits, extra_base, elems, max_length),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        static_tree: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        extra_bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        extra_base: i32,
        elems: i32,
        max_length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (static_tree, extra_bits, extra_base, elems, max_length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Zlib+StaticTree")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Zlib::StaticTree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
