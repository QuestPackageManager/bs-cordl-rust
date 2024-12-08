#[cfg(feature = "System+Resources+IResourceGroveler")]
#[repr(C)]
#[derive(Debug)]
pub struct IResourceGroveler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Resources+IResourceGroveler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::IResourceGroveler =>
    "System.Resources"."IResourceGroveler"
);
#[cfg(feature = "System+Resources+IResourceGroveler")]
impl std::ops::Deref for crate::System::Resources::IResourceGroveler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+IResourceGroveler")]
impl std::ops::DerefMut for crate::System::Resources::IResourceGroveler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+IResourceGroveler")]
impl crate::System::Resources::IResourceGroveler {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Resources+IResourceGroveler")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Resources::IResourceGroveler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
