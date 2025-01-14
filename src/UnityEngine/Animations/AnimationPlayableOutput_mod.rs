#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationPlayableOutput {
    pub m_Handle: crate::UnityEngine::Playables::PlayableOutputHandle,
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Animations::AnimationPlayableOutput {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Animations";
    const CLASS_NAME: &'static str = "AnimationPlayableOutput";
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
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Animations::AnimationPlayableOutput {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Animations::AnimationPlayableOutput {
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
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Animations::AnimationPlayableOutput {
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
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Animations::AnimationPlayableOutput {
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
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationPlayableOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
impl crate::UnityEngine::Animations::AnimationPlayableOutput {
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Animations::AnimationPlayableOutput,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Playables::PlayableGraph,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
                ),
                crate::UnityEngine::Animations::AnimationPlayableOutput,
                3usize,
            >("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Create", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Animations::AnimationPlayableOutput = unsafe {
            method.invoke_unchecked((), (graph, name, target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::PlayableOutputHandle,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Playables::PlayableOutputHandle,
                0usize,
            >("GetHandle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHandle", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutputHandle = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
                0usize,
            >("GetTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTarget", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetTarget(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                >),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
                1usize,
            >("InternalGetTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalGetTarget", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator> = unsafe {
            method.invoke_unchecked((), (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetTarget(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("InternalSetTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalSetTarget", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTarget(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetTarget", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::PlayableOutputHandle),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (handle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Null() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Animations::AnimationPlayableOutput,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Animations::AnimationPlayableOutput,
                0usize,
            >("get_Null")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Null", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Animations::AnimationPlayableOutput = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit(
        output: crate::UnityEngine::Playables::PlayableOutput,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Animations::AnimationPlayableOutput,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Playables::PlayableOutput),
                crate::UnityEngine::Animations::AnimationPlayableOutput,
                1usize,
            >("op_Explicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Explicit", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Animations::AnimationPlayableOutput = unsafe {
            method.invoke_unchecked((), (output))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        output: crate::UnityEngine::Animations::AnimationPlayableOutput,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableOutput> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Animations::AnimationPlayableOutput),
                crate::UnityEngine::Playables::PlayableOutput,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Implicit", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutput = unsafe {
            method.invoke_unchecked((), (output))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
impl AsRef<crate::UnityEngine::Playables::IPlayableOutput>
for crate::UnityEngine::Animations::AnimationPlayableOutput {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayableOutput {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableOutput")]
impl AsMut<crate::UnityEngine::Playables::IPlayableOutput>
for crate::UnityEngine::Animations::AnimationPlayableOutput {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayableOutput {
        todo!()
    }
}
