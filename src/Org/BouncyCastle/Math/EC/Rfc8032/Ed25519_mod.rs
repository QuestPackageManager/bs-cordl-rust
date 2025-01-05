#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    pub const L0: i32 = -50998291i32;
    pub const L1: i32 = 19280294i32;
    pub const L2: i32 = 127719000i32;
    pub const L3: i32 = -6428113i32;
    pub const L4: i32 = 5343i32;
    pub const M28L: i64 = 268435455i64;
    pub const M32L: i64 = 4294967295i64;
    pub const PointBytes: i32 = 32i32;
    pub const PrecompBlocks: i32 = 8i32;
    pub const PrecompMask: i32 = 7i32;
    pub const PrecompPoints: i32 = 8i32;
    pub const PrecompSpacing: i32 = 8i32;
    pub const PrecompTeeth: i32 = 4i32;
    pub const ScalarBytes: i32 = 32i32;
    pub const ScalarUints: i32 = 8i32;
    pub const WnafWidthBase: i32 = 7i32;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+Algorithm")]
    pub type Algorithm = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_Algorithm;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
    pub type PointAccum = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
    pub type PointExt = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
    pub type PointPrecomp = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp;
    pub fn CalculateS(
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateS", (r, k, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckContextVar(
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phflag: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckContextVar", (ctx, phflag))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckPointVar(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckPointVar", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckPoint_Il2CppArray1(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckPoint", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckPoint_Il2CppArray_Il2CppArray0(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckPoint", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckScalarVar(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckScalarVar", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDigest() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateDigest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePrehash() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreatePrehash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode24(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode24", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode32_Il2CppArray_i32_0(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode32", (bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode32_Il2CppArray_i32_i32_1(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bsOff: i32,
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        nOff: i32,
        nLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode32", (bs, bsOff, n, nOff, nLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodePointVar(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pOff: i32,
        negate: bool,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodePointVar", (p, pOff, negate, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeScalar(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodeScalar", (k, kOff, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dom2(
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        phflag: u8,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dom2", (d, phflag, ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode24(
        n: u32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Encode24", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode32(
        n: u32,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Encode32", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode56(
        n: u64,
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Encode56", (n, bs, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodePoint(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodePoint", (p, r, rOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePrivateKey(
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GeneratePrivateKey", (random, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePublicKey(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GeneratePublicKey", (sk, skOff, pk, pkOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWnafVar(
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        width: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWnafVar", (n, width))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSign_IDigest_Il2CppArray_Il2CppArray_i32_Il2CppArray_u8_Il2CppArray_i32_Il2CppArray_i32_0(
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        h: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phflag: u8,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ImplSign",
                (d, h, s, pk, pkOff, ctx, phflag, m, mOff, mLen, sig, sigOff),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSign_Il2CppArray_i32_i32_Il2CppArray_u8_Il2CppArray_i32_Il2CppArray_i32_2(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phflag: u8,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ImplSign",
                (sk, skOff, pk, pkOff, ctx, phflag, m, mOff, mLen, sig, sigOff),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSign_Il2CppArray_i32_u8_Il2CppArray_i32_i32_Il2CppArray1(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phflag: u8,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplSign", (sk, skOff, ctx, phflag, m, mOff, mLen, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplVerify(
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phflag: u8,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplVerify", (sig, sigOff, pk, pkOff, ctx, phflag, m, mOff, mLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PointAddPrecomp(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp,
        >,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointAddPrecomp", (p, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointAddVar_Ed25519_PointAccum0(
        negate: bool,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointAddVar", (negate, p, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointAddVar_Ed25519_PointExt_Ed25519_PointExt1(
        negate: bool,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
        q: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointAddVar", (negate, p, q, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointCopy_Ed25519_PointAccum0(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("PointCopy", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointCopy_Ed25519_PointExt1(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("PointCopy", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointDouble(
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointDouble", (r))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointExtendXY_Ed25519_PointAccum0(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointExtendXY", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointExtendXY_Ed25519_PointExt1(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointExtendXY", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointLookup(
        block: i32,
        index: i32,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointLookup", (block, index, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointPrecompVar(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointPrecompVar", (p, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointSetNeutral_Ed25519_PointAccum0(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointSetNeutral", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointSetNeutral_Ed25519_PointExt1(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointSetNeutral", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn Precompute() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Precompute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PruneScalar(
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        nOff: i32,
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PruneScalar", (n, nOff, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReduceScalar(
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ReduceScalar", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScalarMultBase(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScalarMultBase", (k, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScalarMultBaseEncoded(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScalarMultBaseEncoded", (k, r, rOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScalarMultBaseYZ(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScalarMultBaseYZ", (k, kOff, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScalarMultStrausVar(
        nb: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        np: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt,
        >,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScalarMultStrausVar", (nb, np, p, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn SignPrehash_IDigest_Il2CppArray_i32_2(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SignPrehash", (sk, skOff, ctx, ph, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SignPrehash_Il2CppArray_i32_Il2CppArray_i32_0(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phOff: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SignPrehash", (sk, skOff, ctx, ph, phOff, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SignPrehash_i32_Il2CppArray_IDigest_Il2CppArray_i32_3(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SignPrehash", (sk, skOff, pk, pkOff, ctx, ph, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SignPrehash_i32_Il2CppArray_Il2CppArray_i32_Il2CppArray_i32_1(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phOff: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SignPrehash", (sk, skOff, pk, pkOff, ctx, ph, phOff, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign_Il2CppArray_i32_i32_Il2CppArray_i32_2(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (sk, skOff, ctx, m, mOff, mLen, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign_i32_Il2CppArray_Il2CppArray_i32_i32_Il2CppArray_i32_3(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (sk, skOff, pk, pkOff, ctx, m, mOff, mLen, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign_i32_Il2CppArray_i32_i32_Il2CppArray_i32_1(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (sk, skOff, pk, pkOff, m, mOff, mLen, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign_i32_i32_Il2CppArray_i32_0(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (sk, skOff, m, mOff, mLen, sig, sigOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyPrehash_IDigest1(
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyPrehash", (sig, sigOff, pk, pkOff, ctx, ph))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyPrehash_Il2CppArray_i32_0(
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        phOff: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyPrehash", (sig, sigOff, pk, pkOff, ctx, ph, phOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Verify_Il2CppArray_i32_1(
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Verify", (sig, sigOff, pk, pkOff, ctx, m, mOff, mLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn Verify_i32_0(
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        m: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        mOff: i32,
        mLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Verify", (sig, sigOff, pk, pkOff, m, mOff, mLen))?;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+Algorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ed25519_Algorithm {
    #[default]
    Ed25519 = 0i32,
    Ed25519ctx = 1i32,
    Ed25519ph = 2i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+Algorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_Algorithm =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/Algorithm"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519_PointAccum {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/PointAccum"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
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
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519_PointExt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/PointExt"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
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
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519_PointPrecomp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ypx_h: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub ymx_h: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub xyd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/PointPrecomp"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
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
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
