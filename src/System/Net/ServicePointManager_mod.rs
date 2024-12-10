#[cfg(feature = "System+Net+ServicePointManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePointManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+ServicePointManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ServicePointManager => "System.Net"
    ."ServicePointManager"
);
#[cfg(feature = "System+Net+ServicePointManager")]
impl std::ops::Deref for crate::System::Net::ServicePointManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointManager")]
impl std::ops::DerefMut for crate::System::Net::ServicePointManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointManager")]
impl crate::System::Net::ServicePointManager {
    #[cfg(feature = "System+Net+ServicePointManager+SPKey")]
    pub type SPKey = crate::System::Net::ServicePointManager_SPKey;
}
#[cfg(feature = "System+Net+ServicePointManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ServicePointManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+ServicePointManager+SPKey")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePointManager_SPKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub uri: *mut crate::System::Uri,
    pub proxy: *mut crate::System::Uri,
    pub use_connect: bool,
}
#[cfg(feature = "System+Net+ServicePointManager+SPKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ServicePointManager_SPKey =>
    "System.Net"."ServicePointManager/SPKey"
);
#[cfg(feature = "System+Net+ServicePointManager+SPKey")]
impl std::ops::Deref for crate::System::Net::ServicePointManager_SPKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointManager+SPKey")]
impl std::ops::DerefMut for crate::System::Net::ServicePointManager_SPKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePointManager+SPKey")]
impl crate::System::Net::ServicePointManager_SPKey {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        proxy: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        use_connect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uri, proxy, use_connect))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        proxy: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        use_connect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uri, proxy, use_connect))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UsesProxy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UsesProxy", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ServicePointManager+SPKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::ServicePointManager_SPKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
