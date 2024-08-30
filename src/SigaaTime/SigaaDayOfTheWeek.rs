enum SigaaDayOfTheWeek {
    2,
    3,
    4,
    5,
    6,
}

impl fmt::Display for SigaaDayOfTheWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            2 => write!(f, "2")
            3 => write!(f, "3")
            4 => write!(f, "4")
            5 => write!(f, "5")
            6 => write!(f, "6")
    }
}

impl SigaaDayOfTheWeek {
    fn to_corresponding_string(&self) -> string {
        return match self {
            2 => "Segunda-feira"
            3 => "Terça-feira"
            4 => "Quarta-feira"
            5 => "Quinta-feira"
            6 => "Sexta-feira"
        }
    }
}
