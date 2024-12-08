#[cfg(feature = "UnityEngine+Bindings+FreeFunctionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct FreeFunctionAttribute {
    __cordl_parent: crate::UnityEngine::Bindings::NativeMethodAttribute,
}
#[cfg(feature = "UnityEngine+Bindings+FreeFunctionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::FreeFunctionAttribute =>
    "UnityEngine.Bindings"."FreeFunctionAttribute"
);
#[cfg(feature = "UnityEngine+Bindings+FreeFunctionAttribute")]
impl std::ops::Deref for crate::UnityEngine::Bindings::FreeFunctionAttribute {
    type Target = crate::UnityEngine::Bindings::NativeMethodAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+FreeFunctionAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Bindings::FreeFunctionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+FreeFunctionAttribute")]
impl crate::UnityEngine::Bindings::FreeFunctionAttribute {
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String__cordl_bool2(
        &mut self,
        name: *mut crate::System::String,
        isThreadSafe: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, isThreadSafe))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn New_String__cordl_bool2(
        name: *mut crate::System::String,
        isThreadSafe: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, isThreadSafe))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Bindings+FreeFunctionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Bindings::FreeFunctionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
