#[cfg(feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader")]
#[repr(C)]
#[derive(Debug)]
pub struct FeatureAsyncPreloader {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AsyncPreloader,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::AppFlow::Initialization::FeatureAsyncPreloader =>
    "BGLib.AppFlow.Initialization"."FeatureAsyncPreloader"
);
#[cfg(feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader")]
impl std::ops::Deref for crate::BGLib::AppFlow::Initialization::FeatureAsyncPreloader {
    type Target = crate::BGLib::AppFlow::Initialization::AsyncPreloader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader")]
impl std::ops::DerefMut
for crate::BGLib::AppFlow::Initialization::FeatureAsyncPreloader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader")]
impl crate::BGLib::AppFlow::Initialization::FeatureAsyncPreloader {
    pub const kFeatureAsyncPreloader: &'static str = "FeatureAsyncPreloader";
    #[cfg(
        feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader+_PreloadAsync_d__1"
    )]
    pub type _PreloadAsync_d__1 = crate::BGLib::AppFlow::Initialization::FeatureAsyncPreloader__PreloadAsync_d__1;
    #[cfg(feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader+__c")]
    pub type __c = crate::BGLib::AppFlow::Initialization::FeatureAsyncPreloader___c;
    pub fn PreloadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("PreloadAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+FeatureAsyncPreloader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::FeatureAsyncPreloader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
