#[cfg(feature = "UnityEngine+AndroidJNI")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNI {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJNI => "UnityEngine"
    ."AndroidJNI"
);
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNI {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl crate::UnityEngine::AndroidJNI {
    pub fn AllocObject(
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocObject", (clazz))?;
        Ok(__cordl_ret.into())
    }
    pub fn AttachCurrentThread() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AttachCurrentThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CallBooleanMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallBooleanMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallBooleanMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallBooleanMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallBooleanMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallBooleanMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallByteMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallByteMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallCharMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallCharMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallCharMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallCharMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallCharMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallCharMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallDoubleMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallDoubleMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallDoubleMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallDoubleMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallDoubleMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallDoubleMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallFloatMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallFloatMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallFloatMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallFloatMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallFloatMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallFloatMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallIntMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallIntMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallIntMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallIntMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallIntMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallIntMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallLongMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallLongMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallLongMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallLongMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallLongMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallLongMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallObjectMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallObjectMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallObjectMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallObjectMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallObjectMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallObjectMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallSByteMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallSByteMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallSByteMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallSByteMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallSByteMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallSByteMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallShortMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallShortMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallShortMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallShortMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallShortMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallShortMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticBooleanMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticBooleanMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticBooleanMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticBooleanMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticBooleanMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticBooleanMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticByteMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticByteMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticCharMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticCharMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticCharMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticCharMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticCharMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticCharMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticDoubleMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticDoubleMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticDoubleMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticDoubleMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticDoubleMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticDoubleMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticFloatMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticFloatMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticFloatMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticFloatMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticFloatMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticFloatMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticIntMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticIntMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticIntMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticIntMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticIntMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticIntMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticLongMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticLongMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticLongMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticLongMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticLongMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticLongMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticObjectMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticObjectMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticObjectMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticObjectMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticObjectMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticObjectMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticSByteMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticSByteMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticSByteMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticSByteMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticSByteMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticSByteMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticShortMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticShortMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticShortMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticShortMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticShortMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticShortMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticStringMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticStringMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticStringMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticStringMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticStringMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticStringMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticVoidMethodUnsafe(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticVoidMethodUnsafe", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticVoidMethod_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticVoidMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticVoidMethod_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticVoidMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStringMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStringMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStringMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStringMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStringMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStringMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallVoidMethodUnsafe(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallVoidMethodUnsafe", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallVoidMethod_Il2CppArray0(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallVoidMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallVoidMethod_Span_1_1(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallVoidMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteGlobalRef", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteLocalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteLocalRef", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteWeakGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteWeakGlobalRef", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn DetachCurrentThread() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetachCurrentThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLocalCapacity(capacity: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureLocalCapacity", (capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExceptionClear() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExceptionClear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExceptionDescribe() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExceptionDescribe", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExceptionOccurred() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExceptionOccurred", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FatalError(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FatalError", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindClass(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindClass", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBooleanArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBooleanArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromByteArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromByteArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCharArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCharArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromDoubleArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f64>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromDoubleArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFloatArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFloatArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromIntArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromIntArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromLongArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i64>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromLongArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromObjectArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromObjectArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromReflectedField(
        refField: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromReflectedField", (refField))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromReflectedMethod(
        refMethod: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromReflectedMethod", (refMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSByteArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSByteArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromShortArray(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i16>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromShortArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayLength(
        array: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArrayLength", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBooleanArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBooleanArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBooleanField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBooleanField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByteArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByteField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCharArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCharField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectBuffer<T>(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectBuffer", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectBufferAddress(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectBufferAddress", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectBufferCapacity(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectBufferCapacity", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectByteBuffer(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectByteBuffer", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectSByteBuffer(
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<i8>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<i8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDirectSByteBuffer", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDoubleArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDoubleArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDoubleField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDoubleField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldID", (clazz, name, sig))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFloatArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFloatArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFloatField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFloatField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIntArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIntArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIntField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIntField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetJavaVM() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetJavaVM", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLongArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLongArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLongField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLongField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (clazz, name, sig))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectClass(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectClass", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQueueGlobalRefsCount() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQueueGlobalRefsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSByteArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSByteField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShortArrayElement(
        array: crate::System::IntPtr,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShortArrayElement", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShortField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetShortField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticBooleanField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticBooleanField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticByteField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticCharField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticCharField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticDoubleField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticDoubleField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticFieldID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticFieldID", (clazz, name, sig))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticFloatField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticFloatField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticIntField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticIntField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticLongField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticLongField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticMethodID(
        clazz: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticMethodID", (clazz, name, sig))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticObjectField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticObjectField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticSByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticSByteField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticShortField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticShortField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStaticStringField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStaticStringField", (clazz, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringChars(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringChars", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringLength(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringLength", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringUTFChars(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringUTFChars", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringUTFLength(
        str: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringUTFLength", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSuperclass(
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSuperclass", (clazz))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVersion() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAssignableFrom(
        clazz1: crate::System::IntPtr,
        clazz2: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAssignableFrom", (clazz1, clazz2))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInstanceOf(
        obj: crate::System::IntPtr,
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInstanceOf", (obj, clazz))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSameObject(
        obj1: crate::System::IntPtr,
        obj2: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSameObject", (obj1, obj2))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewBooleanArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewBooleanArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewByteArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewByteArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewCharArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewCharArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBufferFromNativeArray<T>(
        buffer: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewDirectByteBufferFromNativeArray", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBuffer_Il2CppObject_i64_0(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        capacity: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewDirectByteBuffer", (buffer, capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBuffer_NativeArray_1_1(
        buffer: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewDirectByteBuffer", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewDirectByteBuffer_NativeArray_1_2(
        buffer: crate::Unity::Collections::NativeArray_1<i8>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewDirectByteBuffer", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewDoubleArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewDoubleArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewFloatArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewFloatArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewGlobalRef", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewIntArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewIntArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewLocalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewLocalRef", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewLongArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewLongArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewObjectA(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewObjectA", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewObjectArray(
        _cordl_size: i32,
        clazz: crate::System::IntPtr,
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewObjectArray", (_cordl_size, clazz, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewObject_Il2CppArray0(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::jvalue>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewObject", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewObject_Span_1_1(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewObject", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewSByteArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewSByteArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewShortArray(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewShortArray", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewStringFromStr(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewStringFromStr", (chars))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewStringUTF(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewStringUTF", (bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewString_Il2CppArray1(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewString", (chars))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewString_Il2CppString0(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewString", (chars))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewWeakGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewWeakGlobalRef", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopLocalFrame(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopLocalFrame", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn PushLocalFrame(capacity: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PushLocalFrame", (capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueDeleteGlobalRef(
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueDeleteGlobalRef", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNatives(
        clazz: crate::System::IntPtr,
        methods: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::JNINativeMethod>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterNatives", (clazz, methods))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNativesAllocate(
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterNativesAllocate", (length))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNativesAndFree(
        clazz: crate::System::IntPtr,
        natives: crate::System::IntPtr,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterNativesAndFree", (clazz, natives, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNativesSet(
        natives: crate::System::IntPtr,
        idx: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fnPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterNativesSet", (natives, idx, name, signature, fnPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBooleanArrayElement__cordl_bool1(
        array: crate::System::IntPtr,
        index: i32,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetBooleanArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBooleanArrayElement_u8_0(
        array: crate::System::IntPtr,
        index: i32,
        val: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetBooleanArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBooleanField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetBooleanField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetByteArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetByteField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCharArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCharArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCharField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCharField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDoubleArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDoubleArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDoubleField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDoubleField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFloatArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetFloatArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFloatField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetFloatField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIntArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetIntArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIntField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetIntField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLongArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLongArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLongField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLongField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        obj: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetObjectArrayElement", (array, index, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetObjectField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSByteArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSByteArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSByteField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetShortArrayElement(
        array: crate::System::IntPtr,
        index: i32,
        val: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetShortArrayElement", (array, index, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetShortField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetShortField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticBooleanField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticBooleanField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticByteField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticCharField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticCharField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticDoubleField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticDoubleField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticFloatField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticFloatField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticIntField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticIntField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticLongField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticLongField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticObjectField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticObjectField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticSByteField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticSByteField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticShortField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticShortField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStaticStringField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStaticStringField", (clazz, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStringField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStringField", (obj, fieldID, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Throw(obj: crate::System::IntPtr) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Throw", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowNew(
        clazz: crate::System::IntPtr,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowNew", (clazz, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBooleanArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBooleanArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByteArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByteArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToCharArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToCharArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToCharArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToCharArray", (array, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDoubleArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDoubleArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDoubleArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDoubleArray", (array, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFloatArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFloatArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFloatArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFloatArray", (array, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIntArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToIntArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIntArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToIntArray", (array, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLongArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLongArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLongArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLongArray", (array, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToObjectArray_Il2CppArray2(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToObjectArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToObjectArray_Il2CppArray_IntPtr1(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
        arrayClass: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToObjectArray", (array, arrayClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToObjectArray_Il2CppObject_i32_IntPtr0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
        arrayClass: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToObjectArray", (array, length, arrayClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToReflectedField(
        clazz: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToReflectedField", (clazz, fieldID, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToReflectedMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        isStatic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToReflectedMethod", (clazz, methodID, isStatic))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByteArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByteArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByteArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByteArray", (array, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToShortArray_Il2CppArray0(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToShortArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToShortArray_Il2CppObject_i32_1(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToShortArray", (array, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterNatives(
        clazz: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterNatives", (clazz))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
