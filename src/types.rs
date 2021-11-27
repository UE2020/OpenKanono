use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub type Identifer = usize;
pub type Connection = tokio::sync::mpsc::UnboundedSender<warp::ws::Message>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    TrueWhite = 0,
    TrueBlack,
    White,
    Black,
    Grid,
    DarkGrid,
    Blue,
    Grey,
    BorderGrey,
    Red,
    Orange,
    Green,
    Yellow,
    TriangleRed,
    PentagonBlue,
    LightGreen,
    TrueRed,
    DarkGrey,
    CohortBlue,
    ChargingCyan,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        unsafe { std::mem::transmute::<u8, Color>(rng.gen_range(0..=19)) }
    }
}
