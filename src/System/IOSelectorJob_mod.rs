#[cfg(feature = "System+IOSelectorJob")]
#[repr(C)]
#[derive(Debug)]
pub struct IOSelectorJob {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub operation: crate::System::IOOperation,
    pub callback: quest_hook::libil2cpp::Gc<crate::System::IOAsyncCallback>,
    pub state: quest_hook::libil2cpp::Gc<crate::System::IOAsyncResult>,
}
#[cfg(feature = "System+IOSelectorJob")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IOSelectorJob {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "IOSelectorJob";
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
#[cfg(feature = "System+IOSelectorJob")]
impl std::ops::Deref for crate::System::IOSelectorJob {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl std::ops::DerefMut for crate::System::IOSelectorJob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl crate::System::IOSelectorJob {
    pub fn MarkDisposed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MarkDisposed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MarkDisposed", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        operation: crate::System::IOOperation,
        callback: quest_hook::libil2cpp::Gc<crate::System::IOAsyncCallback>,
        state: quest_hook::libil2cpp::Gc<crate::System::IOAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (operation, callback, state))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Threading_IThreadPoolWorkItem_ExecuteWorkItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("System.Threading.IThreadPoolWorkItem.ExecuteWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Threading.IThreadPoolWorkItem.ExecuteWorkItem", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Threading_IThreadPoolWorkItem_MarkAborted(
        &mut self,
        tae: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadAbortException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Threading::ThreadAbortException,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("System.Threading.IThreadPoolWorkItem.MarkAborted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Threading.IThreadPoolWorkItem.MarkAborted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tae))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        operation: crate::System::IOOperation,
        callback: quest_hook::libil2cpp::Gc<crate::System::IOAsyncCallback>,
        state: quest_hook::libil2cpp::Gc<crate::System::IOAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::IOOperation,
                    quest_hook::libil2cpp::Gc<crate::System::IOAsyncCallback>,
                    quest_hook::libil2cpp::Gc<crate::System::IOAsyncResult>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (operation, callback, state))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IOSelectorJob {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl AsRef<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::IOSelectorJob {
    fn as_ref(&self) -> &crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IOSelectorJob")]
impl AsMut<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::IOSelectorJob {
    fn as_mut(&mut self) -> &mut crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
