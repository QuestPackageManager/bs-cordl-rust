#[cfg(feature = "CustomLevelLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomLevelLoader {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultEnvironmentInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
        >,
    >,
    pub _defaultAllDirectionsEnvironmentInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
        >,
    >,
}
#[cfg(feature = "CustomLevelLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CustomLevelLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CustomLevelLoader";
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
#[cfg(feature = "CustomLevelLoader")]
impl std::ops::Deref for crate::GlobalNamespace::CustomLevelLoader {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CustomLevelLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::CustomLevelLoader {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CustomLevelLoader")]
impl crate::GlobalNamespace::CustomLevelLoader {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CustomLevelLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CustomLevelLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CustomLevelLoader")]
impl AsRef<crate::GlobalNamespace::ICustomLevelLoader>
for crate::GlobalNamespace::CustomLevelLoader {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICustomLevelLoader {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CustomLevelLoader")]
impl AsMut<crate::GlobalNamespace::ICustomLevelLoader>
for crate::GlobalNamespace::CustomLevelLoader {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ICustomLevelLoader {
        unsafe { std::mem::transmute(self) }
    }
}
