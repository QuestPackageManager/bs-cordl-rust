#[cfg(feature = "StandaloneMonobehaviorExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StandaloneMonobehaviorExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "StandaloneMonobehaviorExtensions";
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
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    pub fn Dispatch(
        standaloneMonoBehavior: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IStandaloneMonobehavior,
        >,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::StandaloneMonobehaviorExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IStandaloneMonobehavior,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Dispatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::StandaloneMonobehaviorExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "Dispatch", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (standaloneMonoBehavior, action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchAsync(
        standaloneMonoBehavior: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IStandaloneMonobehavior,
        >,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::StandaloneMonobehaviorExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IStandaloneMonobehavior,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                2usize,
            >("DispatchAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::StandaloneMonobehaviorExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "DispatchAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked((), (standaloneMonoBehavior, action))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandaloneMonobehaviorExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandaloneMonobehaviorExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
