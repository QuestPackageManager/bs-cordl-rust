#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegratedSubsystem_1<TSubsystemDescriptor: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::IntegratedSubsystem,
    __cordl_phantom_TSubsystemDescriptor: std::marker::PhantomData<TSubsystemDescriptor>,
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
unsafe impl<
    TSubsystemDescriptor: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "IntegratedSubsystem`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine",
                        "IntegratedSubsystem`1",
                    )
                    .unwrap()
                    .make_generic::<(TSubsystemDescriptor)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<TSubsystemDescriptor: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    type Target = crate::UnityEngine::IntegratedSubsystem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<TSubsystemDescriptor: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<
    TSubsystemDescriptor: quest_hook::libil2cpp::Type,
> crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_SubsystemDescriptor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TSubsystemDescriptor>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), TSubsystemDescriptor, 0usize>("get_SubsystemDescriptor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_SubsystemDescriptor", 0usize
                )
            });
        let __cordl_ret: TSubsystemDescriptor = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_subsystemDescriptor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TSubsystemDescriptor>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), TSubsystemDescriptor, 0usize>("get_subsystemDescriptor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_subsystemDescriptor", 0usize
                )
            });
        let __cordl_ret: TSubsystemDescriptor = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<TSubsystemDescriptor: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
