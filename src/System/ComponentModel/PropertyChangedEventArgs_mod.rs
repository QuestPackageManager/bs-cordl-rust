#[cfg(feature = "System+ComponentModel+PropertyChangedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyChangedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+ComponentModel+PropertyChangedEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::PropertyChangedEventArgs
    => "System.ComponentModel"."PropertyChangedEventArgs"
);
#[cfg(feature = "System+ComponentModel+PropertyChangedEventArgs")]
impl std::ops::Deref for crate::System::ComponentModel::PropertyChangedEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangedEventArgs")]
impl std::ops::DerefMut for crate::System::ComponentModel::PropertyChangedEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangedEventArgs")]
impl crate::System::ComponentModel::PropertyChangedEventArgs {
    pub fn New(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+PropertyChangedEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::PropertyChangedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
