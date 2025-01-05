#[cfg(feature = "FloatFxBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxBaseData {
    __cordl_parent: crate::GlobalNamespace::FxBaseData,
    pub value: f32,
    pub easeType: crate::GlobalNamespace::EaseType,
}
#[cfg(feature = "FloatFxBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatFxBaseData => ""
    ."FloatFxBaseData"
);
#[cfg(feature = "FloatFxBaseData")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxBaseData {
    type Target = crate::GlobalNamespace::FxBaseData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBaseData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBaseData")]
impl crate::GlobalNamespace::FloatFxBaseData {
    pub fn New(
        beat: f32,
        usePreviousEventValue: bool,
        value: f32,
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, usePreviousEventValue, value, easeType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventValue: bool,
        value: f32,
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, usePreviousEventValue, value, easeType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FloatFxBaseData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FloatFxBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
