#[cfg(feature = "System+Threading+IAsyncLocal")]
#[repr(C)]
#[derive(Debug)]
pub struct IAsyncLocal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+IAsyncLocal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::IAsyncLocal =>
    "System.Threading"."IAsyncLocal"
);
#[cfg(feature = "System+Threading+IAsyncLocal")]
impl std::ops::Deref for crate::System::Threading::IAsyncLocal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+IAsyncLocal")]
impl std::ops::DerefMut for crate::System::Threading::IAsyncLocal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+IAsyncLocal")]
impl crate::System::Threading::IAsyncLocal {
    pub fn OnValueChanged(
        &mut self,
        previousValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        currentValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contextChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValueChanged", (previousValue, currentValue, contextChanged))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Threading+IAsyncLocal")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::IAsyncLocal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
