#[cfg(feature = "UnityEngine+AndroidJavaObject")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJavaObject {
    __cordl_parent: crate::System::Object,
    pub m_jobject: *mut crate::UnityEngine::GlobalJavaObjectRef,
    pub m_jclass: *mut crate::UnityEngine::GlobalJavaObjectRef,
}
#[cfg(feature = "UnityEngine+AndroidJavaObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJavaObject => "UnityEngine"
    ."AndroidJavaObject"
);
#[cfg(feature = "UnityEngine+AndroidJavaObject")]
impl std::ops::Deref for crate::UnityEngine::AndroidJavaObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaObject")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJavaObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaObject")]
impl crate::UnityEngine::AndroidJavaObject {
    pub fn _CallStatic_String0(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_CallStatic", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn _CallStatic_IntPtr1(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_CallStatic", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn _CallStatic_String2<ReturnType>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object
            .invoke("_CallStatic", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn _CallStatic_IntPtr3<ReturnType>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object
            .invoke("_CallStatic", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn DebugPrint_String0(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DebugPrint", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn DebugPrint_String_String_Il2CppArray1(
        &mut self,
        call: *mut crate::System::String,
        methodName: *mut crate::System::String,
        signature: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DebugPrint", (call, methodName, signature, args))?;
        Ok(__cordl_ret)
    }
    pub fn GetRawClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetRawClass", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray0(
        &mut self,
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (className, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray1(
        &mut self,
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (className, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray2(
        &mut self,
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaClass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (className, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray3(
        &mut self,
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaProxy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (className, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray4(
        &mut self,
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaRunnable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (className, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray5(
        &mut self,
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (className, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IntPtr6(
        &mut self,
        jobject: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (jobject))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IntPtr_IntPtr_Il2CppArray7(
        &mut self,
        clazz: crate::System::IntPtr,
        constructorID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clazz, constructorID, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_8(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetStatic_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("GetStatic", (fieldName))?;
        Ok(__cordl_ret)
    }
    pub fn GetStatic_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("GetStatic", (fieldID))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn CloneReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AndroidJavaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AndroidJavaObject = __cordl_object
            .invoke("CloneReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn _Call_String0(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_Call", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn _Call_IntPtr1(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_Call", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn _Call_String2<ReturnType>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object
            .invoke("_Call", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn _Call_IntPtr3<ReturnType>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object.invoke("_Call", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn _SetStatic_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_SetStatic", (fieldName, val))?;
        Ok(__cordl_ret)
    }
    pub fn _SetStatic_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_SetStatic", (fieldID, val))?;
        Ok(__cordl_ret)
    }
    pub fn _AndroidJavaObject_String0(
        &mut self,
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_AndroidJavaObject", (className, args))?;
        Ok(__cordl_ret)
    }
    pub fn _AndroidJavaObject_IntPtr1(
        &mut self,
        constructorID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_AndroidJavaObject", (constructorID, args))?;
        Ok(__cordl_ret)
    }
    pub fn _GetStatic_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("_GetStatic", (fieldName))?;
        Ok(__cordl_ret)
    }
    pub fn _GetStatic_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("_GetStatic", (fieldID))?;
        Ok(__cordl_ret)
    }
    pub fn _GetRawObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("_GetRawObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn Call_String0<T>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Call", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn Call_IntPtr1<T>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Call", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn Call_String2(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Call", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn Call_IntPtr3(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Call", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn Call_String4<ReturnType, T>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object.invoke("Call", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn Call_IntPtr5<ReturnType, T>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object.invoke("Call", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn Call_String6<ReturnType>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object.invoke("Call", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn Call_IntPtr7<ReturnType>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object.invoke("Call", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn SetStatic_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStatic", (fieldName, val))?;
        Ok(__cordl_ret)
    }
    pub fn SetStatic_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStatic", (fieldID, val))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_String0<T>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallStatic", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_IntPtr1<T>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallStatic", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_String2(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallStatic", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_IntPtr3(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CallStatic", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_String4<ReturnType, T>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object
            .invoke("CallStatic", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_IntPtr5<ReturnType, T>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object
            .invoke("CallStatic", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_String6<ReturnType>(
        &mut self,
        methodName: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object
            .invoke("CallStatic", (methodName, args))?;
        Ok(__cordl_ret)
    }
    pub fn CallStatic_IntPtr7<ReturnType>(
        &mut self,
        methodID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<ReturnType>
    where
        ReturnType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ReturnType = __cordl_object
            .invoke("CallStatic", (methodID, args))?;
        Ok(__cordl_ret)
    }
    pub fn _Get_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("_Get", (fieldName))?;
        Ok(__cordl_ret)
    }
    pub fn _Get_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("_Get", (fieldID))?;
        Ok(__cordl_ret)
    }
    pub fn _Set_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_Set", (fieldName, val))?;
        Ok(__cordl_ret)
    }
    pub fn _Set_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_Set", (fieldID, val))?;
        Ok(__cordl_ret)
    }
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
    pub fn Set_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (fieldName, val))?;
        Ok(__cordl_ret)
    }
    pub fn Set_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
        val: FieldType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (fieldID, val))?;
        Ok(__cordl_ret)
    }
    pub fn GetRawObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetRawObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn Get_String0<FieldType>(
        &mut self,
        fieldName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("Get", (fieldName))?;
        Ok(__cordl_ret)
    }
    pub fn Get_IntPtr1<FieldType>(
        &mut self,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<FieldType>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: FieldType = __cordl_object.invoke("Get", (fieldID))?;
        Ok(__cordl_ret)
    }
    pub fn _GetRawClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("_GetRawClass", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String_Il2CppArray0(
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (className, args))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray1(
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (className, args))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray2(
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaClass,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (className, args))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray3(
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaProxy,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (className, args))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray4(
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::AndroidJavaRunnable,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (className, args))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray5(
        className: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (className, args))?;
        Ok(__cordl_object)
    }
    pub fn New_IntPtr6(
        jobject: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (jobject))?;
        Ok(__cordl_object)
    }
    pub fn New_IntPtr_IntPtr_Il2CppArray7(
        clazz: crate::System::IntPtr,
        constructorID: crate::System::IntPtr,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (clazz, constructorID, args))?;
        Ok(__cordl_object)
    }
    pub fn New_8() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaObject")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJavaObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
