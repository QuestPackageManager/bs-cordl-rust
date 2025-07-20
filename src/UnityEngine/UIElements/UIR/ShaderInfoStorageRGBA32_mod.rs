#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBA32")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderInfoStorageRGBA32 {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<
        crate::UnityEngine::Color32,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBA32")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBA32 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "ShaderInfoStorageRGBA32";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBA32")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBA32 {
    type Target = crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<
        crate::UnityEngine::Color32,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBA32")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBA32 {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBA32")]
impl crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBA32 {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (initialSize, maxSize))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorageRGBA32")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorageRGBA32 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
