#[cfg(feature = "cordl_class_EnvironmentIdAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentIdAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub emptyIsAllowed: bool,
    pub emptyExplanation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub filter: crate::GlobalNamespace::EnvironmentIdFilter,
}
#[cfg(feature = "cordl_class_EnvironmentIdAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnvironmentIdAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnvironmentIdAttribute";
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
#[cfg(feature = "EnvironmentIdAttribute")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentIdAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentIdAttribute")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentIdAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentIdAttribute")]
impl crate::GlobalNamespace::EnvironmentIdAttribute {
    pub fn New(
        emptyExplanation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filter: crate::GlobalNamespace::EnvironmentIdFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (emptyExplanation, filter))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        emptyExplanation: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filter: crate::GlobalNamespace::EnvironmentIdFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::EnvironmentIdFilter,
                        ),
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
            cordl_method_info.invoke_unchecked(self, (emptyExplanation, filter))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_EnvironmentIdAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentIdAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
