#[cfg(feature = "BeatToTimeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatToTimeConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bpmTimeProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatToTimeConverter,
    >,
}
#[cfg(feature = "BeatToTimeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatToTimeConverter => ""
    ."BeatToTimeConverter"
);
#[cfg(feature = "BeatToTimeConverter")]
impl std::ops::Deref for crate::GlobalNamespace::BeatToTimeConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatToTimeConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatToTimeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatToTimeConverter")]
impl crate::GlobalNamespace::BeatToTimeConverter {
    pub fn BeatToTime(&mut self, beat: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("BeatToTime", (beat))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatToTimeConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatToTimeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
