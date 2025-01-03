#[cfg(feature = "System+Platform")]
#[repr(C)]
#[derive(Debug)]
pub struct Platform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Platform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Platform => "System"."Platform"
);
#[cfg(feature = "System+Platform")]
impl std::ops::Deref for crate::System::Platform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Platform")]
impl std::ops::DerefMut for crate::System::Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Platform")]
impl crate::System::Platform {
    pub fn CheckOS() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckOS", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAix() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsAix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFreeBSD() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsFreeBSD", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsIBMi() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsIBMi", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsMacOS() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsMacOS", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsOpenBSD() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsOpenBSD", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn uname(buf: crate::System::IntPtr) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uname", (buf))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Platform")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Platform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
