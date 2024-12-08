#[cfg(feature = "UnityEngine+AndroidJavaProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJavaProxy {
    __cordl_parent: crate::System::Object,
    pub javaInterface: *mut crate::UnityEngine::AndroidJavaClass,
    pub proxyObject: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+AndroidJavaProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJavaProxy => "UnityEngine"
    ."AndroidJavaProxy"
);
#[cfg(feature = "UnityEngine+AndroidJavaProxy")]
impl std::ops::Deref for crate::UnityEngine::AndroidJavaProxy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaProxy")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJavaProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaProxy")]
impl crate::UnityEngine::AndroidJavaProxy {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn equals(
        &mut self,
        obj: *mut crate::UnityEngine::AndroidJavaObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetProxyObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AndroidJavaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AndroidJavaObject = __cordl_object
            .invoke("GetProxyObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRawProxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetRawProxy", ())?;
        Ok(__cordl_ret)
    }
    pub fn toString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("toString", ())?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_Il2CppArray0(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AndroidJavaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AndroidJavaObject = __cordl_object
            .invoke("Invoke", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_Il2CppArray1(
        &mut self,
        methodName: *mut crate::System::String,
        javaArgs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AndroidJavaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AndroidJavaObject = __cordl_object
            .invoke("Invoke", (methodName, javaArgs))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_IntPtr2(
        &mut self,
        methodName: *mut crate::System::String,
        javaArgs: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (methodName, javaArgs))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        javaInterface: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (javaInterface))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AndroidJavaClass1(
        &mut self,
        javaInterface: *mut crate::UnityEngine::AndroidJavaClass,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (javaInterface))?;
        Ok(__cordl_ret)
    }
    pub fn hashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("hashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        javaInterface: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (javaInterface))?;
        Ok(__cordl_object)
    }
    pub fn New_AndroidJavaClass1(
        javaInterface: *mut crate::UnityEngine::AndroidJavaClass,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (javaInterface))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaProxy")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJavaProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
