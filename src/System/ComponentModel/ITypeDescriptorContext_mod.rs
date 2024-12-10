#[cfg(feature = "System+ComponentModel+ITypeDescriptorContext")]
#[repr(C)]
#[derive(Debug)]
pub struct ITypeDescriptorContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+ITypeDescriptorContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ITypeDescriptorContext
    => "System.ComponentModel"."ITypeDescriptorContext"
);
#[cfg(feature = "System+ComponentModel+ITypeDescriptorContext")]
impl std::ops::Deref for crate::System::ComponentModel::ITypeDescriptorContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ITypeDescriptorContext")]
impl std::ops::DerefMut for crate::System::ComponentModel::ITypeDescriptorContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ITypeDescriptorContext")]
impl crate::System::ComponentModel::ITypeDescriptorContext {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IContainer,
        > = __cordl_object.invoke("get_Container", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+ITypeDescriptorContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ITypeDescriptorContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
