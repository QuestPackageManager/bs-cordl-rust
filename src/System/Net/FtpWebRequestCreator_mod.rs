#[cfg(feature = "System+Net+FtpWebRequestCreator")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpWebRequestCreator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+FtpWebRequestCreator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpWebRequestCreator =>
    "System.Net"."FtpWebRequestCreator"
);
#[cfg(feature = "System+Net+FtpWebRequestCreator")]
impl std::ops::Deref for crate::System::Net::FtpWebRequestCreator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebRequestCreator")]
impl std::ops::DerefMut for crate::System::Net::FtpWebRequestCreator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebRequestCreator")]
impl crate::System::Net::FtpWebRequestCreator {
    pub fn Create(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest> = __cordl_object
            .invoke("Create", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+FtpWebRequestCreator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FtpWebRequestCreator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+FtpWebRequestCreator")]
impl AsRef<crate::System::Net::IWebRequestCreate>
for crate::System::Net::FtpWebRequestCreator {
    fn as_ref(&self) -> &crate::System::Net::IWebRequestCreate {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+FtpWebRequestCreator")]
impl AsMut<crate::System::Net::IWebRequestCreate>
for crate::System::Net::FtpWebRequestCreator {
    fn as_mut(&mut self) -> &mut crate::System::Net::IWebRequestCreate {
        unsafe { std::mem::transmute(self) }
    }
}
