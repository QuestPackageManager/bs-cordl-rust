#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderInfoStorageRGBAFloat {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<
        crate::UnityEngine::Color,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "ShaderInfoStorageRGBAFloat";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    type Target = crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<
        crate::UnityEngine::Color,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    pub fn New(
        initialSize: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialSize, maxSize))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        initialSize: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialSize, maxSize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBAFloat")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBAFloat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
