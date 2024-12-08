#[cfg(feature = "Zenject+ProfileBlock")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfileBlock {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+ProfileBlock")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ProfileBlock => "Zenject"
    ."ProfileBlock"
);
#[cfg(feature = "Zenject+ProfileBlock")]
impl std::ops::Deref for crate::Zenject::ProfileBlock {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl std::ops::DerefMut for crate::Zenject::ProfileBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl crate::Zenject::ProfileBlock {
    pub fn _ctor__cordl_bool0(
        &mut self,
        sampleName: *mut crate::System::String,
        rootBlock: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sampleName, rootBlock))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        sampleName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sampleName))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool0(
        sampleName: *mut crate::System::String,
        rootBlock: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sampleName, rootBlock))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        sampleName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sampleName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ProfileBlock")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ProfileBlock {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
