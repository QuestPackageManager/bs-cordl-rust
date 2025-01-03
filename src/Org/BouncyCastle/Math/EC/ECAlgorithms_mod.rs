#[cfg(feature = "Org+BouncyCastle+Math+EC+ECAlgorithms")]
#[repr(C)]
#[derive(Debug)]
pub struct ECAlgorithms {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECAlgorithms")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::ECAlgorithms =>
    "Org.BouncyCastle.Math.EC"."ECAlgorithms"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECAlgorithms")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::ECAlgorithms {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECAlgorithms")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::ECAlgorithms {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECAlgorithms")]
impl crate::Org::BouncyCastle::Math::EC::ECAlgorithms {
    pub fn CleanPoint(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CleanPoint", (c, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplCheckResult(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplCheckResult", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplShamirsTrickFixedPoint(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        l: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplShamirsTrickFixedPoint", (p, k, q, l))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplShamirsTrickJsf(
        P: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        Q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        l: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplShamirsTrickJsf", (P, k, Q, l))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplShamirsTrickWNaf_ECEndomorphism_ECPoint_BigInteger_BigInteger1(
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
        P: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        l: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplShamirsTrickWNaf", (endomorphism, P, k, l))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplShamirsTrickWNaf_ECPoint_BigInteger_ECPoint_BigInteger0(
        P: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        Q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        l: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplShamirsTrickWNaf", (P, k, Q, l))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplShamirsTrickWNaf_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray2(
        preCompP: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        preCompNegP: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        wnafP: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        preCompQ: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        preCompNegQ: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        wnafQ: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ImplShamirsTrickWNaf",
                (preCompP, preCompNegP, wnafP, preCompQ, preCompNegQ, wnafQ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSumOfMultipliesGlv(
        ps: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        ks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
        glvEndomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplSumOfMultipliesGlv", (ps, ks, glvEndomorphism))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSumOfMultiplies_ECEndomorphism_Il2CppArray1(
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
        ps: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        ks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplSumOfMultiplies", (endomorphism, ps, ks))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSumOfMultiplies_Il2CppArray0(
        ps: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        ks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplSumOfMultiplies", (ps, ks))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSumOfMultiplies_Il2CppArray_Il2CppArray2(
        negs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
        infos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
            >,
        >,
        wnafs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplSumOfMultiplies", (negs, infos, wnafs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportPoint(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImportPoint", (c, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsF2mCurve(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsF2mCurve", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsF2mField(
        field: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsF2mField", (field))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFpCurve(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFpCurve", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFpField(
        field: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFpField", (field))?;
        Ok(__cordl_ret.into())
    }
    pub fn MontgomeryTrick_ECFieldElement1(
        zs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
            >,
        >,
        off: i32,
        len: i32,
        scale: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MontgomeryTrick", (zs, off, len, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn MontgomeryTrick_Il2CppArray_i32_i32_0(
        zs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
            >,
        >,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MontgomeryTrick", (zs, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReferenceMultiply(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReferenceMultiply", (p, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShamirsTrick(
        P: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        Q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        l: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShamirsTrick", (P, k, Q, l))?;
        Ok(__cordl_ret.into())
    }
    pub fn SumOfMultiplies(
        ps: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
            >,
        >,
        ks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SumOfMultiplies", (ps, ks))?;
        Ok(__cordl_ret.into())
    }
    pub fn SumOfTwoMultiplies(
        P: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        a: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        Q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        b: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SumOfTwoMultiplies", (P, a, Q, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidatePoint(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ValidatePoint", (p))?;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECAlgorithms")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::ECAlgorithms {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
