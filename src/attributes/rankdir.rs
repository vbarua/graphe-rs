use std::fmt::Display;

pub enum RankDir {
    TopBottom,
    LeftRight,
    BottomTop,
    RightLeft,
}

impl Display for RankDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RankDir::TopBottom => "TB",
            RankDir::LeftRight => "LR",
            RankDir::BottomTop => "BT",
            RankDir::RightLeft => "RL",
        };
        f.write_str(s)
    }
}
