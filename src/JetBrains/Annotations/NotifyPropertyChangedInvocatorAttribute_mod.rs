#[cfg(feature = "JetBrains+Annotations+NotifyPropertyChangedInvocatorAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NotifyPropertyChangedInvocatorAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ParameterName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "JetBrains+Annotations+NotifyPropertyChangedInvocatorAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::NotifyPropertyChangedInvocatorAttribute =>
    "JetBrains.Annotations"."NotifyPropertyChangedInvocatorAttribute"
);
#[cfg(feature = "JetBrains+Annotations+NotifyPropertyChangedInvocatorAttribute")]
impl std::ops::Deref
for crate::JetBrains::Annotations::NotifyPropertyChangedInvocatorAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+NotifyPropertyChangedInvocatorAttribute")]
impl std::ops::DerefMut
for crate::JetBrains::Annotations::NotifyPropertyChangedInvocatorAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+NotifyPropertyChangedInvocatorAttribute")]
impl crate::JetBrains::Annotations::NotifyPropertyChangedInvocatorAttribute {
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
        parameterName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameterName))?;
        Ok(__cordl_ret)
    }
    pub fn get_ParameterName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ParameterName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ParameterName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ParameterName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        parameterName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameterName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "JetBrains+Annotations+NotifyPropertyChangedInvocatorAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::NotifyPropertyChangedInvocatorAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
