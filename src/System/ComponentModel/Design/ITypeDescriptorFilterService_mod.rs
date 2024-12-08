#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
#[repr(C)]
#[derive(Debug)]
pub struct ITypeDescriptorFilterService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::ITypeDescriptorFilterService =>
    "System.ComponentModel.Design"."ITypeDescriptorFilterService"
);
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl std::ops::Deref
for crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    pub fn FilterAttributes(
        &mut self,
        component: *mut crate::System::ComponentModel::IComponent,
        attributes: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FilterAttributes", (component, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn FilterEvents(
        &mut self,
        component: *mut crate::System::ComponentModel::IComponent,
        events: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FilterEvents", (component, events))?;
        Ok(__cordl_ret)
    }
    pub fn FilterProperties(
        &mut self,
        component: *mut crate::System::ComponentModel::IComponent,
        properties: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FilterProperties", (component, properties))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
