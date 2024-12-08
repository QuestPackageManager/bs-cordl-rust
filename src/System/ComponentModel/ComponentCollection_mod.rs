#[cfg(feature = "System+ComponentModel+ComponentCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ComponentCollection {
    __cordl_parent: crate::System::Collections::ReadOnlyCollectionBase,
}
#[cfg(feature = "System+ComponentModel+ComponentCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ComponentCollection =>
    "System.ComponentModel"."ComponentCollection"
);
#[cfg(feature = "System+ComponentModel+ComponentCollection")]
impl std::ops::Deref for crate::System::ComponentModel::ComponentCollection {
    type Target = crate::System::Collections::ReadOnlyCollectionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ComponentCollection")]
impl std::ops::DerefMut for crate::System::ComponentModel::ComponentCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ComponentCollection")]
impl crate::System::ComponentModel::ComponentCollection {
    pub fn get_Item(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::ComponentModel::IComponent> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::IComponent = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+ComponentCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ComponentCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
