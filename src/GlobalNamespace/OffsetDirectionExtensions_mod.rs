#[cfg(feature = "OffsetDirectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OffsetDirectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OffsetDirectionExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OffsetDirectionExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OffsetDirectionExtensions";
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
#[cfg(feature = "OffsetDirectionExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OffsetDirectionExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OffsetDirectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl crate::GlobalNamespace::OffsetDirectionExtensions {
    pub fn OppositeDirection(
        offsetDirection: crate::GlobalNamespace::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OffsetDirection> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::OffsetDirection),
                crate::GlobalNamespace::OffsetDirection,
                1usize,
            >("OppositeDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OppositeDirection", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OffsetDirection = unsafe {
            method.invoke_unchecked((), (offsetDirection))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OffsetDirectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OffsetDirectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
