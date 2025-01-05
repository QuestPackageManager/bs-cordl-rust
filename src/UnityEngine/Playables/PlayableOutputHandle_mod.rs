#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlayableOutputHandle {
    pub m_Handle: crate::System::IntPtr,
    pub m_Version: u32,
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableOutputHandle =>
    "UnityEngine.Playables"."PlayableOutputHandle"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddNotificationReceiver",
            (receiver),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddNotificationReceiver_Injected", (_unity_self, receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareVersion(
        lhs: crate::UnityEngine::Playables::PlayableOutputHandle,
        rhs: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareVersion", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (p),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_PlayableOutputHandle1(
        &mut self,
        other: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayableOutputType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPlayableOutputType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayableOutputType_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlayableOutputType_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSourceOutputPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSourceOutputPort",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSourceOutputPort_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSourceOutputPort_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSourcePlayable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSourcePlayable",
            (),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSourcePlayable_Injected", (_unity_self, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPlayableOutputOfType<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPlayableOutputOfType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid_Injected", (_unity_self))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PushNotification",
            (origin, notification, context),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PushNotification_Injected",
                (_unity_self, origin, notification, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceObject(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetReferenceObject",
            (target),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceObject_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetReferenceObject_Injected", (_unity_self, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSourcePlayable(
        &mut self,
        target: crate::UnityEngine::Playables::PlayableHandle,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetSourcePlayable",
            (target, port),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSourcePlayable_Injected", (_unity_self, target, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUserData(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetUserData",
            (target),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUserData_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetUserData_Injected", (_unity_self, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWeight(
        &mut self,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetWeight",
            (weight),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWeight_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetWeight_Injected", (_unity_self, weight))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Null() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::PlayableOutputHandle,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutputHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Null", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::Playables::PlayableOutputHandle,
        rhs: crate::UnityEngine::Playables::PlayableOutputHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
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
