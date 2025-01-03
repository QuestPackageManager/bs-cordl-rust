#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
#[repr(C)]
#[derive(Debug)]
pub struct Nat {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Raw::Nat =>
    "Org.BouncyCastle.Math.Raw"."Nat"
);
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Raw::Nat {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Raw::Nat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl crate::Org::BouncyCastle::Math::Raw::Nat {
    pub const M: u64 = 4294967295u64;
    pub fn Add(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (len, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add33At_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add33At", (len, x, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add33At_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add33At", (len, x, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add33To_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add33To", (len, x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add33To_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add33To", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBothTo_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddBothTo", (len, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBothTo_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddBothTo", (len, x, xOff, y, yOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordAt_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDWordAt", (len, x, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordAt_i32_u64_Il2CppArray_i32_0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDWordAt", (len, x, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordTo_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDWordTo", (len, x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordTo_i32_u64_Il2CppArray0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDWordTo", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToEachOther(
        len: i32,
        u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        uOff: i32,
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        vOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddToEachOther", (len, u, uOff, v, vOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTo_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTo", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTo_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTo", (len, x, xOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTo_i32_Il2CppArray_i32_u32_2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        cIn: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTo", (len, x, xOff, z, zOff, cIn))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddWordAt_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddWordAt", (len, x, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddWordAt_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddWordAt", (len, x, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddWordTo_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddWordTo", (len, x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddWordTo_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddWordTo", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn CAdd(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CAdd", (len, mask, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn CMov_i32_i32_Il2CppArray_i32_Il2CppArray_i32_0(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CMov", (len, mask, x, xOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn CMov_i32_i32_Il2CppArray_i32_Il2CppArray_i32_1(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CMov", (len, mask, x, xOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn CSub_Il2CppArray0(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CSub", (len, mask, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn CSub_i32_i32_Il2CppArray_i32_1(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CSub", (len, mask, x, xOff, y, yOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (len, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (len, x, xOff, y, yOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy64_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy64", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy64_i32_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u64>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Copy64", (len, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy64_i32_Il2CppArray_i32_2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy64", (len, x, xOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Copy", (len, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_Il2CppArray_i32_2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (len, x, xOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", (len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create64(
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u64>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create64", (len))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecAt_i32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecAt", (len, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecAt_i32_Il2CppArray_i32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecAt", (len, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dec_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dec", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dec_i32_Il2CppArray0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dec", (len, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Eq(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Eq", (len, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger(
        bits: i32,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBigInteger", (bits, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger64(
        bits: i32,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u64>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBigInteger64", (bits, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBit(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bit: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBit", (x, bit))?;
        Ok(__cordl_ret.into())
    }
    pub fn Gte(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Gte", (len, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncAt_i32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IncAt", (len, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncAt_i32_Il2CppArray_i32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IncAt", (len, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inc_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inc", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inc_i32_Il2CppArray0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inc", (len, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOne(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOne", (len, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsZero(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsZero", (len, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThan_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LessThan", (len, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThan_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LessThan", (len, x, xOff, y, yOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mul31BothAdd(
        len: i32,
        a: u32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        b: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mul31BothAdd", (len, a, x, b, y, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn MulAddTo_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MulAddTo", (len, x, y, zz))?;
        Ok(__cordl_ret.into())
    }
    pub fn MulAddTo_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MulAddTo", (len, x, xOff, y, yOff, zz, zzOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn MulWordAddTo(
        len: i32,
        x: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MulWordAddTo", (len, x, y, yOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn MulWordDwordAddAt(
        len: i32,
        x: u32,
        y: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MulWordDwordAddAt", (len, x, y, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn MulWord_Il2CppArray0(
        len: i32,
        x: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MulWord", (len, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn MulWord_i32_Il2CppArray_i32_1(
        len: i32,
        x: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MulWord", (len, x, y, yOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mul_Il2CppArray_i32_i32_i32_i32_Il2CppArray_i32_2(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        xLen: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        yLen: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mul", (x, xOff, xLen, y, yOff, yLen, zz, zzOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mul_i32_Il2CppArray_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mul", (len, x, y, zz))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mul_i32_Il2CppArray_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mul", (len, x, xOff, y, yOff, zz, zzOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShiftDownBit_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBit", (len, z, zOff, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBit_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBit", (len, x, xOff, c, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBit_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBit", (len, z, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBit_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBit", (len, x, c, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBits", (len, z, zOff, bits, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBits", (len, x, xOff, bits, c, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBits", (len, z, bits, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownBits", (len, x, bits, c, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownWord(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftDownWord", (len, z, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit64(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        xOff: i32,
        c: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBit64", (len, x, xOff, c, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBit", (len, z, zOff, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBit", (len, x, xOff, c, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBit", (len, z, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBit", (len, x, c, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits64_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        xOff: i32,
        bits: i32,
        c: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBits64", (len, x, xOff, bits, c, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits64_i32_Il2CppArray_i32_i32_u64_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
        bits: i32,
        c: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBits64", (len, z, zOff, bits, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBits", (len, z, zOff, bits, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBits", (len, x, xOff, bits, c, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBits", (len, z, bits, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftUpBits", (len, x, bits, c, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAddTo_Il2CppArray0(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SquareWordAddTo", (x, xPos, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAddTo_i32_Il2CppArray_i32_1(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SquareWordAddTo", (x, xOff, xPos, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAdd_Il2CppArray0(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SquareWordAdd", (x, xPos, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAdd_i32_Il2CppArray_i32_1(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SquareWordAdd", (x, xOff, xPos, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Square_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Square", (len, x, zz))?;
        Ok(__cordl_ret.into())
    }
    pub fn Square_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Square", (len, x, xOff, zz, zzOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sub33At_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sub33At", (len, x, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sub33At_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sub33At", (len, x, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sub33From_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sub33From", (len, x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sub33From_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sub33From", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubBothFrom_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubBothFrom", (len, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubBothFrom_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubBothFrom", (len, x, xOff, y, yOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordAt_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubDWordAt", (len, x, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordAt_i32_u64_Il2CppArray_i32_0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubDWordAt", (len, x, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordFrom_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubDWordFrom", (len, x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordFrom_i32_u64_Il2CppArray0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubDWordFrom", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubFrom_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubFrom", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubFrom_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubFrom", (len, x, xOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubWordAt_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubWordAt", (len, x, z, zOff, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubWordAt_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubWordAt", (len, x, z, zPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubWordFrom_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubWordFrom", (len, x, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubWordFrom_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubWordFrom", (len, x, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sub_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sub", (len, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sub_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sub", (len, x, xOff, y, yOff, z, zOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBigInteger(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBigInteger", (len, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Zero(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Zero", (len, z))?;
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
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Math::Raw::Nat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
