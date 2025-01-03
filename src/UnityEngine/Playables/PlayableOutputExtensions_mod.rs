#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableOutputExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableOutputExtensions
    => "UnityEngine.Playables"."PlayableOutputExtensions"
);
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl std::ops::Deref for crate::UnityEngine::Playables::PlayableOutputExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::PlayableOutputExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl crate::UnityEngine::Playables::PlayableOutputExtensions {
    pub fn AddNotificationReceiver<U>(
        output: U,
        receiver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotificationReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddNotificationReceiver", (output, receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSourceOutputPort<U>(output: U) -> quest_hook::libil2cpp::Result<i32>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSourceOutputPort", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSourcePlayable<U>(
        output: U,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSourcePlayable", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn PushNotification<U>(
        output: U,
        origin: crate::UnityEngine::Playables::Playable,
        notification: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotification,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PushNotification", (output, origin, notification, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceObject<U>(
        output: U,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetReferenceObject", (output, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSourcePlayable<U, V>(
        output: U,
        value: V,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        V: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSourcePlayable", (output, value, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUserData<U>(
        output: U,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetUserData", (output, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWeight<U>(
        output: U,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetWeight", (output, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableOutputExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::PlayableOutputExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
