#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+Tnaf")]
#[repr(C)]
#[derive(Debug)]
pub struct Tnaf {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+Tnaf")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Abc::Tnaf {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Abc";
    const CLASS_NAME: &'static str = "Tnaf";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+Tnaf")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Abc::Tnaf {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+Tnaf")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Abc::Tnaf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+Tnaf")]
impl crate::Org::BouncyCastle::Math::EC::Abc::Tnaf {
    pub const Pow2Width: i8 = 16i8;
    pub const Width: i8 = 4i8;
    pub fn ApproximateDivisionByN(
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        s: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        vm: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        a: i8,
        m: i32,
        c: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApproximateDivisionByN", (k, s, vm, a, m, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLucas(
        mu: i8,
        k: i32,
        doV: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLucas", (mu, k, doV))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMu_AbstractF2mCurve0(
        curve: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve,
        >,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMu", (curve))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMu_ECFieldElement1(
        curveA: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        >,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMu", (curveA))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMu_i32_2(curveA: i32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMu", (curveA))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreComp(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        a: i8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetPreComp", (p, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShiftsForCofactor(
        h: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShiftsForCofactor", (h))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSi_AbstractF2mCurve0(
        curve: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetSi", (curve))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSi_i32_i32_BigInteger1(
        fieldSize: i32,
        curveA: i32,
        cofactor: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSi", (fieldSize, curveA, cofactor))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTw(
        mu: i8,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetTw", (mu, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyFromTnaf(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyFromTnaf", (p, u))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyRTnaf(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyRTnaf", (p, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyTnaf(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
        lambda: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyTnaf", (p, lambda))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Norm_SimpleBigDecimal_SimpleBigDecimal1(
        mu: i8,
        u: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        >,
        v: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Norm", (mu, u, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn Norm_ZTauElement0(
        mu: i8,
        lambda: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Norm", (mu, lambda))?;
        Ok(__cordl_ret.into())
    }
    pub fn PartModReduction(
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        m: i32,
        a: i8,
        s: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        >,
        mu: i8,
        c: i8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PartModReduction", (k, m, a, s, mu, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round(
        lambda0: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        >,
        lambda1: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
        >,
        mu: i8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (lambda0, lambda1, mu))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tau(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::AbstractF2mPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Tau", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn TauAdicNaf(
        mu: i8,
        lambda: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TauAdicNaf", (mu, lambda))?;
        Ok(__cordl_ret.into())
    }
    pub fn TauAdicWNaf(
        mu: i8,
        lambda: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
        >,
        width: i8,
        pow2w: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        tw: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        alpha: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TauAdicWNaf", (mu, lambda, width, pow2w, tw, alpha))?;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+Tnaf")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Abc::Tnaf {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
