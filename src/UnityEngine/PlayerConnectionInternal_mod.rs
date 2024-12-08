#[cfg(feature = "UnityEngine+PlayerConnectionInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerConnectionInternal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+PlayerConnectionInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerConnectionInternal =>
    "UnityEngine"."PlayerConnectionInternal"
);
#[cfg(feature = "UnityEngine+PlayerConnectionInternal")]
impl std::ops::Deref for crate::UnityEngine::PlayerConnectionInternal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PlayerConnectionInternal")]
impl std::ops::DerefMut for crate::UnityEngine::PlayerConnectionInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PlayerConnectionInternal")]
impl crate::UnityEngine::PlayerConnectionInternal {
    pub fn UnityEngine_IPlayerEditorConnectionNative_UnregisterInternal(
        &mut self,
        messageId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.IPlayerEditorConnectionNative.UnregisterInternal",
                (messageId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_IPlayerEditorConnectionNative_Poll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.IPlayerEditorConnectionNative.Poll", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_IPlayerEditorConnectionNative_IsConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.IPlayerEditorConnectionNative.IsConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_IPlayerEditorConnectionNative_Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.IPlayerEditorConnectionNative.Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_IPlayerEditorConnectionNative_RegisterInternal(
        &mut self,
        messageId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.IPlayerEditorConnectionNative.RegisterInternal",
                (messageId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_IPlayerEditorConnectionNative_DisconnectAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.IPlayerEditorConnectionNative.DisconnectAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_IPlayerEditorConnectionNative_SendMessage(
        &mut self,
        messageId: crate::System::Guid,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.IPlayerEditorConnectionNative.SendMessage",
                (messageId, data, playerId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_IPlayerEditorConnectionNative_TrySendMessage(
        &mut self,
        messageId: crate::System::Guid,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.IPlayerEditorConnectionNative.TrySendMessage",
                (messageId, data, playerId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+PlayerConnectionInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PlayerConnectionInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
