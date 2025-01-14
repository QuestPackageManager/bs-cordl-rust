#[cfg(feature = "EnvironmentEffectsFilterPresetDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentEffectsFilterPresetDropdown {
    __cordl_parent: crate::GlobalNamespace::ValueDropdownController_1<
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    >,
}
#[cfg(feature = "EnvironmentEffectsFilterPresetDropdown")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnvironmentEffectsFilterPresetDropdown";
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
#[cfg(feature = "EnvironmentEffectsFilterPresetDropdown")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown {
    type Target = crate::GlobalNamespace::ValueDropdownController_1<
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentEffectsFilterPresetDropdown")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentEffectsFilterPresetDropdown")]
impl crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown {
    pub fn GetNamedValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Tuple_2<
                                crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >,
                    >,
                >,
                0usize,
            >("GetNamedValues")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNamedValues", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Tuple_2<
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
}
#[cfg(feature = "EnvironmentEffectsFilterPresetDropdown")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
