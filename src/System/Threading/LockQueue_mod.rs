#[cfg(feature = "System+Threading+LockQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct LockQueue {
    __cordl_parent: crate::System::Object,
    pub rwlock: *mut crate::System::Threading::ReaderWriterLock,
    pub lockCount: i32,
}
#[cfg(feature = "System+Threading+LockQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::LockQueue =>
    "System.Threading"."LockQueue"
);
#[cfg(feature = "System+Threading+LockQueue")]
impl std::ops::Deref for crate::System::Threading::LockQueue {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LockQueue")]
impl std::ops::DerefMut for crate::System::Threading::LockQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LockQueue")]
impl crate::System::Threading::LockQueue {
    pub fn _ctor(
        &mut self,
        rwlock: *mut crate::System::Threading::ReaderWriterLock,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rwlock))?;
        Ok(__cordl_ret)
    }
    pub fn Wait(&mut self, timeout: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Wait", (timeout))?;
        Ok(__cordl_ret)
    }
    pub fn Pulse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pulse", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmpty", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        rwlock: *mut crate::System::Threading::ReaderWriterLock,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rwlock))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+LockQueue")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::LockQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
