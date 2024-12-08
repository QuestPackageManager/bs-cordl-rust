#[cfg(feature = "JetBrains+Annotations+AspRequiredAttributeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AspRequiredAttributeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Attribute_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "JetBrains+Annotations+AspRequiredAttributeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::AspRequiredAttributeAttribute => "JetBrains.Annotations"
    ."AspRequiredAttributeAttribute"
);
#[cfg(feature = "JetBrains+Annotations+AspRequiredAttributeAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::AspRequiredAttributeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspRequiredAttributeAttribute")]
impl std::ops::DerefMut
for crate::JetBrains::Annotations::AspRequiredAttributeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspRequiredAttributeAttribute")]
impl crate::JetBrains::Annotations::AspRequiredAttributeAttribute {
    pub fn New(
        attribute: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attribute))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        attribute: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attribute))?;
        Ok(__cordl_ret)
    }
    pub fn get_Attribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Attribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Attribute(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Attribute", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "JetBrains+Annotations+AspRequiredAttributeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::AspRequiredAttributeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
