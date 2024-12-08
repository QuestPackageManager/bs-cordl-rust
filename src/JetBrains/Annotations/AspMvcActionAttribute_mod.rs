#[cfg(feature = "JetBrains+Annotations+AspMvcActionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AspMvcActionAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _AnonymousProperty_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "JetBrains+Annotations+AspMvcActionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::JetBrains::Annotations::AspMvcActionAttribute =>
    "JetBrains.Annotations"."AspMvcActionAttribute"
);
#[cfg(feature = "JetBrains+Annotations+AspMvcActionAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::AspMvcActionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcActionAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::AspMvcActionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcActionAttribute")]
impl crate::JetBrains::Annotations::AspMvcActionAttribute {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        anonymousProperty: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (anonymousProperty))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        anonymousProperty: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (anonymousProperty))?;
        Ok(__cordl_ret)
    }
    pub fn get_AnonymousProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AnonymousProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AnonymousProperty(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AnonymousProperty", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "JetBrains+Annotations+AspMvcActionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::AspMvcActionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}