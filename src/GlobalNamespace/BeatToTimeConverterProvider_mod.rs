#[cfg(feature = "BeatToTimeConverterProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatToTimeConverterProvider {
    __cordl_parent: crate::System::Object,
    pub _bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
}
#[cfg(feature = "BeatToTimeConverterProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatToTimeConverterProvider =>
    ""."BeatToTimeConverterProvider"
);
#[cfg(feature = "BeatToTimeConverterProvider")]
impl std::ops::Deref for crate::GlobalNamespace::BeatToTimeConverterProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatToTimeConverterProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatToTimeConverterProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatToTimeConverterProvider")]
impl crate::GlobalNamespace::BeatToTimeConverterProvider {
    pub fn BeatToTime(&mut self, beat: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("BeatToTime", (beat))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatToTimeConverterProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatToTimeConverterProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
