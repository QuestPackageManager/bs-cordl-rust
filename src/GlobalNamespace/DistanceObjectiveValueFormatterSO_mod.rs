#[cfg(feature = "DistanceObjectiveValueFormatterSO")]
#[repr(C)]
#[derive(Debug)]
pub struct DistanceObjectiveValueFormatterSO {
    __cordl_parent: crate::GlobalNamespace::ObjectiveValueFormatterSO,
}
#[cfg(feature = "DistanceObjectiveValueFormatterSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DistanceObjectiveValueFormatterSO => ""
    ."DistanceObjectiveValueFormatterSO"
);
#[cfg(feature = "DistanceObjectiveValueFormatterSO")]
impl std::ops::Deref for crate::GlobalNamespace::DistanceObjectiveValueFormatterSO {
    type Target = crate::GlobalNamespace::ObjectiveValueFormatterSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DistanceObjectiveValueFormatterSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::DistanceObjectiveValueFormatterSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DistanceObjectiveValueFormatterSO")]
impl crate::GlobalNamespace::DistanceObjectiveValueFormatterSO {
    pub fn FormatValue(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("FormatValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DistanceObjectiveValueFormatterSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DistanceObjectiveValueFormatterSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
