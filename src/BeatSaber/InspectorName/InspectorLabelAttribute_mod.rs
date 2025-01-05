#[cfg(feature = "BeatSaber+InspectorName+InspectorLabelAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InspectorLabelAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>,
    pub CustomLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BeatSaber+InspectorName+InspectorLabelAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::InspectorName::InspectorLabelAttribute => "BeatSaber.InspectorName"
    ."InspectorLabelAttribute"
);
#[cfg(feature = "BeatSaber+InspectorName+InspectorLabelAttribute")]
impl std::ops::Deref for crate::BeatSaber::InspectorName::InspectorLabelAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+InspectorName+InspectorLabelAttribute")]
impl std::ops::DerefMut for crate::BeatSaber::InspectorName::InspectorLabelAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+InspectorName+InspectorLabelAttribute")]
impl crate::BeatSaber::InspectorName::InspectorLabelAttribute {
    pub fn New(
        customLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (customLabel))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        customLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (customLabel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+InspectorName+InspectorLabelAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::InspectorName::InspectorLabelAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
