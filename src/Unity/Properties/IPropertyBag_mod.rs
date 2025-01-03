#[cfg(feature = "Unity+Properties+IPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct IPropertyBag {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+IPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::IPropertyBag =>
    "Unity.Properties"."IPropertyBag"
);
#[cfg(feature = "Unity+Properties+IPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::IPropertyBag {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::IPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag")]
impl crate::Unity::Properties::IPropertyBag {
    pub fn Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<crate::Unity::Properties::ITypeVisitor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Accept", (visitor))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Properties::IPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
