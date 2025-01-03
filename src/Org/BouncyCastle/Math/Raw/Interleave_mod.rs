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
    pub fn Expand16to32(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Expand16to32", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Expand32to64(x: u32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Expand32to64", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Expand64To128(
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Expand64To128", (x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Expand64To128Rev(
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Expand64To128Rev", (x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Expand8to16(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Expand8to16", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Shuffle2(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shuffle2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Shuffle_u32_0(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shuffle", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Shuffle_u64_1(x: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shuffle", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unshuffle2(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unshuffle2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unshuffle_u32_0(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unshuffle", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unshuffle_u64_1(x: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unshuffle", (x))?;
        Ok(__cordl_ret.into())
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
