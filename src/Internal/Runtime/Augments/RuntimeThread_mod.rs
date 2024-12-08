#[cfg(feature = "Internal+Runtime+Augments+RuntimeThread")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeThread {
    __cordl_parent: crate::System::Object,
    pub thread: *mut crate::System::Threading::Thread,
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeThread")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Runtime::Augments::RuntimeThread =>
    "Internal.Runtime.Augments"."RuntimeThread"
);
#[cfg(feature = "Internal+Runtime+Augments+RuntimeThread")]
impl std::ops::Deref for crate::Internal::Runtime::Augments::RuntimeThread {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeThread")]
impl std::ops::DerefMut for crate::Internal::Runtime::Augments::RuntimeThread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeThread")]
impl crate::Internal::Runtime::Augments::RuntimeThread {
    pub fn New(
        t: *mut crate::System::Threading::Thread,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (t))?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (state))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        t: *mut crate::System::Threading::Thread,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (t))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsBackground(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsBackground", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeThread")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Runtime::Augments::RuntimeThread {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
