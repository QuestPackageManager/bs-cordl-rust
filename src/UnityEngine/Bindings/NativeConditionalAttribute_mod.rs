#[cfg(feature = "UnityEngine+Bindings+NativeConditionalAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeConditionalAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Condition_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _Enabled_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+Bindings+NativeConditionalAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Bindings::NativeConditionalAttribute => "UnityEngine.Bindings"
    ."NativeConditionalAttribute"
);
#[cfg(feature = "UnityEngine+Bindings+NativeConditionalAttribute")]
impl std::ops::Deref for crate::UnityEngine::Bindings::NativeConditionalAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeConditionalAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Bindings::NativeConditionalAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeConditionalAttribute")]
impl crate::UnityEngine::Bindings::NativeConditionalAttribute {
    pub fn New(
        condition: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (condition))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        condition: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (condition))?;
        Ok(__cordl_ret)
    }
    pub fn set_Condition(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Condition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Enabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Enabled", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeConditionalAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Bindings::NativeConditionalAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
