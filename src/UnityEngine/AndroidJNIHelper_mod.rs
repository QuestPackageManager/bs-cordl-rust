#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNIHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AndroidJNIHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AndroidJNIHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNIHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNIHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl crate::UnityEngine::AndroidJNIHelper {
    pub fn Box__cordl_bool8(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box__cordl_char7(
        value: char,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_f32_5(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_f64_6(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_i16_2(
        value: i16,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_i32_3(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_i64_4(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_i8_1(value: i8) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Box_jvalue_Il2CppString_Il2CppString0(
        val: crate::UnityEngine::jvalue,
        boxedClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Box", (val, boxedClass, signature))?;
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
    pub fn CreateJNIArgArray_Il2CppArray0(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJNIArgArray", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJNIArgArray_Span_1_1(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        jniArgs: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJNIArgArray", (args, jniArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJavaProxy(
        proxy: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaProxy>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJavaProxy", (proxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJavaRunnable(
        jrunnable: quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaRunnable>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJavaRunnable", (jrunnable))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteJNIArgArray_Il2CppArray0(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        jniArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteJNIArgArray", (args, jniArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteJNIArgArray_Span_1_1(
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
    pub fn GetConstructorID_Il2CppArray2(
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
        javaClass: crate::System::IntPtr,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConstructorID", (javaClass, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConstructorID_IntPtr0(
        javaClass: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConstructorID", (javaClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID_Il2CppString1(
        javaClass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldID", (javaClass, fieldName, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID_Il2CppString__cordl_bool2(
        javaClass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldID", (javaClass, fieldName, signature, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID_IntPtr_Il2CppString0(
        javaClass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldID", (javaClass, fieldName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID__cordl_bool3<FieldType>(
        jclass: crate::System::IntPtr,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        FieldType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldID", (jclass, fieldName, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID_Il2CppArray__cordl_bool3(
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
    pub fn GetMethodID_Il2CppArray__cordl_bool4<ReturnType>(
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
    pub fn GetMethodID_Il2CppString1(
        javaClass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (javaClass, methodName, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID_Il2CppString__cordl_bool2(
        javaClass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (javaClass, methodName, signature, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID_IntPtr_Il2CppString0(
        javaClass: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (javaClass, methodName))?;
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
    pub fn GetUnboxMethod(
        obj: crate::System::IntPtr,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnboxMethod", (obj, methodName, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut0(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<i8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut1(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<i16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut2(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut3(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut4(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut5(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut6(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox_IntPtr_ByRefMut7(
        obj: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_debug() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_debug", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_debug(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_debug", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNIHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
