#[cfg(feature = "UnityEngine+InputSystem+InputBindingResolver")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputBindingResolver {
    pub totalProcessorCount: i32,
    pub totalCompositeCount: i32,
    pub totalInteractionCount: i32,
    pub maps: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        >,
    >,
    pub controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
    >,
    pub memory: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
    pub interactions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<Blacklisted>,
    >,
    pub processors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputProcessor>,
        >,
    >,
    pub composites: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::InputBindingComposite,
            >,
        >,
    >,
    pub bindingMask: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_IsControlOnlyResolve: bool,
    pub m_Parameters: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingResolver")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputBindingResolver
    => "UnityEngine.InputSystem"."InputBindingResolver"
);
#[cfg(feature = "UnityEngine+InputSystem+InputBindingResolver")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputBindingResolver {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingResolver")]
impl crate::UnityEngine::InputSystem::InputBindingResolver {
    pub fn AddActionMap(
        &mut self,
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddActionMap",
            (actionMap),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyParameters(
        parameters: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        >,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        binding: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
        objectRegistrationName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        namesAndParameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ApplyParameters",
                (
                    parameters,
                    instance,
                    actionMap,
                    binding,
                    objectRegistrationName,
                    namesAndParameters,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignCompositePartIndex(
        composite: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        currentCompositePartCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssignCompositePartIndex",
                (composite, name, currentCompositePartCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateBindingComposite(
        binding: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputBindingComposite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputBindingComposite,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateBindingComposite", (binding, actionMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateWithParameters<TType>(
        &mut self,
        registrations: crate::UnityEngine::InputSystem::Utilities::TypeTable,
        namesAndParameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TType>>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        binding: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InstantiateWithParameters",
            (registrations, namesAndParameters, array, count, actionMap, binding),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartWithPreviousResolve(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionState,
        >,
        isFullResolve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StartWithPreviousResolve",
            (state, isFullResolve),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalActionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalActionCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalBindingCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalBindingCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalControlCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalControlCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalMapCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalMapCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingResolver")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::InputSystem::InputBindingResolver {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingResolver")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::InputSystem::InputBindingResolver {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        todo!()
    }
}
