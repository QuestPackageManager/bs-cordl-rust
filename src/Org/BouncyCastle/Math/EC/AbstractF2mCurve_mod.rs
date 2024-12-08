#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractF2mCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractF2mCurve {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::ECCurve,
    pub si: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Math::BigInteger,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractF2mCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::AbstractF2mCurve =>
    "Org.BouncyCastle.Math.EC"."AbstractF2mCurve"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractF2mCurve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve {
    type Target = crate::Org::BouncyCastle::Math::EC::ECCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractF2mCurve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractF2mCurve")]
impl crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve {
    pub fn CreatePoint(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
        y: *mut crate::Org::BouncyCastle::Math::BigInteger,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("CreatePoint", (x, y, withCompression))?;
        Ok(__cordl_ret)
    }
    pub fn DecompressPoint(
        &mut self,
        yTilde: i32,
        X1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("DecompressPoint", (yTilde, X1))?;
        Ok(__cordl_ret)
    }
    pub fn GetSi(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("GetSi", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsValidFieldElement(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidFieldElement", (x))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k1, k2, k3))?;
        Ok(__cordl_object)
    }
    pub fn RandomFieldElement(
        &mut self,
        r: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("RandomFieldElement", (r))?;
        Ok(__cordl_ret)
    }
    pub fn RandomFieldElementMult(
        &mut self,
        r: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("RandomFieldElementMult", (r))?;
        Ok(__cordl_ret)
    }
    pub fn SolveQuadraticEquation(
        &mut self,
        beta: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("SolveQuadraticEquation", (beta))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k1, k2, k3))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsKoblitz(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsKoblitz", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractF2mCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}