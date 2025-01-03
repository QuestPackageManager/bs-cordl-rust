#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBlock")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputStateBlock {
    pub _format_k__BackingField: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub m_ByteOffset: u32,
    pub _bitOffset_k__BackingField: u32,
    pub _sizeInBits_k__BackingField: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBlock")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputStateBlock =>
    "UnityEngine.InputSystem.LowLevel"."InputStateBlock"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBlock")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputStateBlock {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBlock")]
impl crate::UnityEngine::InputSystem::LowLevel::InputStateBlock {
    pub const AutomaticOffset: u32 = 1089470718u32;
    pub const InvalidOffset: u32 = 4026597119u32;
    pub const kFormatBit: i32 = 1112101920i32;
    pub const kFormatByte: i32 = 1113150533i32;
    pub const kFormatDouble: i32 = 1145195552i32;
    pub const kFormatFloat: i32 = 1179407392i32;
    pub const kFormatInt: i32 = 1229870112i32;
    pub const kFormatInvalid: i32 = 0i32;
    pub const kFormatLong: i32 = 1280198432i32;
    pub const kFormatPose: i32 = 1349481317i32;
    pub const kFormatQuaternion: i32 = 1364541780i32;
    pub const kFormatSBit: i32 = 1396853076i32;
    pub const kFormatSByte: i32 = 1396857172i32;
    pub const kFormatShort: i32 = 1397248596i32;
    pub const kFormatUInt: i32 = 1430867540i32;
    pub const kFormatULong: i32 = 1431064135i32;
    pub const kFormatUShort: i32 = 1431521364i32;
    pub const kFormatVector2: i32 = 1447379762i32;
    pub const kFormatVector3: i32 = 1447379763i32;
    pub fn CopyToFrom(
        &mut self,
        toStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fromStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyToFrom",
            (toStatePtr, fromStatePtr),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FloatToPrimitiveValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FloatToPrimitiveValue",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimitiveFormatFromType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimitiveFormatFromType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSizeOfPrimitiveFormatInBits(
        _cordl_type: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSizeOfPrimitiveFormatInBits", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDouble(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadDouble",
            (statePtr),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFloat(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadFloat",
            (statePtr),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadInt(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadInt",
            (statePtr),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Write",
            (statePtr, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDouble(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteDouble",
            (statePtr, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFloat(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteFloat",
            (statePtr, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteInt(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteInt",
            (statePtr, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignedSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_alignedSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bitOffset(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bitOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_byteOffset(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_byteOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectiveBitOffset(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_effectiveBitOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectiveByteOffset(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_effectiveByteOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_format",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeInBits(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sizeInBits",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bitOffset(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bitOffset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_byteOffset(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_byteOffset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_format(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_format",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sizeInBits(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sizeInBits",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
