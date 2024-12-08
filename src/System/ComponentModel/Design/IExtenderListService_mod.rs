#[cfg(feature = "System+ComponentModel+Design+IExtenderListService")]
#[repr(C)]
#[derive(Debug)]
pub struct IExtenderListService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+IExtenderListService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::IExtenderListService =>
    "System.ComponentModel.Design"."IExtenderListService"
);
#[cfg(feature = "System+ComponentModel+Design+IExtenderListService")]
impl std::ops::Deref for crate::System::ComponentModel::Design::IExtenderListService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IExtenderListService")]
impl std::ops::DerefMut for crate::System::ComponentModel::Design::IExtenderListService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IExtenderListService")]
impl crate::System::ComponentModel::Design::IExtenderListService {
    pub fn GetExtenderProviders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::IExtenderProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::IExtenderProvider,
        > = __cordl_object.invoke("GetExtenderProviders", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IExtenderListService")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::IExtenderListService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}