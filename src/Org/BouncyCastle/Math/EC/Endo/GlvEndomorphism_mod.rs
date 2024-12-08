#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvEndomorphism")]
#[repr(C)]
#[derive(Debug)]
pub struct GlvEndomorphism {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvEndomorphism")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism =>
    "Org.BouncyCastle.Math.EC.Endo"."GlvEndomorphism"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvEndomorphism")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvEndomorphism")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvEndomorphism")]
impl crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism {
    pub fn DecomposeScalar(
        &mut self,
        k: *mut crate::Org::BouncyCastle::Math::BigInteger,
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
        > = __cordl_object.invoke("DecomposeScalar", (k))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvEndomorphism")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
