#[cfg(feature = "System+ComponentModel+INestedSite")]
#[repr(C)]
#[derive(Debug)]
pub struct INestedSite {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+INestedSite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::INestedSite =>
    "System.ComponentModel"."INestedSite"
);
#[cfg(feature = "System+ComponentModel+INestedSite")]
impl std::ops::Deref for crate::System::ComponentModel::INestedSite {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+INestedSite")]
impl std::ops::DerefMut for crate::System::ComponentModel::INestedSite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+INestedSite")]
impl crate::System::ComponentModel::INestedSite {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_FullName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+INestedSite")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ComponentModel::INestedSite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
