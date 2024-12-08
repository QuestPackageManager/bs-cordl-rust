#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NumericFieldDraggerUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::NumericFieldDraggerUtility =>
    "UnityEngine"."NumericFieldDraggerUtility"
);
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl std::ops::Deref for crate::UnityEngine::NumericFieldDraggerUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl std::ops::DerefMut for crate::UnityEngine::NumericFieldDraggerUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl crate::UnityEngine::NumericFieldDraggerUtility {}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::NumericFieldDraggerUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
