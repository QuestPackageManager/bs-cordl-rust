#[cfg(feature = "System+ComponentModel+IExtenderProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IExtenderProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+IExtenderProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::IExtenderProvider =>
    "System.ComponentModel"."IExtenderProvider"
);
#[cfg(feature = "System+ComponentModel+IExtenderProvider")]
impl std::ops::Deref for crate::System::ComponentModel::IExtenderProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IExtenderProvider")]
impl std::ops::DerefMut for crate::System::ComponentModel::IExtenderProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+IExtenderProvider")]
impl crate::System::ComponentModel::IExtenderProvider {
    pub fn CanExtend(
        &mut self,
        extendee: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanExtend", (extendee))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+IExtenderProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::IExtenderProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
