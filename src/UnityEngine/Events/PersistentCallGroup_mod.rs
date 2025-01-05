#[cfg(feature = "UnityEngine+Events+PersistentCallGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct PersistentCallGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Calls: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::PersistentCall>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+Events+PersistentCallGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::PersistentCallGroup =>
    "UnityEngine.Events"."PersistentCallGroup"
);
#[cfg(feature = "UnityEngine+Events+PersistentCallGroup")]
impl std::ops::Deref for crate::UnityEngine::Events::PersistentCallGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+PersistentCallGroup")]
impl std::ops::DerefMut for crate::UnityEngine::Events::PersistentCallGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+PersistentCallGroup")]
impl crate::UnityEngine::Events::PersistentCallGroup {
    pub fn Initialize(
        &mut self,
        invokableList: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::InvokableCallList,
        >,
        unityEventBase: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (invokableList, unityEventBase))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Events+PersistentCallGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Events::PersistentCallGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
