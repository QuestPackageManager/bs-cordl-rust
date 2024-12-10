#[cfg(feature = "JumpReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct JumpReceiver {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _jumpToDestinationValid_k__BackingField: bool,
}
#[cfg(feature = "JumpReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::JumpReceiver => ""
    ."JumpReceiver"
);
#[cfg(feature = "JumpReceiver")]
impl std::ops::Deref for crate::GlobalNamespace::JumpReceiver {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JumpReceiver")]
impl std::ops::DerefMut for crate::GlobalNamespace::JumpReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JumpReceiver")]
impl crate::GlobalNamespace::JumpReceiver {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnNotify(
        &mut self,
        origin: crate::UnityEngine::Playables::Playable,
        notification: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotification,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNotify", (origin, notification, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_jumpToDestinationValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_jumpToDestinationValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_jumpToDestinationValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_jumpToDestinationValid", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "JumpReceiver")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::JumpReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "JumpReceiver")]
impl AsRef<crate::UnityEngine::Playables::INotificationReceiver>
for crate::GlobalNamespace::JumpReceiver {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::INotificationReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "JumpReceiver")]
impl AsMut<crate::UnityEngine::Playables::INotificationReceiver>
for crate::GlobalNamespace::JumpReceiver {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::INotificationReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
