#[cfg(feature = "IBeatToTimeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeatToTimeConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBeatToTimeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IBeatToTimeConverter => ""
    ."IBeatToTimeConverter"
);
#[cfg(feature = "IBeatToTimeConverter")]
impl std::ops::Deref for crate::GlobalNamespace::IBeatToTimeConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatToTimeConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBeatToTimeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatToTimeConverter")]
impl crate::GlobalNamespace::IBeatToTimeConverter {
    pub fn ConvertBeatToTime(
        &mut self,
        beat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ConvertBeatToTime", (beat))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IBeatToTimeConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IBeatToTimeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
