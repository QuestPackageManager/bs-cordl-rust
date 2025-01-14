#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayableOutputHandle {
    pub m_Handle: crate::System::IntPtr,
    pub m_Version: u32,
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Playables::PlayableOutputHandle {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Playables";
    const CLASS_NAME: &'static str = "PlayableOutputHandle";
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
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Playables::PlayableOutputHandle {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Playables::PlayableOutputHandle {
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
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Playables::PlayableOutputHandle {
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
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Playables::PlayableOutputHandle {
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
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::PlayableOutputHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
impl crate::UnityEngine::Playables::PlayableOutputHandle {
    pub fn AddNotificationReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotificationReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Playables::INotificationReceiver,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddNotificationReceiver")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddNotificationReceiver", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (receiver))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddNotificationReceiver_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        receiver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotificationReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Playables::INotificationReceiver,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddNotificationReceiver_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddNotificationReceiver_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_unity_self, receiver))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareVersion(
        lhs: crate::UnityEngine::Playables::PlayableOutputHandle,
        rhs: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                ),
                bool,
                2usize,
            >("CompareVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareVersion", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (lhs, rhs)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (p)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_PlayableOutputHandle1(
        &mut self,
        other: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::PlayableOutputHandle),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayableOutputType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                0usize,
            >("GetPlayableOutputType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPlayableOutputType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayableOutputType_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("GetPlayableOutputType_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPlayableOutputType_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (_unity_self))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSourceOutputPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetSourceOutputPort")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSourceOutputPort", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSourceOutputPort_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                >),
                i32,
                1usize,
            >("GetSourceOutputPort_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSourceOutputPort_Injected", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (_unity_self)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSourcePlayable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Playables::PlayableHandle,
                0usize,
            >("GetSourcePlayable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSourcePlayable", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSourcePlayable_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableHandle,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetSourcePlayable_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSourcePlayable_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_unity_self, ret))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPlayableOutputOfType<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsPlayableOutputOfType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsPlayableOutputOfType", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsValid", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                >),
                bool,
                1usize,
            >("IsValid_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsValid_Injected", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_unity_self)) };
        Ok(__cordl_ret.into())
    }
    pub fn PushNotification(
        &mut self,
        origin: crate::UnityEngine::Playables::PlayableHandle,
        notification: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotification,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::PlayableHandle,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Playables::INotification,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PushNotification")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PushNotification", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (origin, notification, context))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PushNotification_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        origin: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        notification: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotification,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableHandle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Playables::INotification,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("PushNotification_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PushNotification_Injected", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_unity_self, origin, notification, context))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceObject(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetReferenceObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetReferenceObject", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceObject_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetReferenceObject_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetReferenceObject_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_unity_self, target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSourcePlayable(
        &mut self,
        target: crate::UnityEngine::Playables::PlayableHandle,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::PlayableHandle, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetSourcePlayable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetSourcePlayable", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (target, port))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSourcePlayable_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        target: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableHandle,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetSourcePlayable_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetSourcePlayable_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_unity_self, target, port))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetUserData(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetUserData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetUserData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetUserData_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetUserData_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetUserData_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_unity_self, target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWeight(
        &mut self,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("SetWeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWeight", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (weight))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWeight_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetWeight_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetWeight_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_unity_self, weight))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Null() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::PlayableOutputHandle,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::Playables::PlayableOutputHandle,
                0usize,
            >("get_Null")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Null", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutputHandle = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::Playables::PlayableOutputHandle,
        rhs: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                    crate::UnityEngine::Playables::PlayableOutputHandle,
                ),
                bool,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (lhs, rhs)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::Playables::PlayableOutputHandle>,
> for crate::UnityEngine::Playables::PlayableOutputHandle {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Playables::PlayableOutputHandle,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::Playables::PlayableOutputHandle>,
> for crate::UnityEngine::Playables::PlayableOutputHandle {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Playables::PlayableOutputHandle,
    > {
        todo!()
    }
}
