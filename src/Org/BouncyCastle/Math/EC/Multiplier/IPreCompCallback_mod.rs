#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+IPreCompCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct IPreCompCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+IPreCompCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback =>
    "Org.BouncyCastle.Math.EC.Multiplier"."IPreCompCallback"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+IPreCompCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+IPreCompCallback")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+IPreCompCallback")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
    pub fn Precompute(
        &mut self,
        existing: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo = __cordl_object
            .invoke("Precompute", (existing))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+IPreCompCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
