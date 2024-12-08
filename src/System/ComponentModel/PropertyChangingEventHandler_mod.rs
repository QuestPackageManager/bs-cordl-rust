#[cfg(feature = "System+ComponentModel+PropertyChangingEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyChangingEventHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::PropertyChangingEventHandler => "System.ComponentModel"
    ."PropertyChangingEventHandler"
);
#[cfg(feature = "System+ComponentModel+PropertyChangingEventHandler")]
impl std::ops::Deref for crate::System::ComponentModel::PropertyChangingEventHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventHandler")]
impl std::ops::DerefMut for crate::System::ComponentModel::PropertyChangingEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventHandler")]
impl crate::System::ComponentModel::PropertyChangingEventHandler {
    pub fn Invoke(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::ComponentModel::PropertyChangingEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::PropertyChangingEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
