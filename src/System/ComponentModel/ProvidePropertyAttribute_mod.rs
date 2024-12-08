#[cfg(feature = "System+ComponentModel+ProvidePropertyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ProvidePropertyAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _PropertyName_k__BackingField: *mut crate::System::String,
    pub _ReceiverTypeName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+ComponentModel+ProvidePropertyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ProvidePropertyAttribute
    => "System.ComponentModel"."ProvidePropertyAttribute"
);
#[cfg(feature = "System+ComponentModel+ProvidePropertyAttribute")]
impl std::ops::Deref for crate::System::ComponentModel::ProvidePropertyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ProvidePropertyAttribute")]
impl std::ops::DerefMut for crate::System::ComponentModel::ProvidePropertyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ProvidePropertyAttribute")]
impl crate::System::ComponentModel::ProvidePropertyAttribute {
    pub fn get_PropertyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_PropertyName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReceiverTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ReceiverTypeName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+ProvidePropertyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ProvidePropertyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
