#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct IEditorPlayerConnection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Networking.PlayerConnection";
    const CLASS_NAME: &'static str = "IEditorPlayerConnection";
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
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    pub fn Register(
        &mut self,
        messageId: crate::System::Guid,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Guid,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityAction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Register")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection
                    as quest_hook::libil2cpp::Type > ::class(), "Register", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (messageId, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterConnection(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Events::UnityAction_1<i32>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterConnection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection
                    as quest_hook::libil2cpp::Type > ::class(), "RegisterConnection",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDisconnection(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Events::UnityAction_1<i32>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterDisconnection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection
                    as quest_hook::libil2cpp::Type > ::class(), "RegisterDisconnection",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        messageId: crate::System::Guid,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Guid,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Send")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection
                    as quest_hook::libil2cpp::Type > ::class(), "Send", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (messageId, data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
