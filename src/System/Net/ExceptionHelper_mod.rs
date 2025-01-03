#[cfg(feature = "System+Net+ExceptionHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+ExceptionHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ExceptionHelper => "System.Net"
    ."ExceptionHelper"
);
#[cfg(feature = "System+Net+ExceptionHelper")]
impl std::ops::Deref for crate::System::Net::ExceptionHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ExceptionHelper")]
impl std::ops::DerefMut for crate::System::Net::ExceptionHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ExceptionHelper")]
impl crate::System::Net::ExceptionHelper {
    pub fn get_MethodNotImplementedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::NotImplementedException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::NotImplementedException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MethodNotImplementedException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertyNotImplementedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::NotImplementedException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::NotImplementedException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PropertyNotImplementedException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertyNotSupportedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::NotSupportedException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::NotSupportedException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PropertyNotSupportedException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequestAbortedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebException> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_RequestAbortedException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeoutException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebException> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TimeoutException", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ExceptionHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ExceptionHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
