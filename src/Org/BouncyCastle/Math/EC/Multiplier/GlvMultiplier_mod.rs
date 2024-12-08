#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+GlvMultiplier")]
#[repr(C)]
#[derive(Debug)]
pub struct GlvMultiplier {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::Multiplier::AbstractECMultiplier,
    pub curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
    pub glvEndomorphism: *mut crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+GlvMultiplier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::GlvMultiplier =>
    "Org.BouncyCastle.Math.EC.Multiplier"."GlvMultiplier"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+GlvMultiplier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Multiplier::GlvMultiplier {
    type Target = crate::Org::BouncyCastle::Math::EC::Multiplier::AbstractECMultiplier;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+GlvMultiplier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::GlvMultiplier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+GlvMultiplier")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::GlvMultiplier {
    pub fn MultiplyPositive(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        k: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("MultiplyPositive", (p, k))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        glvEndomorphism: *mut crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, glvEndomorphism))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        glvEndomorphism: *mut crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, glvEndomorphism))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+GlvMultiplier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::GlvMultiplier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
