#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct _AndroidJNIHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::_AndroidJNIHelper => "UnityEngine"
    ."_AndroidJNIHelper"
);
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl std::ops::Deref for crate::UnityEngine::_AndroidJNIHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl std::ops::DerefMut for crate::UnityEngine::_AndroidJNIHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl crate::UnityEngine::_AndroidJNIHelper {
    pub fn Box(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AndroidJavaObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Box", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertFromJNIArray<ArrayType>(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<ArrayType>
    where
        ArrayType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: ArrayType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertFromJNIArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToJNIArray(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertToJNIArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJNIArgArray(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        ret: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJNIArgArray", (args, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJavaProxy(
        player: crate::System::IntPtr,
        delegateHandle: crate::System::IntPtr,
        proxy: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaProxy>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJavaProxy", (player, delegateHandle, proxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJavaRunnable(
        jrunnable: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaRunnable>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJavaRunnable", (jrunnable))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteJNIArgArray(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        jniArgs: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteJNIArgArray", (args, jniArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConstructorID_Il2CppArray0(
        jclass: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConstructorID", (jclass, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConstructorID_Il2CppString1(
        jclass: crate::System::IntPtr,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConstructorID", (jclass, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID_Il2CppString__cordl_bool1(
        jclass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldID", (jclass, fieldName, signature, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID__cordl_bool0<ReturnType>(
        jclass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldID", (jclass, fieldName, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodIDFallback(
        jclass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodIDFallback", (jclass, methodName, signature, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID_Il2CppArray0(
        jclass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (jclass, methodName, args, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID_Il2CppArray1<ReturnType>(
        jclass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (jclass, methodName, args, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID_Il2CppString2(
        jclass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (jclass, methodName, signature, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignature_Il2CppArray1(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignature", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignature_Il2CppArray2<ReturnType>(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignature", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignature_Il2CppObject0(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignature", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeJavaProxyMethod(
        proxy: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaProxy>,
        jmethodName: crate::System::IntPtr,
        jargs: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeJavaProxyMethod", (proxy, jmethodName, jargs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Unbox(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Unbox", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnboxArray(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("UnboxArray", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::_AndroidJNIHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
