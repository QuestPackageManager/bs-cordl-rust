#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionSetupExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionSetupExtensions";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    #[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
    pub type BindingSyntax = crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax"
    )]
    pub type CompositeSyntax = crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
    )]
    pub type ControlSchemeSyntax = crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax;
    pub fn AddAction(
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::UnityEngine::InputSystem::InputActionType,
        binding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expectedControlLayout: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::InputSystem::InputActionType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputAction,
                        >,
                        8usize,
                    >("AddAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddAction", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        map,
                        name,
                        _cordl_type,
                        binding,
                        interactions,
                        processors,
                        groups,
                        expectedControlLayout,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddActionMap_Il2CppString0(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputActionMap,
                        >,
                        2usize,
                    >("AddActionMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddActionMap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = unsafe { method.invoke_unchecked((), (asset, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddActionMap_InputActionMap1(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddActionMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddActionMap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBindingInternal(
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        binding: crate::UnityEngine::InputSystem::InputBinding,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            crate::UnityEngine::InputSystem::InputBinding,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("AddBindingInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBindingInternal", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (map, binding, bindingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBinding_InputActionMap_Il2CppString_Guid_Il2CppString_Il2CppString5(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: crate::System::Guid,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Guid,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        5usize,
                    >("AddBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBinding", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (actionMap, path, action, interactions, groups))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBinding_InputActionMap_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString3(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        6usize,
                    >("AddBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBinding", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (actionMap, path, interactions, groups, action, processors),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBinding_InputActionMap_Il2CppString_InputAction_Il2CppString_Il2CppString4(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        5usize,
                    >("AddBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBinding", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (actionMap, path, action, interactions, groups))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBinding_InputActionMap_InputBinding6(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        binding: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            crate::UnityEngine::InputSystem::InputBinding,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("AddBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (actionMap, binding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBinding_InputAction_Il2CppString_Il2CppString_Il2CppString_Il2CppString0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        5usize,
                    >("AddBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBinding", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method
                .invoke_unchecked((), (action, path, interactions, processors, groups))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBinding_InputAction_InputBinding2(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        binding: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            crate::UnityEngine::InputSystem::InputBinding,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("AddBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, binding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBinding_InputAction_InputControl1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("AddBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, control))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCompositeBinding(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        composite: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax,
                        4usize,
                    >("AddCompositeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddCompositeBinding", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax = unsafe {
            method.invoke_unchecked((), (action, composite, interactions, processors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddControlScheme_Il2CppString1(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        2usize,
                    >("AddControlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddControlScheme", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked((), (asset, name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddControlScheme_InputControlScheme0(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        controlScheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            crate::UnityEngine::InputSystem::InputControlScheme,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddControlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddControlScheme", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, controlScheme))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBindingWithGroup(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBindingWithGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBindingWithGroup", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, group))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBindingWithId_Guid1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            crate::System::Guid,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBindingWithId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBindingWithId", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBindingWithId_Il2CppString0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBindingWithId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBindingWithId", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBindingWithPath(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBindingWithPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBindingWithPath", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, path))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBinding_InputActionMap_i32_2(
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (actionMap, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBinding_InputAction_Il2CppString1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBinding_InputAction_InputBinding3(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        _cordl_match: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            crate::UnityEngine::InputSystem::InputBinding,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, _cordl_match))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeBinding_InputAction_i32_0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeCompositeBinding(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        compositeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("ChangeCompositeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ChangeCompositeBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked((), (action, compositeName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrWithOptionalDevice(
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::InputControlScheme,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputControlScheme,
                        2usize,
                    >("OrWithOptionalDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OrWithOptionalDevice", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = unsafe {
            method.invoke_unchecked((), (scheme, controlPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrWithRequiredDevice(
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::InputControlScheme,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputControlScheme,
                        2usize,
                    >("OrWithRequiredDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OrWithRequiredDevice", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = unsafe {
            method.invoke_unchecked((), (scheme, controlPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveActionMap_Il2CppString1(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        nameOrId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RemoveActionMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RemoveActionMap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, nameOrId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveActionMap_InputActionMap0(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RemoveActionMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RemoveActionMap", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAction_InputAction0(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputAction,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RemoveAction", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAction_InputActionAsset_Il2CppString1(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        nameOrId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RemoveAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RemoveAction", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, nameOrId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveControlScheme(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RemoveControlScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RemoveControlScheme", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Rename(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        newName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Rename")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Rename", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (action, newName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithBindingGroup(
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
        bindingGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::InputControlScheme,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputControlScheme,
                        2usize,
                    >("WithBindingGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithBindingGroup", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = unsafe {
            method.invoke_unchecked((), (scheme, bindingGroup))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithDevice(
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        required: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::InputControlScheme,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        crate::UnityEngine::InputSystem::InputControlScheme,
                        3usize,
                    >("WithDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithDevice", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = unsafe {
            method.invoke_unchecked((), (scheme, controlPath, required))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithOptionalDevice(
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::InputControlScheme,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputControlScheme,
                        2usize,
                    >("WithOptionalDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithOptionalDevice", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = unsafe {
            method.invoke_unchecked((), (scheme, controlPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithRequiredDevice(
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::InputControlScheme,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputControlScheme,
                        2usize,
                    >("WithRequiredDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithRequiredDevice", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = unsafe {
            method.invoke_unchecked((), (scheme, controlPath))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionSetupExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionSetupExtensions_BindingSyntax {
    pub m_ActionMap: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionMap,
    >,
    pub m_Action: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_BindingIndexInMap: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionSetupExtensions/BindingSyntax";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+BindingSyntax")]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax {
    pub fn Erase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Erase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Erase", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertPartBinding(
        &mut self,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("InsertPartBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InsertPartBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (partName, path))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Iterate(
        &mut self,
        next: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("Iterate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Iterate", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (next))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IterateCompositeBinding(
        &mut self,
        next: bool,
        compositeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("IterateCompositeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IterateCompositeBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (next, compositeName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IteratePartBinding(
        &mut self,
        next: bool,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        2usize,
                    >("IteratePartBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IteratePartBinding", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (next, partName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        0usize,
                    >("NextBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NextBinding", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextCompositeBinding(
        &mut self,
        compositeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("NextCompositeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NextCompositeBinding", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (compositeName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextPartBinding(
        &mut self,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("NextPartBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NextPartBinding", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (partName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PreviousBinding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        0usize,
                    >("PreviousBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PreviousBinding", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PreviousCompositeBinding(
        &mut self,
        compositeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("PreviousCompositeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PreviousCompositeBinding", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (compositeName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PreviousPartBinding(
        &mut self,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("PreviousPartBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PreviousPartBinding", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (partName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn To(
        &mut self,
        binding: crate::UnityEngine::InputSystem::InputBinding,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::InputBinding),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("To")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "To", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (binding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Triggering(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputAction,
                        >),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("Triggering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Triggering", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithGroup", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (group))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithGroups(
        &mut self,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithGroups", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (groups))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithInteraction_1<TInteraction>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    >
    where
        TInteraction: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        0usize,
                    >("WithInteraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithInteraction", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithInteraction_Il2CppString0(
        &mut self,
        interaction: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithInteraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithInteraction", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (interaction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithInteractions(
        &mut self,
        interactions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithInteractions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithInteractions", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (interactions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithPath(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithPath", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (path))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessor_1<TProcessor>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    >
    where
        TProcessor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        0usize,
                    >("WithProcessor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithProcessor", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessor_Il2CppString0(
        &mut self,
        processor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithProcessor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithProcessor", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (processor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessors(
        &mut self,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax,
                        1usize,
                    >("WithProcessors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithProcessors", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_BindingSyntax = unsafe {
            method.invoke_unchecked(self, (processors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        bindingIndexInMap: i32,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map, bindingIndexInMap, action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_binding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::InputBinding> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputBinding,
                        0usize,
                    >("get_binding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_binding", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputBinding = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_bindingIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_bindingIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_bindingIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("get_valid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_valid", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionSetupExtensions_CompositeSyntax {
    pub m_Action: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_ActionMap: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionMap,
    >,
    pub m_BindingIndexInMap: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionSetupExtensions/CompositeSyntax";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionSetupExtensions+CompositeSyntax")]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax {
    pub fn With(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        binding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        groups: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax,
                        4usize,
                    >("With")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "With", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_CompositeSyntax = unsafe {
            method.invoke_unchecked(self, (name, binding, groups, processors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        compositeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionMap,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputAction,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map, action, compositeIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_bindingIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_bindingIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_bindingIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionSetupExtensions_ControlSchemeSyntax {
    pub m_Asset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionAsset,
    >,
    pub m_ControlSchemeIndex: i32,
    pub m_ControlScheme: crate::UnityEngine::InputSystem::InputControlScheme,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionSetupExtensions/ControlSchemeSyntax";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionSetupExtensions+ControlSchemeSyntax"
)]
impl crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax {
    pub fn AddDeviceEntry(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: crate::UnityEngine::InputSystem::DeviceRequirement_InputControlScheme_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::InputSystem::DeviceRequirement_InputControlScheme_Flags,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddDeviceEntry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddDeviceEntry", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (controlPath, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeviceTypeToControlPath<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("DeviceTypeToControlPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DeviceTypeToControlPath", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Done(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputControlScheme,
                        0usize,
                    >("Done")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Done", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrWithOptionalDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        0usize,
                    >("OrWithOptionalDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OrWithOptionalDevice", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrWithOptionalDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        1usize,
                    >("OrWithOptionalDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OrWithOptionalDevice", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, (controlPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrWithRequiredDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        0usize,
                    >("OrWithRequiredDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OrWithRequiredDevice", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OrWithRequiredDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        1usize,
                    >("OrWithRequiredDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "OrWithRequiredDevice", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, (controlPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithBindingGroup(
        &mut self,
        bindingGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        1usize,
                    >("WithBindingGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithBindingGroup", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, (bindingGroup))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithOptionalDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        0usize,
                    >("WithOptionalDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithOptionalDevice", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithOptionalDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        1usize,
                    >("WithOptionalDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithOptionalDevice", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, (controlPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithRequiredDevice_0<TDevice>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        0usize,
                    >("WithRequiredDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithRequiredDevice", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithRequiredDevice_Il2CppString1(
        &mut self,
        controlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax,
                        1usize,
                    >("WithRequiredDevice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithRequiredDevice", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionSetupExtensions_ControlSchemeSyntax = unsafe {
            method.invoke_unchecked(self, (controlPath))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputActionAsset_i32_0(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputActionAsset,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (asset, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputControlScheme1(
        &mut self,
        controlScheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::InputControlScheme),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (controlScheme))?
        };
        Ok(__cordl_ret.into())
    }
}
