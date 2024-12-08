#[cfg(feature = "Tweening+FrameParity")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameParity {
    AllFrames = 0i32,
    EvenFrames = 2i32,
    OddFrames = 1i32,
}
#[cfg(feature = "Tweening+FrameParity")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Tweening::FrameParity => "Tweening"
    ."FrameParity"
);
