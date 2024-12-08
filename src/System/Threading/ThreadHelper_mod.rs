#[cfg(feature = "System+Threading+ThreadHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadHelper {
    __cordl_parent: crate::System::Object,
    pub _start: *mut crate::System::Delegate,
    pub _startArg: *mut crate::System::Object,
    pub _executionContext: *mut crate::System::Threading::ExecutionContext,
}
#[cfg(feature = "System+Threading+ThreadHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ThreadHelper =>
    "System.Threading"."ThreadHelper"
);
#[cfg(feature = "System+Threading+ThreadHelper")]
impl std::ops::Deref for crate::System::Threading::ThreadHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadHelper")]
impl std::ops::DerefMut for crate::System::Threading::ThreadHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadHelper")]
impl crate::System::Threading::ThreadHelper {
    pub fn New(
        start: *mut crate::System::Delegate,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start))?;
        Ok(__cordl_object)
    }
    pub fn SetExecutionContextHelper(
        &mut self,
        ec: *mut crate::System::Threading::ExecutionContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExecutionContextHelper", (ec))?;
        Ok(__cordl_ret)
    }
    pub fn ThreadStart_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThreadStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThreadStart_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThreadStart", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        start: *mut crate::System::Delegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+ThreadHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ThreadHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
