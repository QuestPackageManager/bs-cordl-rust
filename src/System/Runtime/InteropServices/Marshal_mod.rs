#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
#[repr(C)]
#[derive(Debug)]
pub struct Marshal {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::Marshal =>
    "System.Runtime.InteropServices"."Marshal"
);
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::Marshal {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::Marshal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl crate::System::Runtime::InteropServices::Marshal {
    #[cfg(
        feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer"
    )]
    pub type MarshalerInstanceKeyComparer = crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer;
    #[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
    pub type SecureStringAllocator = crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator;
    pub fn AllocCoTaskMem(
        cb: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocCoTaskMem", (cb))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocHGlobal_IntPtr0(
        cb: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocHGlobal", (cb))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocHGlobal_i32_1(
        cb: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocHGlobal", (cb))?;
        Ok(__cordl_ret.into())
    }
    pub fn BufferToBSTR(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        slen: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BufferToBSTR", (ptr, slen))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAnsi(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearAnsi", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearBSTR(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearBSTR", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearUnicode(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearUnicode", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_Gc_i32_IntPtr0(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        destination: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (source, startIndex, destination, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_IntPtr_Gc_i32_1(
        source: crate::System::IntPtr,
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (source, destination, startIndex, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_IntPtr_Gc_i32_2(
        source: crate::System::IntPtr,
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (source, destination, startIndex, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_IntPtr_Gc_i32_3(
        source: crate::System::IntPtr,
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (source, destination, startIndex, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeBSTR(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeBSTR", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeCoTaskMem(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeCoTaskMem", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeHGlobal(
        hglobal: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeHGlobal", (hglobal))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomMarshalerInstance(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        cookie: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::ICustomMarshaler,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::ICustomMarshaler,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCustomMarshalerInstance", (_cordl_type, cookie))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDelegateForFunctionPointerInternal(
        ptr: crate::System::IntPtr,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDelegateForFunctionPointerInternal", (ptr, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDelegateForFunctionPointer_Gc0(
        ptr: crate::System::IntPtr,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDelegateForFunctionPointer", (ptr, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDelegateForFunctionPointer_IntPtr1<TDelegate>(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<TDelegate>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDelegate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDelegateForFunctionPointer", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFunctionPointerForDelegateInternal(
        d: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFunctionPointerForDelegateInternal", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFunctionPointerForDelegate_Gc0(
        d: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFunctionPointerForDelegate", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFunctionPointerForDelegate_TDelegate1<TDelegate>(
        d: TDelegate,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFunctionPointerForDelegate", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHRForException(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHRForException", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastWin32Error() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastWin32Error", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsComObject(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsComObject", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn OffsetOf(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        fieldName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OffsetOf", (t, fieldName))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStringAnsi_IntPtr0(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStringAnsi", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStringAnsi_i32_1(
        ptr: crate::System::IntPtr,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStringAnsi", (ptr, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStringUni_IntPtr0(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStringUni", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStringUni_i32_1(
        ptr: crate::System::IntPtr,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStringUni", (ptr, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStructure_Gc0(
        ptr: crate::System::IntPtr,
        structure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStructure", (ptr, structure))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStructure_Gc1(
        ptr: crate::System::IntPtr,
        structureType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStructure", (ptr, structureType))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStructure_IntPtr3<T>(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStructure", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToStructure_T2<T>(
        ptr: crate::System::IntPtr,
        structure: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToStructure", (ptr, structure))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadByte(
        ptr: crate::System::IntPtr,
        ofs: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadByte", (ptr, ofs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadInt16(
        ptr: crate::System::IntPtr,
        ofs: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadInt16", (ptr, ofs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadInt32_IntPtr0(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadInt32", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadInt32_i32_1(
        ptr: crate::System::IntPtr,
        ofs: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadInt32", (ptr, ofs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecureStringGlobalAllocator(
        len: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecureStringGlobalAllocator", (len))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecureStringToBSTR(
        s: quest_hook::libil2cpp::Gc<crate::System::Security::SecureString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecureStringToBSTR", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecureStringToGlobalAllocUnicode(
        s: quest_hook::libil2cpp::Gc<crate::System::Security::SecureString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecureStringToGlobalAllocUnicode", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecureStringToUnicode(
        s: quest_hook::libil2cpp::Gc<crate::System::Security::SecureString>,
        allocator: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecureStringToUnicode", (s, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn SizeOf_Gc0(
        structure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SizeOf", (structure))?;
        Ok(__cordl_ret.into())
    }
    pub fn SizeOf_Gc1(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SizeOf", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn SizeOf_T2<T>(structure: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SizeOf", (structure))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToHGlobalAnsi_Gc1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToHGlobalAnsi", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToHGlobalAnsi_i32_0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToHGlobalAnsi", (s, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn StructureToPtr_Gc0(
        structure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ptr: crate::System::IntPtr,
        fDeleteOld: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StructureToPtr", (structure, ptr, fDeleteOld))?;
        Ok(__cordl_ret.into())
    }
    pub fn StructureToPtr_T1<T>(
        structure: T,
        ptr: crate::System::IntPtr,
        fDeleteOld: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StructureToPtr", (structure, ptr, fDeleteOld))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeAddrOfPinnedArrayElement_Gc_i32_0(
        arr: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeAddrOfPinnedArrayElement", (arr, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeAddrOfPinnedArrayElement_Gc_i32_1<T>(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeAddrOfPinnedArrayElement", (arr, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteByte(
        ptr: crate::System::IntPtr,
        ofs: i32,
        val: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteByte", (ptr, ofs, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteInt16(
        ptr: crate::System::IntPtr,
        ofs: i32,
        val: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteInt16", (ptr, ofs, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn ZeroFreeBSTR(
        s: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroFreeBSTR", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ZeroFreeGlobalAllocAnsi(
        s: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroFreeGlobalAllocAnsi", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ZeroFreeGlobalAllocUnicode(
        s: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroFreeGlobalAllocUnicode", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn copy_from_unmanaged(
        source: crate::System::IntPtr,
        startIndex: i32,
        destination: quest_hook::libil2cpp::Gc<crate::System::Array>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("copy_from_unmanaged", (source, startIndex, destination, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn copy_from_unmanaged_fixed(
        source: crate::System::IntPtr,
        startIndex: i32,
        destination: quest_hook::libil2cpp::Gc<crate::System::Array>,
        length: i32,
        fixed_destination_element: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "copy_from_unmanaged_fixed",
                (source, startIndex, destination, length, fixed_destination_element),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn copy_to_unmanaged(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        destination: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("copy_to_unmanaged", (source, startIndex, destination, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn copy_to_unmanaged_fixed(
        source: quest_hook::libil2cpp::Gc<crate::System::Array>,
        startIndex: i32,
        destination: crate::System::IntPtr,
        length: i32,
        fixed_source_element: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "copy_to_unmanaged_fixed",
                (source, startIndex, destination, length, fixed_source_element),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn skip_fixed(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("skip_fixed", (array, startIndex))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::Marshal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct Marshal_MarshalerInstanceKeyComparer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer =>
    "System.Runtime.InteropServices"."Marshal/MarshalerInstanceKeyComparer"
);
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    pub fn Equals(
        &mut self,
        lhs: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        rhs: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        key: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
> for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+MarshalerInstanceKeyComparer")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
> for crate::System::Runtime::InteropServices::Marshal_MarshalerInstanceKeyComparer {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct Marshal_SecureStringAllocator {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::Marshal_SecureStringAllocator =>
    "System.Runtime.InteropServices"."Marshal/SecureStringAllocator"
);
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    pub fn Invoke(
        &mut self,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object.invoke("Invoke", (len))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+Marshal+SecureStringAllocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::Marshal_SecureStringAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
