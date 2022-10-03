#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq, Debug)]
pub enum Market {
    /// 미국전체
    NASD,
    /// 나스닥
    NAS,
    /// 뉴욕
    NYSE,
    /// 아멕스
    AMEX,
    /// 홍콩
    SEHK,
    /// 중국상해
    SHAA,
    /// 중국심천
    SZAA,
    // 일본
    TKSE,
    /// 베트남 하노이   
    HASE,
    /// 베트남 호치민
    VNSE,
}
impl std::fmt::Display for Market {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Market::NASD => write!(f, "NASD"),
            Market::NYSE => write!(f, "NYSE"),
            Market::AMEX => write!(f, "AMEX"),
            Market::SEHK => write!(f, "SEHK"),
            Market::SHAA => write!(f, "SHAA"),
            Market::SZAA => write!(f, "SZAA"),
            Market::TKSE => write!(f, "TKSE"),
            Market::HASE => write!(f, "HASE"),
            Market::VNSE => write!(f, "VNSE"),
            Market::NAS => write!(f, "NAS"),
        }
    }
}
impl std::str::FromStr for Market {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Market::try_from(s)
    }
}
impl std::convert::TryFrom<String> for Market {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Market::try_from(s.as_str())
    }
}
impl std::convert::TryFrom<&str> for Market {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "NASD" => Ok(Market::NASD),
            "NYSE" => Ok(Market::NYSE),
            "AMEX" => Ok(Market::AMEX),
            "SEHK" => Ok(Market::SEHK),
            "SHAA" => Ok(Market::SHAA),
            "SZAA" => Ok(Market::SZAA),
            "TKSE" => Ok(Market::TKSE),
            "HASE" => Ok(Market::HASE),
            "VNSE" => Ok(Market::VNSE),
            "NAS" => Ok(Market::NAS),
            _ => Err(()),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
pub enum Exchange {
    /// 홍콩
    HKS,
    /// 뉴욕
    NYS,
    /// 나스닥
    NAS,
    /// 아멕스
    AMS,
    /// 도쿄
    TSE,
    /// 상해
    SHS,
    /// 심천
    SZS,
    /// 상해지수
    SHI,
    /// 심천지수
    SZI,
    /// 호치민
    HSX,
    /// 하노이
    HNX,
}
impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Exchange::HKS => write!(f, "HKS"),
            Exchange::NYS => write!(f, "NYS"),
            Exchange::NAS => write!(f, "NAS"),
            Exchange::AMS => write!(f, "AMS"),
            Exchange::TSE => write!(f, "TSE"),
            Exchange::SHS => write!(f, "SHS"),
            Exchange::SZS => write!(f, "SZS"),
            Exchange::SHI => write!(f, "SHI"),
            Exchange::SZI => write!(f, "SZI"),
            Exchange::HSX => write!(f, "HSX"),
            Exchange::HNX => write!(f, "HNX"),
        }
    }
}
impl std::str::FromStr for Exchange {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Exchange::try_from(s)
    }
}
impl std::convert::TryFrom<String> for Exchange {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Exchange::try_from(s.as_str())
    }
}
impl std::convert::TryFrom<&str> for Exchange {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "HKS" => Ok(Exchange::HKS),
            "NYS" => Ok(Exchange::NYS),
            "NAS" => Ok(Exchange::NAS),
            "AMS" => Ok(Exchange::AMS),
            "TSE" => Ok(Exchange::TSE),
            "SHS" => Ok(Exchange::SHS),
            "SZS" => Ok(Exchange::SZS),
            "SHI" => Ok(Exchange::SHI),
            "SZI" => Ok(Exchange::SZI),
            "HSX" => Ok(Exchange::HSX),
            "HNX" => Ok(Exchange::HNX),
            _ => Err(()),
        }
    }
}

