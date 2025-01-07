#[cfg(feature = "LufsMetering+AudioJobHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioJobHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LufsMetering+AudioJobHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::LufsMetering::AudioJobHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LufsMetering";
    const CLASS_NAME: &'static str = "AudioJobHelpers";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
