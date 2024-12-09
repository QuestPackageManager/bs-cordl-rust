#[cfg(feature = "Org+BouncyCastle+Math+Raw+Interleave")]
#[repr(C)]
#[derive(Debug)]
pub struct Interleave {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Interleave")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Raw::Interleave =>
    "Org.BouncyCastle.Math.Raw"."Interleave"
);
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Interleave")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Raw::Interleave {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Interleave")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Raw::Interleave {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Interleave")]
impl crate::Org::BouncyCastle::Math::Raw::Interleave {
    pub const M32: u64 = 1431655765u64;
    pub const M64: u64 = 6148914691236517205u64;
    pub const M64R: u64 = 12297829382473034410u64;
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
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Interleave")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::Raw::Interleave {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
