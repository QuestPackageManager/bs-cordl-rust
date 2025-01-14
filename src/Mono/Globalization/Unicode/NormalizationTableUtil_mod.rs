#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct NormalizationTableUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "NormalizationTableUtil";
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
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    pub fn MapIdx(cp: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("MapIdx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MapIdx", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (cp)) };
        Ok(__cordl_ret.into())
    }
    pub fn PropIdx(cp: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("PropIdx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PropIdx", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (cp)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
