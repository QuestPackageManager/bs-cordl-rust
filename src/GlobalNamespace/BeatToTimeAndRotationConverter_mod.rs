#[cfg(feature = "BeatToTimeAndRotationConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatToTimeAndRotationConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverter,
    pub _rotationTimeProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RotationTimeProcessor,
    >,
}
#[cfg(feature = "BeatToTimeAndRotationConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatToTimeAndRotationConverter
    => ""."BeatToTimeAndRotationConverter"
);
#[cfg(feature = "BeatToTimeAndRotationConverter")]
impl std::ops::Deref for crate::GlobalNamespace::BeatToTimeAndRotationConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatToTimeAndRotationConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatToTimeAndRotationConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatToTimeAndRotationConverter")]
impl crate::GlobalNamespace::BeatToTimeAndRotationConverter {
    pub fn BeatToRotation(&mut self, beat: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("BeatToRotation", (beat))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        rotationTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RotationTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor, rotationTimeProcessor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatToTimeAndRotationConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatToTimeAndRotationConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
