#[cfg(feature = "IntFxBaseData")]
#[repr(C)]
#[derive(Debug)]
pub struct IntFxBaseData {
    __cordl_parent: FxBaseData,
    pub value: i32,
}
#[cfg(feature = "IntFxBaseData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IntFxBaseData => ""."IntFxBaseData"
);
#[cfg(feature = "IntFxBaseData")]
impl std::ops::Deref for IntFxBaseData {
    type Target = FxBaseData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IntFxBaseData")]
impl std::ops::DerefMut for IntFxBaseData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IntFxBaseData")]
impl IntFxBaseData {
    pub fn _ctor(
        &mut self,
        beat: f32,
        usePreviousEventValue: bool,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, usePreviousEventValue, value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beat: f32,
        usePreviousEventValue: bool,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, usePreviousEventValue, value))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "IntFxBaseData")]
impl quest_hook::libil2cpp::ObjectType for IntFxBaseData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
