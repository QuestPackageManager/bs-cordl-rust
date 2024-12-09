#[cfg(feature = "System+Net+Sockets+LingerOption")]
#[repr(C)]
#[derive(Debug)]
pub struct LingerOption {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub enabled: bool,
    pub lingerTime: i32,
}
#[cfg(feature = "System+Net+Sockets+LingerOption")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::LingerOption =>
    "System.Net.Sockets"."LingerOption"
);
#[cfg(feature = "System+Net+Sockets+LingerOption")]
impl std::ops::Deref for crate::System::Net::Sockets::LingerOption {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+LingerOption")]
impl std::ops::DerefMut for crate::System::Net::Sockets::LingerOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+LingerOption")]
impl crate::System::Net::Sockets::LingerOption {
    pub fn New(enable: bool, seconds: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (enable, seconds))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        enable: bool,
        seconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (enable, seconds))?;
        Ok(__cordl_ret)
    }
    pub fn set_Enabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Enabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_LingerTime(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LingerTime", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Sockets+LingerOption")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Sockets::LingerOption {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
