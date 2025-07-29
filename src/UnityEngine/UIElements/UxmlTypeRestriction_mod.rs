#[cfg(feature = "cordl_class_UnityEngine+UIElements+UxmlTypeRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlTypeRestriction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UxmlTypeRestriction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UxmlTypeRestriction";
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
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl crate::UnityEngine::UIElements::UxmlTypeRestriction {
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlTypeRestriction,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UxmlTypeRestriction,
                        >),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UxmlTypeRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UxmlTypeRestriction>,
    >,
> for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UxmlTypeRestriction>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UxmlTypeRestriction>,
    >,
> for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UxmlTypeRestriction>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
