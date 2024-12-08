#[cfg(feature = "System+ComponentModel+RefreshEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct RefreshEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _TypeChanged_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "System+ComponentModel+RefreshEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::RefreshEventArgs =>
    "System.ComponentModel"."RefreshEventArgs"
);
#[cfg(feature = "System+ComponentModel+RefreshEventArgs")]
impl std::ops::Deref for crate::System::ComponentModel::RefreshEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+RefreshEventArgs")]
impl std::ops::DerefMut for crate::System::ComponentModel::RefreshEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+RefreshEventArgs")]
impl crate::System::ComponentModel::RefreshEventArgs {
    pub fn New(
        typeChanged: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeChanged))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        typeChanged: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeChanged))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+RefreshEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::RefreshEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}