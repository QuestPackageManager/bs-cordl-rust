#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractFpCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractFpCurve {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::ECCurve,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractFpCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::AbstractFpCurve =>
    "Org.BouncyCastle.Math.EC"."AbstractFpCurve"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractFpCurve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::AbstractFpCurve {
    type Target = crate::Org::BouncyCastle::Math::EC::ECCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractFpCurve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::AbstractFpCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractFpCurve")]
impl crate::Org::BouncyCastle::Math::EC::AbstractFpCurve {
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
    pub fn _ctor(
        &mut self,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q))?;
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
    pub fn New(
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+AbstractFpCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::AbstractFpCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
