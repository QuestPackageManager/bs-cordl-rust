#[cfg(feature = "UnityEngine+Random")]
#[repr(C)]
#[derive(Debug)]
pub struct Random {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Random")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Random => "UnityEngine"."Random"
);
#[cfg(feature = "UnityEngine+Random")]
impl std::ops::Deref for crate::UnityEngine::Random {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Random")]
impl std::ops::DerefMut for crate::UnityEngine::Random {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Random")]
impl crate::UnityEngine::Random {
    pub fn ColorHSV_0() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ColorHSV", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorHSV_f32_f32_f32_f32_f32_f32_f32_f32_1(
        hueMin: f32,
        hueMax: f32,
        saturationMin: f32,
        saturationMax: f32,
        valueMin: f32,
        valueMax: f32,
        alphaMin: f32,
        alphaMax: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ColorHSV",
                (
                    hueMin,
                    hueMax,
                    saturationMin,
                    saturationMax,
                    valueMin,
                    valueMax,
                    alphaMin,
                    alphaMax,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitState(
        seed: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitState", (seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn RandomRangeInt(
        minInclusive: i32,
        maxExclusive: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RandomRangeInt", (minInclusive, maxExclusive))?;
        Ok(__cordl_ret.into())
    }
    pub fn Range_f32_f32_0(
        minInclusive: f32,
        maxInclusive: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Range", (minInclusive, maxInclusive))?;
        Ok(__cordl_ret.into())
    }
    pub fn Range_i32_i32_1(
        minInclusive: i32,
        maxExclusive: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Range", (minInclusive, maxExclusive))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onUnitSphere() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector3,
    > {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_onUnitSphere", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onUnitSphere_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_onUnitSphere_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Quaternion,
    > {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_rotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_rotation_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Random")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Random {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
