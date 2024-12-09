#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+ZTauElement")]
#[repr(C)]
#[derive(Debug)]
pub struct ZTauElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub u: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub v: *mut crate::Org::BouncyCastle::Math::BigInteger,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+ZTauElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::Abc::ZTauElement =>
    "Org.BouncyCastle.Math.EC.Abc"."ZTauElement"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+ZTauElement")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+ZTauElement")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+ZTauElement")]
impl crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement {
    pub fn New(
        u: *mut crate::Org::BouncyCastle::Math::BigInteger,
        v: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (u, v))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        u: *mut crate::Org::BouncyCastle::Math::BigInteger,
        v: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (u, v))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+ZTauElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Abc::ZTauElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
