#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed448 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Rfc8032";
    const CLASS_NAME: &'static str = "Ed448";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    pub const C_d: i32 = -39081i32;
    pub const L4_0: i32 = 43969588i32;
    pub const L4_1: i32 = 30366549i32;
    pub const L4_2: i32 = 163752818i32;
    pub const L4_3: i32 = 258169998i32;
    pub const L4_4: i32 = 96434764i32;
    pub const L4_5: i32 = 227822194i32;
    pub const L4_6: i32 = 149865618i32;
    pub const L4_7: i32 = 550336261i32;
    pub const L_0: i32 = 78101261i32;
    pub const L_1: i32 = 141809365i32;
    pub const L_2: i32 = 175155932i32;
    pub const L_3: i32 = 64542499i32;
    pub const L_4: i32 = 158326419i32;
    pub const L_5: i32 = 191173276i32;
    pub const L_6: i32 = 104575268i32;
    pub const L_7: i32 = 137584065i32;
    pub const M26UL: u64 = 67108863u64;
    pub const M28UL: u64 = 268435455u64;
    pub const PointBytes: i32 = 57i32;
    pub const PrecompBlocks: i32 = 5i32;
    pub const PrecompMask: i32 = 15i32;
    pub const PrecompPoints: i32 = 16i32;
    pub const PrecompSpacing: i32 = 18i32;
    pub const PrecompTeeth: i32 = 5i32;
    pub const ScalarBytes: i32 = 57i32;
    pub const ScalarUints: i32 = 14i32;
    pub const WnafWidthBase: i32 = 7i32;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
    pub type Algorithm = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
    pub type PointExt = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
    pub type PointPrecomp = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp;
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
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckContextVar", (ctx))?;
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
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckPoint", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckPoint_Il2CppArray_Il2CppArray0(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
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
    pub fn CreatePrehash() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IXof>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IXof,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreatePrehash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateXof() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IXof>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IXof,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateXof", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode16(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode16", (bs, off))?;
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
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
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
    pub fn Dom4(
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IXof>,
        x: u8,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dom4", (d, x, y))?;
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
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
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
    pub fn ImplSign_IXof_Il2CppArray_Il2CppArray_i32_Il2CppArray_u8_Il2CppArray_i32_Il2CppArray_i32_0(
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IXof>,
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
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp,
        >,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointAddPrecomp", (p, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointAddVar(
        negate: bool,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointAddVar", (negate, p, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointCopy(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("PointCopy", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointDouble(
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointDouble", (r))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointExtendXY(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
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
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointLookup", (block, index, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointPrecompVar(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointPrecompVar", (p, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointSetNeutral(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
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
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
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
    pub fn ScalarMultBaseXY(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScalarMultBaseXY", (k, kOff, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScalarMultStrausVar(
        nb: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        np: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
        r: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScalarMultStrausVar", (nb, np, p, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn SignPrehash_IXof_Il2CppArray_i32_2(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IXof>,
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
    pub fn SignPrehash_i32_Il2CppArray_IXof_Il2CppArray_i32_3(
        sk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        skOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IXof>,
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
    pub fn Sign_Il2CppArray_i32_i32_Il2CppArray0(
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
    pub fn Sign_i32_Il2CppArray_Il2CppArray_i32_Il2CppArray_i32_1(
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
    pub fn Verify(
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
    pub fn VerifyPrehash_IXof1(
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
        pk: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pkOff: i32,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ph: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IXof>,
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ed448_Algorithm {
    #[default]
    Ed448 = 0i32,
    Ed448ph = 1i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Rfc8032";
    const CLASS_NAME: &'static str = "Ed448/Algorithm";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed448_PointExt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Rfc8032";
    const CLASS_NAME: &'static str = "Ed448/PointExt";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed448_PointPrecomp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Rfc8032";
    const CLASS_NAME: &'static str = "Ed448/PointPrecomp";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
