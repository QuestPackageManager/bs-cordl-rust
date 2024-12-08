#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource")]
#[repr(C)]
#[derive(Debug)]
pub struct IValueTaskSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::Sources::IValueTaskSource =>
    "System.Threading.Tasks.Sources"."IValueTaskSource"
);
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource")]
impl std::ops::Deref for crate::System::Threading::Tasks::Sources::IValueTaskSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::Sources::IValueTaskSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource")]
impl crate::System::Threading::Tasks::Sources::IValueTaskSource {
    pub fn GetStatus(
        &mut self,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::Sources::ValueTaskSourceStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::Sources::ValueTaskSourceStatus = __cordl_object
            .invoke("GetStatus", (token))?;
        Ok(__cordl_ret)
    }
    pub fn GetResult(
        &mut self,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetResult", (token))?;
        Ok(__cordl_ret)
    }
    pub fn OnCompleted(
        &mut self,
        continuation: *mut crate::System::Action_1<*mut crate::System::Object>,
        state: *mut crate::System::Object,
        token: i16,
        flags: crate::System::Threading::Tasks::Sources::ValueTaskSourceOnCompletedFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleted", (continuation, state, token, flags))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::Sources::IValueTaskSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
