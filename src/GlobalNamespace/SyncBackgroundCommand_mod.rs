#[cfg(feature = "SyncBackgroundCommand")]
#[repr(C)]
#[derive(Debug)]
pub struct SyncBackgroundCommand {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _taskCompletionSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::TaskCompletionSource_1<i32>,
    >,
}
#[cfg(feature = "SyncBackgroundCommand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SyncBackgroundCommand => ""
    ."SyncBackgroundCommand"
);
#[cfg(feature = "SyncBackgroundCommand")]
impl std::ops::Deref for crate::GlobalNamespace::SyncBackgroundCommand {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SyncBackgroundCommand")]
impl std::ops::DerefMut for crate::GlobalNamespace::SyncBackgroundCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SyncBackgroundCommand")]
impl crate::GlobalNamespace::SyncBackgroundCommand {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("Execute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteInternal", ())?;
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
    pub fn get_resultTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("get_resultTask", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SyncBackgroundCommand")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SyncBackgroundCommand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SyncBackgroundCommand")]
impl AsRef<crate::GlobalNamespace::IBackgroundCommand>
for crate::GlobalNamespace::SyncBackgroundCommand {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBackgroundCommand {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SyncBackgroundCommand")]
impl AsMut<crate::GlobalNamespace::IBackgroundCommand>
for crate::GlobalNamespace::SyncBackgroundCommand {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBackgroundCommand {
        unsafe { std::mem::transmute(self) }
    }
}
