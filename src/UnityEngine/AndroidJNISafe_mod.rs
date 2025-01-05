#[cfg(feature = "UnityEngine+AndroidJNISafe")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNISafe {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJNISafe => "UnityEngine"
    ."AndroidJNISafe"
);
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNISafe {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNISafe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl crate::UnityEngine::AndroidJNISafe {
    pub fn CallBooleanMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallBooleanMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallCharMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallCharMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallDoubleMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallDoubleMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallFloatMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallFloatMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallIntMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallIntMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallLongMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallLongMethod", (obj, methodID, args))?;
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
    pub fn CallSByteMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallSByteMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallShortMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallShortMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticBooleanMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticBooleanMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticCharMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticCharMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticDoubleMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticDoubleMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticFloatMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticFloatMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticIntMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticIntMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticLongMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticLongMethod", (clazz, methodID, args))?;
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
    pub fn CallStaticSByteMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticSByteMethod", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallStaticShortMethod(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallStaticShortMethod", (clazz, methodID, args))?;
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
    pub fn CallStringMethod(
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
    pub fn CallVoidMethod(
        obj: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallVoidMethod", (obj, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteLocalRef(
        localref: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteLocalRef", (localref))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteWeakGlobalRef(
        globalref: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteWeakGlobalRef", (globalref))?;
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
    pub fn GetBooleanField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBooleanField", (obj, fieldID))?;
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
    pub fn GetFloatField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFloatField", (obj, fieldID))?;
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
    pub fn GetLongField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLongField", (obj, fieldID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodID(
        obj: crate::System::IntPtr,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodID", (obj, name, sig))?;
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
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectClass", (ptr))?;
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
    pub fn GetSByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSByteField", (obj, fieldID))?;
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
    pub fn NewObject(
        clazz: crate::System::IntPtr,
        methodID: crate::System::IntPtr,
        args: crate::System::Span_1<crate::UnityEngine::jvalue>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewObject", (clazz, methodID, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewString(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewString", (chars))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueDeleteGlobalRef(
        globalref: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueDeleteGlobalRef", (globalref))?;
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
    pub fn SetCharField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCharField", (obj, fieldID, val))?;
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
    pub fn SetFloatField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetFloatField", (obj, fieldID, val))?;
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
    pub fn SetLongField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLongField", (obj, fieldID, val))?;
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
    pub fn SetSByteField(
        obj: crate::System::IntPtr,
        fieldID: crate::System::IntPtr,
        val: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSByteField", (obj, fieldID, val))?;
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
    pub fn ToCharArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToCharArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDoubleArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDoubleArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFloatArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFloatArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIntArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToIntArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLongArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLongArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToObjectArray(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
        _cordl_type: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToObjectArray", (array, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByteArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByteArray", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToShortArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToShortArray", (array))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNISafe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
