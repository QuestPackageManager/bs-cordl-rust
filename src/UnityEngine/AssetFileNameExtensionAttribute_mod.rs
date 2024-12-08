#[cfg(feature = "UnityEngine+AssetFileNameExtensionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetFileNameExtensionAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _preferredExtension_k__BackingField: *mut crate::System::String,
    pub _otherExtensions_k__BackingField: *mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "UnityEngine+AssetFileNameExtensionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssetFileNameExtensionAttribute =>
    "UnityEngine"."AssetFileNameExtensionAttribute"
);
#[cfg(feature = "UnityEngine+AssetFileNameExtensionAttribute")]
impl std::ops::Deref for crate::UnityEngine::AssetFileNameExtensionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetFileNameExtensionAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::AssetFileNameExtensionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetFileNameExtensionAttribute")]
impl crate::UnityEngine::AssetFileNameExtensionAttribute {
    pub fn _ctor(
        &mut self,
        preferredExtension: *mut crate::System::String,
        otherExtensions: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (preferredExtension, otherExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        preferredExtension: *mut crate::System::String,
        otherExtensions: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (preferredExtension, otherExtensions))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+AssetFileNameExtensionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AssetFileNameExtensionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
