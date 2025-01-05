#[cfg(feature = "System+AttributeUsageAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeUsageAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub _attributeTarget: crate::System::AttributeTargets,
    pub _allowMultiple: bool,
    pub _inherited: bool,
}
#[cfg(feature = "System+AttributeUsageAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AttributeUsageAttribute => "System"
    ."AttributeUsageAttribute"
);
#[cfg(feature = "System+AttributeUsageAttribute")]
impl std::ops::Deref for crate::System::AttributeUsageAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AttributeUsageAttribute")]
impl std::ops::DerefMut for crate::System::AttributeUsageAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AttributeUsageAttribute")]
impl crate::System::AttributeUsageAttribute {
    pub fn New(
        validOn: crate::System::AttributeTargets,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (validOn))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        validOn: crate::System::AttributeTargets,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (validOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllowMultiple(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowMultiple", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Inherited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Inherited", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AllowMultiple(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowMultiple", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Inherited(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Inherited", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+AttributeUsageAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AttributeUsageAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
