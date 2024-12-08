#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
#[repr(C)]
#[derive(Debug)]
pub struct CallSite {
    __cordl_parent: crate::System::Object,
    pub _binder: *mut crate::System::Runtime::CompilerServices::CallSiteBinder,
    pub _match: bool,
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::CallSite =>
    "System.Runtime.CompilerServices"."CallSite"
);
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::CallSite {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::CallSite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl crate::System::Runtime::CompilerServices::CallSite {
    pub fn get_Binder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::CompilerServices::CallSiteBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::CompilerServices::CallSiteBinder = __cordl_object
            .invoke("get_Binder", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        binder: *mut crate::System::Runtime::CompilerServices::CallSiteBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        binder: *mut crate::System::Runtime::CompilerServices::CallSiteBinder,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binder))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::CallSite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