impl Market {
    pub fn get_currency(&self) -> &str {
        match *self {
            Market::NAS => "USD",
            Market::NASD => "USD",
            Market::NYSE => "USD",
            Market::AMEX => "USD",
            Market::SEHK => "HKD",
            Market::SHAA => "CNY",
            Market::SZAA => "CNY",
            Market::TKSE => "JPY",
            Market::HASE => "VND",
            Market::VNSE => "VND",
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
pub enum Currency {
    /// 미국달러
    USD,
    /// 홍콩달러
    HKD,
    /// 중국위안화
    CNY,
    /// 일본엔화  
    JPY,
    /// 베트남동
    VND,
}
impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::CNY => write!(f, "CNY"),
            Currency::JPY => write!(f, "JPY"),
            Currency::HKD => write!(f, "HKD"),
            Currency::VND => write!(f, "VND"),
        }
    }
}
impl std::convert::TryFrom<&str> for Currency {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "USD" => Ok(Currency::USD),
            "CNY" => Ok(Currency::CNY),
            "JPY" => Ok(Currency::JPY),
            "HKD" => Ok(Currency::HKD),
            "VND" => Ok(Currency::VND),
            _ => Err(()),
        }
    }
}
impl std::convert::TryFrom<String> for Currency {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Currency::try_from(value.as_str())
    }
}
impl std::str::FromStr for Currency {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Currency::try_from(value)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Interval {
    Day,
    Week,
    Month,
}

impl std::str::FromStr for Interval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Interval::try_from(s)
    }
}
impl std::convert::TryFrom<String> for Interval {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Interval::try_from(s.as_str())
    }
}
impl std::convert::TryFrom<&str> for Interval {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "0" => Ok(Interval::Day),
            "1" => Ok(Interval::Week),
            "2" => Ok(Interval::Month),
            "D" => Ok(Interval::Day),
            "W" => Ok(Interval::Week),
            "M" => Ok(Interval::Month),
            _ => Err(()),
        }
    }
}
impl Interval {
    pub fn to_num_str(&self) -> &str {
        match *self {
            Interval::Day => "0",
            Interval::Week => "1",
            Interval::Month => "2",
        }
    }
    pub fn to_char(&self) -> &str {
        match *self {
            Interval::Day => "D",
            Interval::Week => "W",
            Interval::Month => "M",
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_to() {
        assert_eq!(Interval::Day.to_char(), "D");
        assert_eq!(Interval::Week.to_char(), "W");
        assert_eq!(Interval::Month.to_char(), "M");
        assert_eq!(Interval::Day.to_num_str(), "0");
        assert_eq!(Interval::Week.to_num_str(), "1");
        assert_eq!(Interval::Month.to_num_str(), "2");
    }
    #[test]
    fn market_currency() {
        assert_eq!(Market::NAS.get_currency(), "USD");
        assert_eq!(Market::NASD.get_currency(), "USD");
        assert_eq!(Market::NYSE.get_currency(), "USD");
        assert_eq!(Market::AMEX.get_currency(), "USD");
        assert_eq!(Market::SEHK.get_currency(), "HKD");
        assert_eq!(Market::SHAA.get_currency(), "CNY");
        assert_eq!(Market::SZAA.get_currency(), "CNY");
        assert_eq!(Market::TKSE.get_currency(), "JPY");
        assert_eq!(Market::HASE.get_currency(), "VND");
        assert_eq!(Market::VNSE.get_currency(), "VND");
    }
    #[test]
    fn currency_convert() {
        assert_eq!(Currency::USD.to_string(), "USD".to_owned());
        assert_eq!(Currency::CNY.to_string(), "CNY".to_owned());
        assert_eq!(Currency::JPY.to_string(), "JPY".to_owned());
        assert_eq!(Currency::HKD.to_string(), "HKD".to_owned());
        assert_eq!(Currency::VND.to_string(), "VND".to_owned());
        assert_eq!(Ok(Currency::USD), "USD".parse::<Currency>());
        assert_eq!(Ok(Currency::CNY), "CNY".parse::<Currency>());
        assert_eq!(Ok(Currency::JPY), "JPY".parse::<Currency>());
        assert_eq!(Ok(Currency::HKD), "HKD".parse::<Currency>());
        assert_eq!(Ok(Currency::VND), "VND".parse::<Currency>());
        assert_eq!(Ok(Currency::USD), Currency::try_from("USD"));
        assert_eq!(Ok(Currency::CNY), Currency::try_from("CNY"));
        assert_eq!(Ok(Currency::JPY), Currency::try_from("JPY"));
        assert_eq!(Ok(Currency::HKD), Currency::try_from("HKD"));
        assert_eq!(Ok(Currency::VND), Currency::try_from("VND"));
        assert_eq!(Ok(Currency::USD), Currency::try_from("USD".to_owned()));
        assert_eq!(Ok(Currency::CNY), Currency::try_from("CNY".to_owned()));
        assert_eq!(Ok(Currency::JPY), Currency::try_from("JPY".to_owned()));
        assert_eq!(Ok(Currency::HKD), Currency::try_from("HKD".to_owned()));
        assert_eq!(Ok(Currency::VND), Currency::try_from("VND".to_owned()));
        let result: Result<Currency, ()> = "USD".try_into();
        assert_eq!(Ok(Currency::USD), result);
        let result: Result<Currency, ()> = "CNY".try_into();
        assert_eq!(Ok(Currency::CNY), result);
        let result: Result<Currency, ()> = "JPY".try_into();
        assert_eq!(Ok(Currency::JPY), result);
        let result: Result<Currency, ()> = "HKD".try_into();
        assert_eq!(Ok(Currency::HKD), result);
        let result: Result<Currency, ()> = "VND".try_into();
        assert_eq!(Ok(Currency::VND), result);
        assert_eq!(Err(()), "V".parse::<Currency>());
        assert_eq!(Err(()), Currency::try_from("V".to_owned()));
        assert_eq!(Err(()), Currency::try_from("V"));
        let result: Result<Currency, ()> = "V".try_into();
        assert_eq!(Err(()), result);
    }
    #[test]
    fn exchange_convert() {
        assert_eq!(Exchange::HKS.to_string(), "HKS".to_owned());
        assert_eq!(Exchange::NYS.to_string(), "NYS".to_owned());
        assert_eq!(Exchange::NAS.to_string(), "NAS".to_owned());
        assert_eq!(Exchange::AMS.to_string(), "AMS".to_owned());
        assert_eq!(Exchange::TSE.to_string(), "TSE".to_owned());
        assert_eq!(Exchange::SHS.to_string(), "SHS".to_owned());
        assert_eq!(Exchange::SZS.to_string(), "SZS".to_owned());
        assert_eq!(Exchange::SHI.to_string(), "SHI".to_owned());
        assert_eq!(Exchange::SZI.to_string(), "SZI".to_owned());
        assert_eq!(Exchange::HSX.to_string(), "HSX".to_owned());
        assert_eq!(Exchange::HNX.to_string(), "HNX".to_owned());
        assert_eq!(Ok(Exchange::HKS), "HKS".parse::<Exchange>());
        assert_eq!(Ok(Exchange::NYS), "NYS".parse::<Exchange>());
        assert_eq!(Ok(Exchange::NAS), "NAS".parse::<Exchange>());
        assert_eq!(Ok(Exchange::AMS), "AMS".parse::<Exchange>());
        assert_eq!(Ok(Exchange::TSE), "TSE".parse::<Exchange>());
        assert_eq!(Ok(Exchange::SHS), "SHS".parse::<Exchange>());
        assert_eq!(Ok(Exchange::SZS), "SZS".parse::<Exchange>());
        assert_eq!(Ok(Exchange::SHI), "SHI".parse::<Exchange>());
        assert_eq!(Ok(Exchange::SZI), "SZI".parse::<Exchange>());
        assert_eq!(Ok(Exchange::HSX), "HSX".parse::<Exchange>());
        assert_eq!(Ok(Exchange::HNX), "HNX".parse::<Exchange>());
        assert_eq!(Ok(Exchange::HKS), Exchange::try_from("HKS"));
        assert_eq!(Ok(Exchange::NYS), Exchange::try_from("NYS"));
        assert_eq!(Ok(Exchange::NAS), Exchange::try_from("NAS"));
        assert_eq!(Ok(Exchange::AMS), Exchange::try_from("AMS"));
        assert_eq!(Ok(Exchange::TSE), Exchange::try_from("TSE"));
        assert_eq!(Ok(Exchange::SHS), Exchange::try_from("SHS"));
        assert_eq!(Ok(Exchange::SZS), Exchange::try_from("SZS"));
        assert_eq!(Ok(Exchange::SHI), Exchange::try_from("SHI"));
        assert_eq!(Ok(Exchange::SZI), Exchange::try_from("SZI"));
        assert_eq!(Ok(Exchange::HSX), Exchange::try_from("HSX"));
        assert_eq!(Ok(Exchange::HNX), Exchange::try_from("HNX"));
        assert_eq!(Ok(Exchange::HKS), Exchange::try_from("HKS".to_owned()));
        assert_eq!(Ok(Exchange::NYS), Exchange::try_from("NYS".to_owned()));
        assert_eq!(Ok(Exchange::NAS), Exchange::try_from("NAS".to_owned()));
        assert_eq!(Ok(Exchange::AMS), Exchange::try_from("AMS".to_owned()));
        assert_eq!(Ok(Exchange::TSE), Exchange::try_from("TSE".to_owned()));
        assert_eq!(Ok(Exchange::SHS), Exchange::try_from("SHS".to_owned()));
        assert_eq!(Ok(Exchange::SZS), Exchange::try_from("SZS".to_owned()));
        assert_eq!(Ok(Exchange::SHI), Exchange::try_from("SHI".to_owned()));
        assert_eq!(Ok(Exchange::SZI), Exchange::try_from("SZI".to_owned()));
        assert_eq!(Ok(Exchange::HSX), Exchange::try_from("HSX".to_owned()));
        assert_eq!(Ok(Exchange::HNX), Exchange::try_from("HNX".to_owned()));
        let result: Result<Exchange, ()> = "HKS".try_into();
        assert_eq!(Ok(Exchange::HKS), result);
        let result: Result<Exchange, ()> = "NYS".try_into();
        assert_eq!(Ok(Exchange::NYS), result);
        let result: Result<Exchange, ()> = "NAS".try_into();
        assert_eq!(Ok(Exchange::NAS), result);
        let result: Result<Exchange, ()> = "AMS".try_into();
        assert_eq!(Ok(Exchange::AMS), result);
        let result: Result<Exchange, ()> = "TSE".try_into();
        assert_eq!(Ok(Exchange::TSE), result);
        let result: Result<Exchange, ()> = "SHS".try_into();
        assert_eq!(Ok(Exchange::SHS), result);
        let result: Result<Exchange, ()> = "SZS".try_into();
        assert_eq!(Ok(Exchange::SZS), result);
        let result: Result<Exchange, ()> = "SHI".try_into();
        assert_eq!(Ok(Exchange::SHI), result);
        let result: Result<Exchange, ()> = "SZI".try_into();
        assert_eq!(Ok(Exchange::SZI), result);
        let result: Result<Exchange, ()> = "HSX".try_into();
        assert_eq!(Ok(Exchange::HSX), result);
        let result: Result<Exchange, ()> = "HNX".try_into();
        assert_eq!(Ok(Exchange::HNX), result);
        assert_eq!(Err(()), "V".parse::<Exchange>());
        assert_eq!(Err(()), Exchange::try_from("V".to_owned()));
        assert_eq!(Err(()), Exchange::try_from("V"));
        let result: Result<Exchange, ()> = "V".try_into();
        assert_eq!(Err(()), result);
    }
    #[test]
    fn market_convert() {
        assert_eq!(Market::NAS.to_string(), "NAS".to_owned());
        assert_eq!(Market::NASD.to_string(), "NASD".to_owned());
        assert_eq!(Market::NYSE.to_string(), "NYSE".to_owned());
        assert_eq!(Market::AMEX.to_string(), "AMEX".to_owned());
        assert_eq!(Market::SEHK.to_string(), "SEHK".to_owned());
        assert_eq!(Market::SHAA.to_string(), "SHAA".to_owned());
        assert_eq!(Market::SZAA.to_string(), "SZAA".to_owned());
        assert_eq!(Market::TKSE.to_string(), "TKSE".to_owned());
        assert_eq!(Market::HASE.to_string(), "HASE".to_owned());
        assert_eq!(Market::VNSE.to_string(), "VNSE".to_owned());
        assert_eq!(Ok(Market::NAS), "NAS".parse::<Market>());
        assert_eq!(Ok(Market::NASD), "NASD".parse::<Market>());
        assert_eq!(Ok(Market::NYSE), "NYSE".parse::<Market>());
        assert_eq!(Ok(Market::AMEX), "AMEX".parse::<Market>());
        assert_eq!(Ok(Market::SEHK), "SEHK".parse::<Market>());
        assert_eq!(Ok(Market::SHAA), "SHAA".parse::<Market>());
        assert_eq!(Ok(Market::SZAA), "SZAA".parse::<Market>());
        assert_eq!(Ok(Market::TKSE), "TKSE".parse::<Market>());
        assert_eq!(Ok(Market::HASE), "HASE".parse::<Market>());
        assert_eq!(Ok(Market::VNSE), "VNSE".parse::<Market>());
        assert_eq!(Ok(Market::NAS), Market::try_from("NAS".to_owned()));
        assert_eq!(Ok(Market::NASD), Market::try_from("NASD".to_owned()));
        assert_eq!(Ok(Market::NYSE), Market::try_from("NYSE".to_owned()));
        assert_eq!(Ok(Market::AMEX), Market::try_from("AMEX".to_owned()));
        assert_eq!(Ok(Market::SEHK), Market::try_from("SEHK".to_owned()));
        assert_eq!(Ok(Market::SHAA), Market::try_from("SHAA".to_owned()));
        assert_eq!(Ok(Market::SZAA), Market::try_from("SZAA".to_owned()));
        assert_eq!(Ok(Market::TKSE), Market::try_from("TKSE".to_owned()));
        assert_eq!(Ok(Market::HASE), Market::try_from("HASE".to_owned()));
        assert_eq!(Ok(Market::VNSE), Market::try_from("VNSE".to_owned()));
        assert_eq!(Ok(Market::NAS), Market::try_from("NAS"));
        assert_eq!(Ok(Market::NASD), Market::try_from("NASD"));
        assert_eq!(Ok(Market::NYSE), Market::try_from("NYSE"));
        assert_eq!(Ok(Market::AMEX), Market::try_from("AMEX"));
        assert_eq!(Ok(Market::SEHK), Market::try_from("SEHK"));
        assert_eq!(Ok(Market::SHAA), Market::try_from("SHAA"));
        assert_eq!(Ok(Market::SZAA), Market::try_from("SZAA"));
        assert_eq!(Ok(Market::TKSE), Market::try_from("TKSE"));
        assert_eq!(Ok(Market::HASE), Market::try_from("HASE"));
        assert_eq!(Ok(Market::VNSE), Market::try_from("VNSE"));
        let result: Result<Market, ()> = "NAS".try_into();
        assert_eq!(Ok(Market::NAS), result);
        let result: Result<Market, ()> = "NASD".try_into();
        assert_eq!(Ok(Market::NASD), result);
        let result: Result<Market, ()> = "NYSE".try_into();
        assert_eq!(Ok(Market::NYSE), result);
        let result: Result<Market, ()> = "AMEX".try_into();
        assert_eq!(Ok(Market::AMEX), result);
        let result: Result<Market, ()> = "SEHK".try_into();
        assert_eq!(Ok(Market::SEHK), result);
        let result: Result<Market, ()> = "SHAA".try_into();
        assert_eq!(Ok(Market::SHAA), result);
        let result: Result<Market, ()> = "SZAA".try_into();
        assert_eq!(Ok(Market::SZAA), result);
        let result: Result<Market, ()> = "TKSE".try_into();
        assert_eq!(Ok(Market::TKSE), result);
        let result: Result<Market, ()> = "HASE".try_into();
        assert_eq!(Ok(Market::HASE), result);
        let result: Result<Market, ()> = "VNSE".try_into();
        assert_eq!(Ok(Market::VNSE), result);

        assert_eq!(Err(()), "V".parse::<Market>());
        assert_eq!(Err(()), Market::try_from("V".to_owned()));
        assert_eq!(Err(()), Market::try_from("V"));
        let result: Result<Market, ()> = "V".try_into();
        assert_eq!(Err(()), result);
    }
    #[test]
    fn interval_convert() {
        let day = Ok(Interval::Day);
        let week = Ok(Interval::Week);
        let month = Ok(Interval::Month);
        let err = Err(());
        assert_eq!(Interval::try_from("0".to_owned()), day);
        assert_eq!(Interval::try_from("1".to_owned()), week);
        assert_eq!(Interval::try_from("2".to_owned()), month);
        assert_eq!(Interval::try_from("D".to_owned()), day);
        assert_eq!(Interval::try_from("W".to_owned()), week);
        assert_eq!(Interval::try_from("M".to_owned()), month);
        assert_eq!(Interval::try_from("3".to_owned()), err);
        assert_eq!(Interval::try_from("0"), day);
        assert_eq!(Interval::try_from("1"), week);
        assert_eq!(Interval::try_from("2"), month);
        assert_eq!(Interval::try_from("D"), day);
        assert_eq!(Interval::try_from("W"), week);
        assert_eq!(Interval::try_from("M"), month);
        assert_eq!(Interval::try_from("3"), err);
        let result: Result<Interval, ()> = "0".try_into();
        assert_eq!(result, day);
        let result: Result<Interval, ()> = "1".try_into();
        assert_eq!(result, week);
        let result: Result<Interval, ()> = "2".try_into();
        assert_eq!(result, month);
        let result: Result<Interval, ()> = "D".try_into();
        assert_eq!(result, day);
        let result: Result<Interval, ()> = "W".try_into();
        assert_eq!(result, week);
        let result: Result<Interval, ()> = "M".try_into();
        assert_eq!(result, month);
        let result: Result<Interval, ()> = "3".try_into();
        assert_eq!(result, err);
        assert_eq!("0".parse::<Interval>(), day);
        assert_eq!("1".parse::<Interval>(), week);
        assert_eq!("2".parse::<Interval>(), month);
        assert_eq!("D".parse::<Interval>(), day);
        assert_eq!("W".parse::<Interval>(), week);
        assert_eq!("M".parse::<Interval>(), month);
        assert_eq!("3".parse::<Interval>(), err);
    }
}
