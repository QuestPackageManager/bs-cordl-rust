#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapperUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils =>
    "UnityEngine.Experimental.GlobalIllumination"."LightmapperUtils"
);
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    pub fn ApplyColorTemperature(
        cct: crate::UnityEngine::Color,
        lightColor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyColorTemperature", (cct, lightColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractColorTemperature(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractColorTemperature", (l))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractIndirect(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractIndirect", (l))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractInnerCone(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractInnerCone", (l))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut1(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        dir: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::DirectionalLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract", (l, dir))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut2(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        point: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::PointLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract", (l, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut3(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        spot: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::SpotLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract", (l, spot))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut4(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        rect: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::RectangleLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract", (l, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut5(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        disc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::DiscLight,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract", (l, disc))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract_Light_ByRefMut6(
        l: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        cookie: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract", (l, cookie))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract_LightmapBakeType0(
        baketype: crate::UnityEngine::LightmapBakeType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::GlobalIllumination::LightMode,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::GlobalIllumination::LightMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract", (baketype))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
