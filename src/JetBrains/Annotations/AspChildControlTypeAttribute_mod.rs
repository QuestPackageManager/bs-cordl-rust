#[cfg(feature = "JetBrains+Annotations+AspChildControlTypeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AspChildControlTypeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _TagName_k__BackingField: *mut crate::System::String,
    pub _ControlType_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "JetBrains+Annotations+AspChildControlTypeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::AspChildControlTypeAttribute => "JetBrains.Annotations"
    ."AspChildControlTypeAttribute"
);
#[cfg(feature = "JetBrains+Annotations+AspChildControlTypeAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::AspChildControlTypeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspChildControlTypeAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::AspChildControlTypeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspChildControlTypeAttribute")]
impl crate::JetBrains::Annotations::AspChildControlTypeAttribute {
    pub fn get_ControlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ControlType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ControlType(
        &mut self,
        value: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ControlType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TagName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TagName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        tagName: *mut crate::System::String,
        controlType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagName, controlType))?;
        Ok(__cordl_ret)
    }
    pub fn get_TagName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TagName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        tagName: *mut crate::System::String,
        controlType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagName, controlType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "JetBrains+Annotations+AspChildControlTypeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::AspChildControlTypeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
