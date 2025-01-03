#[cfg(feature = "SaberTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberTypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SaberTypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberTypeExtensions => ""
    ."SaberTypeExtensions"
);
#[cfg(feature = "SaberTypeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::SaberTypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberTypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl crate::GlobalNamespace::SaberTypeExtensions {
    pub fn MainSaber(
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SaberType> {
        let __cordl_ret: crate::GlobalNamespace::SaberType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MainSaber", (leftHanded))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesColorType(
        saberType: crate::GlobalNamespace::SaberType,
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesColorType", (saberType, colorType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Node(
        saberType: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::XRNode> {
        let __cordl_ret: crate::UnityEngine::XR::XRNode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Node", (saberType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToColorType(
        saberType: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        let __cordl_ret: crate::GlobalNamespace::ColorType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToColorType", (saberType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSaberType(
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SaberType> {
        let __cordl_ret: crate::GlobalNamespace::SaberType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSaberType", (colorType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
