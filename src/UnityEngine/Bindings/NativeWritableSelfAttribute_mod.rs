#[cfg(feature = "UnityEngine+Bindings+NativeWritableSelfAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeWritableSelfAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _WritableSelf_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+Bindings+NativeWritableSelfAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Bindings::NativeWritableSelfAttribute => "UnityEngine.Bindings"
    ."NativeWritableSelfAttribute"
);
#[cfg(feature = "UnityEngine+Bindings+NativeWritableSelfAttribute")]
impl std::ops::Deref for crate::UnityEngine::Bindings::NativeWritableSelfAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeWritableSelfAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Bindings::NativeWritableSelfAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeWritableSelfAttribute")]
impl crate::UnityEngine::Bindings::NativeWritableSelfAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WritableSelf(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WritableSelf", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeWritableSelfAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Bindings::NativeWritableSelfAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
