#[cfg(feature = "Org+BouncyCastle+Crypto+Utilities+Pack")]
#[repr(C)]
#[derive(Debug)]
pub struct Pack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Utilities+Pack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Utilities::Pack =>
    "Org.BouncyCastle.Crypto.Utilities"."Pack"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Utilities+Pack")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Utilities::Pack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Utilities+Pack")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Utilities::Pack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Utilities+Pack")]
impl crate::Org::BouncyCastle::Crypto::Utilities::Pack {
    pub fn BE_To_UInt16_Il2CppArray0(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt16", (bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt16_i32_1(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt16", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt32_Il2CppArray0(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt32", (bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt32_i32_1(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt32", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt32_i32_Il2CppArray2(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt32", (bs, off, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt32_i32_Il2CppArray_i32_i32_3(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bsOff: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        nsOff: i32,
        nsLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt32", (bs, bsOff, ns, nsOff, nsLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt64_Il2CppArray0(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt64", (bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt64_i32_1(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt64", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt64_i32_Il2CppArray2(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt64", (bs, off, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn BE_To_UInt64_i32_Il2CppArray_i32_i32_3(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bsOff: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        nsOff: i32,
        nsLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BE_To_UInt64", (bs, bsOff, ns, nsOff, nsLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt16_Il2CppArray0(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt16", (bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt16_i32_1(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt16", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt32_Il2CppArray0(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt32", (bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt32_i32_1(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt32", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt32_i32_Il2CppArray2(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt32", (bs, off, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt32_i32_Il2CppArray_i32_i32_3(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bOff: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        nOff: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt32", (bs, bOff, ns, nOff, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt32_i32_i32_4(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt32", (bs, off, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt64_Il2CppArray0(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt64", (bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt64_i32_1(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt64", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt64_i32_Il2CppArray2(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt64", (bs, off, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn LE_To_UInt64_i32_Il2CppArray_i32_i32_3(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bsOff: i32,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        nsOff: i32,
        nsLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LE_To_UInt64", (bs, bsOff, ns, nsOff, nsLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UInt16_To_BE_i32_1(
        n: u16,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt16_To_BE", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt16_To_BE_u16_Il2CppArray0(
        n: u16,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt16_To_BE", (n, bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt16_To_LE_i32_1(
        n: u16,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt16_To_LE", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt16_To_LE_u16_Il2CppArray0(
        n: u16,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt16_To_LE", (n, bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_BE_Il2CppArray3(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt32_To_BE", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_BE_Il2CppArray_Il2CppArray_i32_4(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32_To_BE", (ns, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_BE_Il2CppArray_i32_i32_Il2CppArray_i32_5(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        nsOff: i32,
        nsLen: i32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bsOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32_To_BE", (ns, nsOff, nsLen, bs, bsOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_BE_u32_0(
        n: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt32_To_BE", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_BE_u32_Il2CppArray1(
        n: u32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32_To_BE", (n, bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_BE_u32_Il2CppArray_i32_2(
        n: u32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32_To_BE", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_LE_Il2CppArray3(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt32_To_LE", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_LE_Il2CppArray_Il2CppArray_i32_4(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32_To_LE", (ns, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_LE_u32_0(
        n: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt32_To_LE", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_LE_u32_Il2CppArray1(
        n: u32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32_To_LE", (n, bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32_To_LE_u32_Il2CppArray_i32_2(
        n: u32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32_To_LE", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_BE_Il2CppArray3(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt64_To_BE", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_BE_Il2CppArray_Il2CppArray_i32_4(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_BE", (ns, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_BE_Il2CppArray_i32_i32_Il2CppArray_i32_5(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        nsOff: i32,
        nsLen: i32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bsOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_BE", (ns, nsOff, nsLen, bs, bsOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_BE_u64_0(
        n: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt64_To_BE", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_BE_u64_Il2CppArray1(
        n: u64,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_BE", (n, bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_BE_u64_Il2CppArray_i32_2(
        n: u64,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_BE", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_LE_Il2CppArray3(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt64_To_LE", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_LE_Il2CppArray_Il2CppArray_i32_4(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_LE", (ns, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_LE_Il2CppArray_i32_i32_Il2CppArray_i32_5(
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        nsOff: i32,
        nsLen: i32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bsOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_LE", (ns, nsOff, nsLen, bs, bsOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_LE_u64_0(
        n: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UInt64_To_LE", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_LE_u64_Il2CppArray1(
        n: u64,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_LE", (n, bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64_To_LE_u64_Il2CppArray_i32_2(
        n: u64,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64_To_LE", (n, bs, off))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Utilities+Pack")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Utilities::Pack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
