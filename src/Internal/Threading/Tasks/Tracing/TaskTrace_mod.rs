#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskTrace {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Threading::Tasks::Tracing::TaskTrace
    => "Internal.Threading.Tasks.Tracing"."TaskTrace"
);
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl std::ops::Deref for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl std::ops::DerefMut for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl crate::Internal::Threading::Tasks::Tracing::TaskTrace {}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}