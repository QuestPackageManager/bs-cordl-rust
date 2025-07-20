#[cfg(feature = "XRSubsystemHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSubsystemHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "XRSubsystemHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::XRSubsystemHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "XRSubsystemHelper";
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
#[cfg(feature = "XRSubsystemHelper")]
impl std::ops::Deref for crate::GlobalNamespace::XRSubsystemHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::XRSubsystemHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl crate::GlobalNamespace::XRSubsystemHelper {
    pub fn GetCurrentDisplaySubsystem() -> quest_hook::libil2cpp::Result<Blacklisted> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::XRSubsystemHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), Blacklisted, 0usize>("GetCurrentDisplaySubsystem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::XRSubsystemHelper as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetCurrentDisplaySubsystem", 0usize
                )
            });
        let __cordl_ret: Blacklisted = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentDisplaySubsystemDescriptor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRDisplaySubsystemDescriptor>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::XRSubsystemHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::XR::XRDisplaySubsystemDescriptor,
                >,
                0usize,
            >("GetCurrentDisplaySubsystemDescriptor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::XRSubsystemHelper as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetCurrentDisplaySubsystemDescriptor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRDisplaySubsystemDescriptor,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentInputSubsystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRInputSubsystem>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::XRSubsystemHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRInputSubsystem>,
                0usize,
            >("GetCurrentInputSubsystem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::XRSubsystemHelper as
                    quest_hook::libil2cpp::Type > ::class(), "GetCurrentInputSubsystem",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRInputSubsystem,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "XRSubsystemHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::XRSubsystemHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
