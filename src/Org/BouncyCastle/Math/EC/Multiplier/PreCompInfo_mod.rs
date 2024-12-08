#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+PreCompInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PreCompInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+PreCompInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo =>
    "Org.BouncyCastle.Math.EC.Multiplier"."PreCompInfo"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+PreCompInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+PreCompInfo")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+PreCompInfo")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+PreCompInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
