#[cfg(feature = "Unity+Properties+Internal+IPropertyBagRegister")]
#[repr(C)]
#[derive(Debug)]
pub struct IPropertyBagRegister {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+Internal+IPropertyBagRegister")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::IPropertyBagRegister => "Unity.Properties.Internal"
    ."IPropertyBagRegister"
);
#[cfg(feature = "Unity+Properties+Internal+IPropertyBagRegister")]
impl std::ops::Deref for crate::Unity::Properties::Internal::IPropertyBagRegister {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+IPropertyBagRegister")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::IPropertyBagRegister {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+IPropertyBagRegister")]
impl crate::Unity::Properties::Internal::IPropertyBagRegister {
    pub fn Register(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Register", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Unity+Properties+Internal+IPropertyBagRegister")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::IPropertyBagRegister {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
