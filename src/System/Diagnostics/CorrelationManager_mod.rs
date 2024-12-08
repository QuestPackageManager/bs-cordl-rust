#[cfg(feature = "System+Diagnostics+CorrelationManager")]
#[repr(C)]
#[derive(Debug)]
pub struct CorrelationManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Diagnostics+CorrelationManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::CorrelationManager =>
    "System.Diagnostics"."CorrelationManager"
);
#[cfg(feature = "System+Diagnostics+CorrelationManager")]
impl std::ops::Deref for crate::System::Diagnostics::CorrelationManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CorrelationManager")]
impl std::ops::DerefMut for crate::System::Diagnostics::CorrelationManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CorrelationManager")]
impl crate::System::Diagnostics::CorrelationManager {
    pub fn GetLogicalOperationStack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Stack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Stack = __cordl_object
            .invoke("GetLogicalOperationStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_LogicalOperationStack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Stack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Stack = __cordl_object
            .invoke("get_LogicalOperationStack", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+CorrelationManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::CorrelationManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
