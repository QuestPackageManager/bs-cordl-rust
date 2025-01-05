#[cfg(feature = "System+Runtime+Versioning+TargetFrameworkAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TargetFrameworkAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub _frameworkName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _frameworkDisplayName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Runtime+Versioning+TargetFrameworkAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Versioning::TargetFrameworkAttribute =>
    "System.Runtime.Versioning"."TargetFrameworkAttribute"
);
#[cfg(feature = "System+Runtime+Versioning+TargetFrameworkAttribute")]
impl std::ops::Deref for crate::System::Runtime::Versioning::TargetFrameworkAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Versioning+TargetFrameworkAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Versioning::TargetFrameworkAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Versioning+TargetFrameworkAttribute")]
impl crate::System::Runtime::Versioning::TargetFrameworkAttribute {
    pub fn New(
        frameworkName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (frameworkName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        frameworkName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (frameworkName))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FrameworkDisplayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FrameworkDisplayName", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Versioning+TargetFrameworkAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Versioning::TargetFrameworkAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
