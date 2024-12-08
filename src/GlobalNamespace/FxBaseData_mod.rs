#[cfg(feature = "FxBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct FxBaseData {
    __cordl_parent: crate::System::Object,
    pub beat: f32,
    pub usePreviousEventValue: bool,
}
#[cfg(feature = "FxBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FxBaseData => ""."FxBaseData"
);
#[cfg(feature = "FxBaseData")]
impl std::ops::Deref for FxBaseData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FxBaseData")]
impl std::ops::DerefMut for FxBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FxBaseData")]
impl FxBaseData {
    pub fn New(
        beat: f32,
        usePreviousEventValue: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, usePreviousEventValue))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, usePreviousEventValue))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FxBaseData")]
impl quest_hook::libil2cpp::ObjectType for FxBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}