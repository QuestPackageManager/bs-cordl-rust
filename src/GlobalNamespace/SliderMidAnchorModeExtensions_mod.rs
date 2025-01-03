#[cfg(feature = "SliderMidAnchorModeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMidAnchorModeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderMidAnchorModeExtensions
    => ""."SliderMidAnchorModeExtensions"
);
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::SliderMidAnchorModeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderMidAnchorModeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl crate::GlobalNamespace::SliderMidAnchorModeExtensions {
    pub fn OppositeDirection(
        sliderMidAnchorMode: crate::GlobalNamespace::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderMidAnchorMode> {
        let __cordl_ret: crate::GlobalNamespace::SliderMidAnchorMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OppositeDirection", (sliderMidAnchorMode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderMidAnchorModeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
