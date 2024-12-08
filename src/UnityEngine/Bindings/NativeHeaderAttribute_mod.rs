#[cfg(feature = "UnityEngine+Bindings+NativeHeaderAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeHeaderAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Header_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+Bindings+NativeHeaderAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::NativeHeaderAttribute =>
    "UnityEngine.Bindings"."NativeHeaderAttribute"
);
#[cfg(feature = "UnityEngine+Bindings+NativeHeaderAttribute")]
impl std::ops::Deref for crate::UnityEngine::Bindings::NativeHeaderAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeHeaderAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Bindings::NativeHeaderAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeHeaderAttribute")]
impl crate::UnityEngine::Bindings::NativeHeaderAttribute {
    pub fn set_Header(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Header", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        header: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        header: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeHeaderAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Bindings::NativeHeaderAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
