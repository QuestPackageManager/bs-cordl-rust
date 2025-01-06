#[cfg(feature = "UnityEngine+LightmapSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapSettings {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+LightmapSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LightmapSettings => "UnityEngine"
    ."LightmapSettings"
);
#[cfg(feature = "UnityEngine+LightmapSettings")]
impl std::ops::Deref for crate::UnityEngine::LightmapSettings {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightmapSettings")]
impl std::ops::DerefMut for crate::UnityEngine::LightmapSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LightmapSettings")]
impl crate::UnityEngine::LightmapSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reset", ())?;
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
    pub fn get_bakedColorSpace() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ColorSpace,
    > {
        let __cordl_ret: crate::UnityEngine::ColorSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bakedColorSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightProbes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::LightProbes>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::LightProbes> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_lightProbes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmaps() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::LightmapData>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::LightmapData>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_lightmaps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapsMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::LightmapsMode,
    > {
        let __cordl_ret: crate::UnityEngine::LightmapsMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_lightmapsMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapsModeLegacy() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::LightmapsModeLegacy,
    > {
        let __cordl_ret: crate::UnityEngine::LightmapsModeLegacy = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_lightmapsModeLegacy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bakedColorSpace(
        value: crate::UnityEngine::ColorSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_bakedColorSpace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lightProbes(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::LightProbes>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_lightProbes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lightmaps(
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::LightmapData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_lightmaps", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lightmapsMode(
        value: crate::UnityEngine::LightmapsMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_lightmapsMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lightmapsModeLegacy(
        value: crate::UnityEngine::LightmapsModeLegacy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_lightmapsModeLegacy", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+LightmapSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::LightmapSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
