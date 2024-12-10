#[cfg(feature = "System+ComponentModel+AttributeProviderAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeProviderAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _TypeName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _PropertyName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+ComponentModel+AttributeProviderAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::AttributeProviderAttribute => "System.ComponentModel"
    ."AttributeProviderAttribute"
);
#[cfg(feature = "System+ComponentModel+AttributeProviderAttribute")]
impl std::ops::Deref for crate::System::ComponentModel::AttributeProviderAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeProviderAttribute")]
impl std::ops::DerefMut for crate::System::ComponentModel::AttributeProviderAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+AttributeProviderAttribute")]
impl crate::System::ComponentModel::AttributeProviderAttribute {
    pub fn get_PropertyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_PropertyName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_TypeName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+AttributeProviderAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::AttributeProviderAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
