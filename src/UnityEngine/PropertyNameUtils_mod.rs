#[cfg(feature = "UnityEngine+PropertyNameUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyNameUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PropertyNameUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "PropertyNameUtils";
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
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl std::ops::Deref for crate::UnityEngine::PropertyNameUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl std::ops::DerefMut for crate::UnityEngine::PropertyNameUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl crate::UnityEngine::PropertyNameUtils {
    pub fn PropertyNameFromString(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::UnityEngine::PropertyName,
                1usize,
            >("PropertyNameFromString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PropertyNameFromString", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::PropertyName = unsafe {
            method.invoke_unchecked((), (name))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PropertyNameFromString_Injected(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PropertyName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PropertyName>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PropertyNameFromString_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PropertyNameFromString_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, ret))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PropertyNameUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
