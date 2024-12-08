#[cfg(feature = "System+ComponentModel+PropertyChangingEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyChangingEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _propertyName: *mut crate::System::String,
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::PropertyChangingEventArgs => "System.ComponentModel"
    ."PropertyChangingEventArgs"
);
#[cfg(feature = "System+ComponentModel+PropertyChangingEventArgs")]
impl std::ops::Deref for crate::System::ComponentModel::PropertyChangingEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventArgs")]
impl std::ops::DerefMut for crate::System::ComponentModel::PropertyChangingEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventArgs")]
impl crate::System::ComponentModel::PropertyChangingEventArgs {
    pub fn _ctor(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangingEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::PropertyChangingEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
