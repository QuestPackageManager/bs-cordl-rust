#[cfg(feature = "System+Threading+InternalThread")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalThread {
    __cordl_parent: crate::System::Runtime::ConstrainedExecution::CriticalFinalizerObject,
    pub lock_thread_id: i32,
    pub handle: crate::System::IntPtr,
    pub native_handle: crate::System::IntPtr,
    pub name_chars: crate::System::IntPtr,
    pub name_free: i32,
    pub name_length: i32,
    pub state: crate::System::Threading::ThreadState,
    pub abort_exc: *mut crate::System::Object,
    pub abort_state_handle: i32,
    pub thread_id: i64,
    pub debugger_thread: crate::System::IntPtr,
    pub static_data: crate::System::UIntPtr,
    pub runtime_thread_info: crate::System::IntPtr,
    pub current_appcontext: *mut crate::System::Object,
    pub root_domain_thread: *mut crate::System::Object,
    pub _serialized_principal: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _serialized_principal_version: i32,
    pub appdomain_refs: crate::System::IntPtr,
    pub interruption_requested: i32,
    pub longlived: crate::System::IntPtr,
    pub threadpool_thread: bool,
    pub thread_interrupt_requested: bool,
    pub stack_size: i32,
    pub apartment_state: u8,
    pub critical_region_level: i32,
    pub managed_id: i32,
    pub small_id: i32,
    pub manage_callback: crate::System::IntPtr,
    pub flags: crate::System::IntPtr,
    pub thread_pinning_ref: crate::System::IntPtr,
    pub abort_protected_block_count: crate::System::IntPtr,
    pub priority: i32,
    pub owned_mutex: crate::System::IntPtr,
    pub suspended_event: crate::System::IntPtr,
    pub self_suspended: i32,
    pub thread_state: crate::System::IntPtr,
    pub netcore0: crate::System::IntPtr,
    pub netcore1: crate::System::IntPtr,
    pub netcore2: crate::System::IntPtr,
    pub last: crate::System::IntPtr,
}
#[cfg(feature = "System+Threading+InternalThread")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::InternalThread =>
    "System.Threading"."InternalThread"
);
#[cfg(feature = "System+Threading+InternalThread")]
impl std::ops::Deref for crate::System::Threading::InternalThread {
    type Target = crate::System::Runtime::ConstrainedExecution::CriticalFinalizerObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+InternalThread")]
impl std::ops::DerefMut for crate::System::Threading::InternalThread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+InternalThread")]
impl crate::System::Threading::InternalThread {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Thread_free_internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Thread_free_internal", ())?;
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
}
#[cfg(feature = "System+Threading+InternalThread")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::InternalThread {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}