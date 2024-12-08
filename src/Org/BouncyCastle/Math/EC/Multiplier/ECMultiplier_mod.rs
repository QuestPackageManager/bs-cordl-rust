#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ECMultiplier")]
#[repr(C)]
#[derive(Debug)]
pub struct ECMultiplier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ECMultiplier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier =>
    "Org.BouncyCastle.Math.EC.Multiplier"."ECMultiplier"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ECMultiplier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ECMultiplier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ECMultiplier")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier {
    pub fn Multiply(
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
            .invoke("Multiply", (p, k))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+ECMultiplier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
