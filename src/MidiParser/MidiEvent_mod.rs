#[cfg(feature = "MidiParser+MidiEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub AbsoluteTicksTime: i32,
    pub Type: u8,
    pub Arg1: i32,
    pub Arg2: i32,
    pub Arg3: i32,
}
#[cfg(feature = "MidiParser+MidiEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MidiParser::MidiEvent => "MidiParser"
    ."MidiEvent"
);
#[cfg(feature = "MidiParser+MidiEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::MidiParser::MidiEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MidiParser+MidiEvent")]
impl crate::MidiParser::MidiEvent {
    pub fn get_Channel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Channel",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_MetaEventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::MidiParser::MetaEventType> {
        let __cordl_ret: crate::MidiParser::MetaEventType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MetaEventType",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Velocity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Velocity",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Note(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Note",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
