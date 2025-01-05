#[cfg(feature = "LufsMetering+AudioJobHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioJobHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LufsMetering+AudioJobHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::AudioJobHelpers => "LufsMetering"
    ."AudioJobHelpers"
);
#[cfg(feature = "LufsMetering+AudioJobHelpers")]
impl std::ops::Deref for crate::LufsMetering::AudioJobHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LufsMetering+AudioJobHelpers")]
impl std::ops::DerefMut for crate::LufsMetering::AudioJobHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LufsMetering+AudioJobHelpers")]
impl crate::LufsMetering::AudioJobHelpers {
    pub fn GetHighPassDeManCoefficients(
        quality: f32,
        frequency: f32,
        rate: f32,
    ) -> quest_hook::libil2cpp::Result<crate::LufsMetering::FilterCoefficients> {
        let __cordl_ret: crate::LufsMetering::FilterCoefficients = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHighPassDeManCoefficients", (quality, frequency, rate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHighShelfDeManCoefficients(
        gain: f32,
        quality: f32,
        frequency: f32,
        rate: f32,
    ) -> quest_hook::libil2cpp::Result<crate::LufsMetering::FilterCoefficients> {
        let __cordl_ret: crate::LufsMetering::FilterCoefficients = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHighShelfDeManCoefficients", (gain, quality, frequency, rate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LufsMetering+AudioJobHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::LufsMetering::AudioJobHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
