#[cfg(feature = "System+Security+Principal+WindowsImpersonationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsImpersonationContext {
    __cordl_parent: crate::System::Object,
    pub _token: crate::System::IntPtr,
    pub undo: bool,
}
#[cfg(feature = "System+Security+Principal+WindowsImpersonationContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Principal::WindowsImpersonationContext =>
    "System.Security.Principal"."WindowsImpersonationContext"
);
#[cfg(feature = "System+Security+Principal+WindowsImpersonationContext")]
impl std::ops::Deref
for crate::System::Security::Principal::WindowsImpersonationContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsImpersonationContext")]
impl std::ops::DerefMut
for crate::System::Security::Principal::WindowsImpersonationContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsImpersonationContext")]
impl crate::System::Security::Principal::WindowsImpersonationContext {
    pub fn _ctor(
        &mut self,
        token: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token))?;
        Ok(__cordl_ret)
    }
    pub fn Undo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Undo", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        token: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Principal+WindowsImpersonationContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Principal::WindowsImpersonationContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
