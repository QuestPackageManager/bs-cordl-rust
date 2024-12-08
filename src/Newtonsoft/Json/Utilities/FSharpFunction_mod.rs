#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct FSharpFunction {
    __cordl_parent: crate::System::Object,
    pub _instance: *mut crate::System::Object,
    pub _invoker: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::FSharpFunction =>
    "Newtonsoft.Json.Utilities"."FSharpFunction"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpFunction")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::FSharpFunction {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpFunction")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::FSharpFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpFunction")]
impl crate::Newtonsoft::Json::Utilities::FSharpFunction {
    pub fn Invoke(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Invoke", (args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        instance: *mut crate::System::Object,
        invoker: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (instance, invoker))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        instance: *mut crate::System::Object,
        invoker: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (instance, invoker))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::FSharpFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
