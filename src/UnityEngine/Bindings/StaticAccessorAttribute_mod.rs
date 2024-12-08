#[cfg(feature = "UnityEngine+Bindings+StaticAccessorAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticAccessorAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Name_k__BackingField: *mut crate::System::String,
    pub _Type_k__BackingField: crate::UnityEngine::Bindings::StaticAccessorType,
}
#[cfg(feature = "UnityEngine+Bindings+StaticAccessorAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::StaticAccessorAttribute
    => "UnityEngine.Bindings"."StaticAccessorAttribute"
);
#[cfg(feature = "UnityEngine+Bindings+StaticAccessorAttribute")]
impl std::ops::Deref for crate::UnityEngine::Bindings::StaticAccessorAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+StaticAccessorAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Bindings::StaticAccessorAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+StaticAccessorAttribute")]
impl crate::UnityEngine::Bindings::StaticAccessorAttribute {
    pub fn New_StaticAccessorType1(
        name: *mut crate::System::String,
        _cordl_type: crate::UnityEngine::Bindings::StaticAccessorType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, _cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn New_String0(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_StaticAccessorType1(
        &mut self,
        name: *mut crate::System::String,
        _cordl_type: crate::UnityEngine::Bindings::StaticAccessorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn set_Name(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Name", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Type(
        &mut self,
        value: crate::UnityEngine::Bindings::StaticAccessorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Type", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Bindings+StaticAccessorAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Bindings::StaticAccessorAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
