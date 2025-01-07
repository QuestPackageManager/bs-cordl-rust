#[cfg(feature = "System+Threading+ThreadHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _start: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    pub _startArg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _executionContext: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ExecutionContext,
    >,
}
#[cfg(feature = "System+Threading+ThreadHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::ThreadHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "ThreadHelper";
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
#[cfg(feature = "System+Threading+ThreadHelper")]
impl std::ops::Deref for crate::System::Threading::ThreadHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        start: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start))?;
        Ok(__cordl_object.into())
    }
    pub fn SetExecutionContextHelper(
        &mut self,
        ec: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExecutionContextHelper", (ec))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThreadStart_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThreadStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThreadStart_Context(
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThreadStart_Context", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThreadStart_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThreadStart", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        start: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start))?;
        Ok(__cordl_ret.into())
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
