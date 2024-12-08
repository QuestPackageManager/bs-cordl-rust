#[cfg(feature = "ENet+Packet")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Packet {
    pub nativePacket: crate::System::IntPtr,
}
#[cfg(feature = "ENet+Packet")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::Packet => "ENet"."Packet"
);
#[cfg(feature = "ENet+Packet")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::Packet {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+Packet")]
impl crate::ENet::Packet {
    pub fn CopyTo(
        &mut self,
        destination: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        startPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyTo",
            (destination, startPos),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Create_Il2CppArray0(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Create",
            (data),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Create_Il2CppArray_PacketFlags2(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Create",
            (data, flags),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Create_Il2CppArray_i32_1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Create",
            (data, length),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Create_Il2CppArray_i32_PacketFlags3(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        length: i32,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Create",
            (data, length, flags),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Create_Il2CppArray_i32_i32_PacketFlags5(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Create",
            (data, offset, length, flags),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Create_IntPtr_i32_PacketFlags4(
        &mut self,
        data: crate::System::IntPtr,
        length: i32,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Create",
            (data, length, flags),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Create_IntPtr_i32_i32_PacketFlags6(
        &mut self,
        data: crate::System::IntPtr,
        offset: i32,
        length: i32,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Create",
            (data, offset, length, flags),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFreeCallback_IntPtr0(
        &mut self,
        callback: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFreeCallback",
            (callback),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFreeCallback_PacketFreeCallback1(
        &mut self,
        callback: *mut crate::ENet::PacketFreeCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFreeCallback",
            (callback),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ThrowIfNotCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ThrowIfNotCreated",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (packet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Data",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_HasReferences(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasReferences",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsSet",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_NativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NativeData",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_UserData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_UserData",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_NativeData(
        &mut self,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_NativeData",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_UserData(
        &mut self,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_UserData",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
