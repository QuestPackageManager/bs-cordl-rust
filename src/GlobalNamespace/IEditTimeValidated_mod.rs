#[cfg(feature = "IEditTimeValidated")]
#[repr(C)]
#[derive(Debug)]
pub struct IEditTimeValidated {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IEditTimeValidated")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IEditTimeValidated => ""
    ."IEditTimeValidated"
);
#[cfg(feature = "IEditTimeValidated")]
impl std::ops::Deref for crate::GlobalNamespace::IEditTimeValidated {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IEditTimeValidated")]
impl std::ops::DerefMut for crate::GlobalNamespace::IEditTimeValidated {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IEditTimeValidated")]
impl crate::GlobalNamespace::IEditTimeValidated {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IEditTimeValidated")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IEditTimeValidated {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
