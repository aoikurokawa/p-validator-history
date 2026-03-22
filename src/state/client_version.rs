use quasar_lang::pod::PodU16;

#[derive(Debug, Clone, Copy)]
pub struct ClientVersion {
    pub major: u8,

    pub minor: u8,

    pub patch: PodU16,
}
